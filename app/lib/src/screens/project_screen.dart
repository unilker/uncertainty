import 'dart:convert';
import 'dart:io';

import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';

import '../app_state.dart';
import '../engine.dart';
import '../files.dart';
import '../strings.dart';
import '../widgets/json_editor.dart';
import '../widgets/model_editor.dart';
import '../widgets/overview.dart';
import '../widgets/calibration_editor.dart';
import '../widgets/control_chart_editor.dart';
import '../widgets/error_function_editor.dart';
import '../widgets/consensus_editor.dart';
import '../widgets/sampling_editor.dart';
import '../widgets/conformity_editor.dart';
import '../widgets/precision_editor.dart';
import '../widgets/proficiency_editor.dart';
import '../widgets/results_view.dart';
import '../widgets/settings_editor.dart';
import '../widgets/topdown_editor.dart';
import '../learn/guide.dart';
import 'guide_screen.dart';

/// Editing, calculating and exporting one project.
class ProjectScreen extends StatefulWidget {
  const ProjectScreen({super.key, required this.project, this.filePath, this.fromLibrary = false, this.runOnOpen = false});
  final Map<String, dynamic> project;
  final String? filePath;
  final bool fromLibrary;

  /// Calculate as soon as the screen is shown.
  final bool runOnOpen;

  @override
  State<ProjectScreen> createState() => _ProjectScreenState();
}

class _ProjectScreenState extends State<ProjectScreen> with SingleTickerProviderStateMixin {
  late Map<String, dynamic> _project = widget.project;
  late String? _path = widget.fromLibrary ? null : widget.filePath;
  late final TabController _tabs = TabController(length: 5, vsync: this);
  bool _dirty = false;
  String? _invalid;
  bool _running = false;
  Map<String, dynamic>? _response;
  String? _engineError;
  bool _stale = false;

  /// Incremented when the whole project is replaced, to rebuild the editors.
  int _revision = 0;

  bool get _isTopDown => _project.containsKey('topdown');
  bool get _isProficiency => _project.containsKey('proficiency');
  bool get _isCalibration => _project.containsKey('calibration');
  bool get _isConformity => _project.containsKey('conformity') && !_project.containsKey('model');
  bool get _isPrecision => _project.containsKey('precision_experiment');
  bool get _isControlChart => _project.containsKey('control_chart');
  bool get _isSampling => _project.containsKey('sampling');
  bool get _isErrorFunction => _project.containsKey('error_function');
  bool get _isConsensus => _project.containsKey('consensus');

  @override
  void initState() {
    super.initState();
    _invalid = const Engine().validate(_project);
    if (widget.runOnOpen && _invalid == null) {
      WidgetsBinding.instance.addPostFrameCallback((_) => _run());
    }
  }

  @override
  void dispose() {
    _tabs.dispose();
    super.dispose();
  }

  void _changed() {
    setState(() {
      _dirty = true;
      _stale = _response != null;
      _invalid = const Engine().validate(_project);
    });
  }

  void _replace(Map<String, dynamic> project) {
    setState(() {
      _project = project;
      _revision++;
    });
    _changed();
  }

  String _today() {
    final d = DateTime.now().toUtc();
    String two(int v) => v.toString().padLeft(2, '0');
    return '${d.year}-${two(d.month)}-${two(d.day)}';
  }

  Future<void> _run() async {
    final lang = AppScope.of(context).lang;
    setState(() {
      _running = true;
      _engineError = null;
    });
    try {
      final r = await const Engine().evaluate(_project, EvaluateOptions(lang: lang, generated: _today()));
      if (!mounted) return;
      setState(() {
        _response = r;
        _stale = false;
      });
      _tabs.animateTo(4);
    } on Object catch (e) {
      if (!mounted) return;
      setState(() => _engineError = e.toString());
    } finally {
      if (mounted) setState(() => _running = false);
    }
  }

  String _suggestedName(String ext) {
    final id = (_project['metadata']?['id'] ?? 'project').toString();
    return '$id.$ext';
  }

  Future<void> _save({bool saveAs = false}) async {
    final s = AppScope.of(context).s;
    final text = '${const JsonEncoder.withIndent('  ').convert(_project)}\n';
    if (saveAs && isMobile) {
      await shareText(_suggestedName('json'), text);
      return;
    }
    var path = _path;
    if (path == null || saveAs) {
      const group = XTypeGroup(label: 'JSON', extensions: ['json']);
      path = await chooseSavePath(_suggestedName('json'), group);
      if (path == null) return;
    }
    await File(path).writeAsString(text);
    if (!mounted) return;
    setState(() {
      _path = path;
      _dirty = false;
    });
    ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('${s['saved']}: $path')));
  }

  Future<void> _export(String format) async {
    final app = AppScope.of(context);
    final s = app.s;
    try {
      final r = await const Engine().evaluate(
        _project,
        EvaluateOptions(
          lang: app.lang,
          html: format == 'html',
          markdown: format == 'md',
          pdf: format == 'pdf',
          xlsx: format == 'xlsx',
          // The workbook recomputes the GUM and other results, not the Monte Carlo ones.
          skipMcm: format == 'xlsx',
          generated: _today(),
        ),
      );
      final bytes = switch (format) {
        'pdf' => base64Decode(r['pdf_base64'] as String),
        'xlsx' => base64Decode(r['xlsx_base64'] as String),
        _ => utf8.encode(r[format == 'html' ? 'html' : 'markdown'] as String),
      };
      if (isMobile) {
        await shareBytes(_suggestedName(format), bytes);
        return;
      }
      final group = XTypeGroup(label: format.toUpperCase(), extensions: [format]);
      final path = await chooseSavePath(_suggestedName(format), group);
      if (path == null) return;
      await File(path).writeAsBytes(bytes);
      if (!mounted) return;
      final note = format == 'xlsx' ? '\n${s['xlsx_saved']}' : '';
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('${s['saved']}: $path$note')));
    } on Object catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('${s['engine_error']}: $e')));
    }
  }

  void _openGuide(String target) {
    if (target.startsWith('example:') || target.startsWith('http')) return;
    Navigator.of(context).push(MaterialPageRoute(builder: (_) => GuideScreen(initial: target)));
  }

  @override
  Widget build(BuildContext context) {
    final app = AppScope.of(context);
    final s = app.s;
    final title = s.text(_project['metadata']?['title']);
    return Scaffold(
      appBar: AppBar(
        title: Text(title.isEmpty ? s['app'] : title, overflow: TextOverflow.ellipsis),
        actions: [
          TextButton(
            style: TextButton.styleFrom(foregroundColor: Theme.of(context).appBarTheme.foregroundColor),
            onPressed: app.toggleLanguage,
            child: Text(app.lang == 'en' ? 'TR' : 'EN'),
          ),
          IconButton(
            key: const Key('project-help'),
            tooltip: s['help'],
            onPressed: () => _openGuide(helpChapterFor(_project)),
            icon: const Icon(Icons.help_outline),
          ),
          IconButton(tooltip: s['save'], onPressed: () => _save(), icon: const Icon(Icons.save_outlined)),
          PopupMenuButton<String>(
            onSelected: (v) => switch (v) {
              'save_as' => _save(saveAs: true),
              'html' => _export('html'),
              'pdf' => _export('pdf'),
              'xlsx' => _export('xlsx'),
              _ => _export('md'),
            },
            itemBuilder: (_) => [
              PopupMenuItem(value: 'save_as', child: Text(s['save_as'])),
              PopupMenuItem(value: 'pdf', child: Text(s['export_pdf'])),
              PopupMenuItem(value: 'html', child: Text(s['export_html'])),
              PopupMenuItem(value: 'md', child: Text(s['export_md'])),
              if (hasVerificationWorkbook(_project))
                PopupMenuItem(key: const Key('export-xlsx'), value: 'xlsx', child: Text(s['export_xlsx'])),
            ],
          ),
        ],
        bottom: TabBar(
          controller: _tabs,
          isScrollable: true,
          tabs: [
            Tab(text: s['overview']),
            Tab(
              text:
                  _isTopDown ||
                      _isProficiency ||
                      _isCalibration ||
                      _isConformity ||
                      _isPrecision ||
                      _isControlChart ||
                      _isSampling ||
                      _isErrorFunction ||
                      _isConsensus
                  ? s['data']
                  : s['model'],
            ),
            Tab(text: s['settings']),
            Tab(text: s['json']),
            Tab(key: const Key('results-tab'), text: s['results']),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton.extended(
        key: const Key('run'),
        onPressed: _running || _invalid != null ? null : _run,
        icon: _running ? const SizedBox(width: 18, height: 18, child: CircularProgressIndicator(strokeWidth: 2)) : const Icon(Icons.play_arrow),
        label: Text(_running ? s['running'] : s['run']),
      ),
      body: Column(
        children: [
          if (_invalid != null) _Banner(text: '${s['invalid']}: $_invalid', error: true),
          if (_engineError != null) _Banner(text: '${s['engine_error']}: $_engineError', error: true),
          if (_dirty && _invalid == null) _Banner(text: s['unsaved'], error: false),
          Expanded(
            child: TabBarView(
              controller: _tabs,
              children: [
                OverviewTab(key: ValueKey('o$_revision'), project: _project, onChanged: _changed, onLink: _openGuide),
                _isTopDown
                    ? TopDownEditor(key: ValueKey('t$_revision'), project: _project, onChanged: _changed)
                    : _isProficiency
                    ? ProficiencyEditor(key: ValueKey('p$_revision'), project: _project, onChanged: _changed)
                    : _isCalibration
                    ? CalibrationEditor(key: ValueKey('c$_revision'), project: _project, onChanged: _changed)
                    : _isConformity
                    ? ConformityEditor(key: ValueKey('f$_revision'), project: _project, onChanged: _changed)
                    : _isPrecision
                    ? PrecisionEditor(key: ValueKey('e$_revision'), project: _project, onChanged: _changed)
                    : _isControlChart
                    ? ControlChartEditor(key: ValueKey('q$_revision'), project: _project, onChanged: _changed)
                    : _isSampling
                    ? SamplingEditor(key: ValueKey('u$_revision'), project: _project, onChanged: _changed)
                    : _isErrorFunction
                    ? ErrorFunctionEditor(key: ValueKey('w$_revision'), project: _project, onChanged: _changed)
                    : _isConsensus
                    ? ConsensusEditor(key: ValueKey('n$_revision'), project: _project, onChanged: _changed)
                    : ModelEditor(key: ValueKey('m$_revision'), project: _project, onChanged: _changed),
                SettingsEditor(key: ValueKey('s$_revision'), project: _project, onChanged: _changed),
                JsonEditor(key: ValueKey('j$_revision'), project: _project, onApply: _replace),
                ResultsView(response: _response, stale: _stale, strings: S(app.lang)),
              ],
            ),
          ),
        ],
      ),
    );
  }
}

class _Banner extends StatelessWidget {
  const _Banner({required this.text, required this.error});
  final String text;
  final bool error;

  @override
  Widget build(BuildContext context) {
    final c = Theme.of(context).colorScheme;
    return Material(
      color: error ? c.errorContainer : c.secondaryContainer,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        child: Row(
          children: [
            Icon(error ? Icons.error_outline : Icons.edit_note, color: error ? c.onErrorContainer : c.onSecondaryContainer),
            const SizedBox(width: 8),
            Expanded(
              child: Text(text, style: TextStyle(color: error ? c.onErrorContainer : c.onSecondaryContainer)),
            ),
          ],
        ),
      ),
    );
  }
}
