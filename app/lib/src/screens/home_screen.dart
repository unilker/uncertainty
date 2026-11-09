import 'dart:convert';
import 'dart:io';

import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';

import '../app_state.dart';
import '../engine.dart';
import '../library.dart';
import '../theme.dart';
import 'project_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  late final Future<List<LibraryEntry>> _library = loadLibrary();
  String _query = '';

  void _openProject(Map<String, dynamic> project, {String? path, bool fromLibrary = false}) {
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => ProjectScreen(project: deepCopy(project), filePath: path, fromLibrary: fromLibrary),
      ),
    );
  }

  Future<void> _openFile() async {
    final s = AppScope.of(context).s;
    const group = XTypeGroup(label: 'JSON', extensions: ['json']);
    final file = await openFile(acceptedTypeGroups: [group]);
    if (file == null || !mounted) return;
    try {
      final project = jsonDecode(await File(file.path).readAsString()) as Map<String, dynamic>;
      _openProject(project, path: file.path);
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('${s['file_error']}: $e')));
    }
  }

  void _about() {
    final s = AppScope.of(context).s;
    Map<String, dynamic>? v;
    String? error;
    try {
      v = const Engine().version();
    } on Object catch (e) {
      error = e.toString();
    }
    showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(s['app']),
        content: SelectableText(
          error ??
              '${v!['name']} ${v['version']}\n\nİlker Ün, TÜBİTAK UME\nORCID 0000-0001-5089-4602\n\n'
                  'GPL-3.0-or-later\n\n${s['cite']}:\n${v['citation']}',
        ),
        actions: [TextButton(onPressed: () => Navigator.pop(context), child: Text(s['close']))],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final app = AppScope.of(context);
    final s = app.s;
    return Scaffold(
      appBar: AppBar(
        title: Text(s['app']),
        actions: [
          TextButton(
            key: const Key('language'),
            style: TextButton.styleFrom(foregroundColor: Theme.of(context).appBarTheme.foregroundColor),
            onPressed: app.toggleLanguage,
            child: Text(app.lang == 'en' ? 'TR' : 'EN'),
          ),
          IconButton(tooltip: s['about'], onPressed: _about, icon: const Icon(Icons.info_outline)),
        ],
      ),
      body: FutureBuilder<List<LibraryEntry>>(
        future: _library,
        builder: (context, snap) {
          final entries = snap.data ?? const <LibraryEntry>[];
          final q = _query.trim().toLowerCase();
          final shown = q.isEmpty ? entries : entries.where((e) => e.searchText.contains(q)).toList();
          final fields = <String, List<LibraryEntry>>{};
          for (final e in shown) {
            fields.putIfAbsent(e.field ?? '', () => []).add(e);
          }
          return ListView(
            padding: const EdgeInsets.all(16),
            children: [
              _ActionGrid(
                heading: tubitakDesign ? s['start_heading'] : null,
                cards: [
                  _ActionCard(
                    icon: Icons.functions,
                    title: s['new_model'],
                    subtitle: s['new_model_sub'],
                    onTap: () => _openProject(const Engine().template('model')),
                  ),
                  _ActionCard(
                    icon: Icons.science_outlined,
                    title: s['new_topdown'],
                    subtitle: s['new_topdown_sub'],
                    onTap: () => _openProject(const Engine().template('topdown')),
                  ),
                  _ActionCard(
                    icon: Icons.groups_outlined,
                    title: s['new_pt'],
                    subtitle: s['new_pt_sub'],
                    onTap: () => _openProject(const Engine().template('proficiency')),
                  ),
                  _ActionCard(
                    icon: Icons.show_chart,
                    title: s['new_cal'],
                    subtitle: s['new_cal_sub'],
                    onTap: () => _openProject(const Engine().template('calibration')),
                  ),
                  _ActionCard(
                    icon: Icons.rule,
                    title: s['new_cf'],
                    subtitle: s['new_cf_sub'],
                    onTap: () => _openProject(const Engine().template('conformity')),
                  ),
                  _ActionCard(
                    icon: Icons.hub_outlined,
                    title: s['new_pe'],
                    subtitle: s['new_pe_sub'],
                    onTap: () => _openProject(const Engine().template('precision')),
                  ),
                  _ActionCard(
                    icon: Icons.scatter_plot_outlined,
                    title: s['new_mv'],
                    subtitle: s['new_mv_sub'],
                    onTap: () => _openProject(const Engine().template('multivariate')),
                  ),
                  _ActionCard(
                    icon: Icons.stacked_line_chart,
                    title: s['new_qc'],
                    subtitle: s['new_qc_sub'],
                    onTap: () => _openProject(const Engine().template('control_chart')),
                  ),
                  _ActionCard(icon: Icons.grain, title: s['new_sp'], subtitle: s['new_sp_sub'], onTap: () => _openProject(const Engine().template('sampling'))),
                  _ActionCard(
                    icon: Icons.scale_outlined,
                    title: s['new_ef'],
                    subtitle: s['new_ef_sub'],
                    onTap: () => _openProject(const Engine().template('error_function')),
                  ),
                  _ActionCard(
                    icon: Icons.handshake_outlined,
                    title: s['new_cs'],
                    subtitle: s['new_cs_sub'],
                    onTap: () => _openProject(const Engine().template('consensus')),
                  ),
                  _ActionCard(icon: Icons.folder_open, title: s['open'], subtitle: s['open_sub'], onTap: _openFile, accent: true),
                ],
              ),
              const SizedBox(height: 24),
              _Heading('${s['library']} (${entries.length} ${s['examples']})'),
              const SizedBox(height: 8),
              TextField(
                key: const Key('search'),
                decoration: InputDecoration(prefixIcon: const Icon(Icons.search), hintText: s['search']),
                onChanged: (v) => setState(() => _query = v),
              ),
              const SizedBox(height: 8),
              if (snap.hasError) Text('${snap.error}'),
              for (final f in fields.entries)
                _FieldTile(
                  child: ExpansionTile(
                    key: PageStorageKey('field-${f.key}-${q.isNotEmpty}'),
                    initiallyExpanded: q.isNotEmpty,
                    title: Text('${s.field(f.key)} (${f.value.length})'),
                    tilePadding: tubitakDesign ? const EdgeInsets.symmetric(horizontal: 16) : null,
                    children: [
                      for (final e in f.value)
                        ListTile(
                          title: Text(s.text(e.metadata['title'])),
                          subtitle: Text('${e.id} · ${e.methodLabel}'),
                          trailing: const Icon(Icons.chevron_right),
                          onTap: () => _openProject(e.project, fromLibrary: true),
                        ),
                    ],
                  ),
                ),
            ],
          );
        },
      ),
    );
  }
}

/// The cards that start a new project. With the TÜBİTAK design all cards have
/// the same size and fill the width in as many columns as fit.
class _ActionGrid extends StatelessWidget {
  const _ActionGrid({required this.cards, this.heading});
  final List<_ActionCard> cards;
  final String? heading;

  static const double _minWidth = 320;
  static const double _height = 120;
  static const double _gap = 14;

  @override
  Widget build(BuildContext context) {
    if (!tubitakDesign) return Wrap(spacing: 12, runSpacing: 12, children: cards);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        if (heading != null) ...[_Heading(heading!), const SizedBox(height: 12)],
        LayoutBuilder(
          builder: (context, box) {
            // The largest column count that fits and divides the number of cards, so that no row is ragged.
            final fit = ((box.maxWidth + _gap) / (_minWidth + _gap)).floor().clamp(1, 6);
            var columns = fit;
            while (columns > 1 && cards.length % columns != 0) {
              columns--;
            }
            final width = (box.maxWidth - _gap * (columns - 1)) / columns;
            return Wrap(
              spacing: _gap,
              runSpacing: _gap,
              children: [for (final c in cards) SizedBox(width: width, height: _height, child: c)],
            );
          },
        ),
      ],
    );
  }
}

/// A heading with a red bar in the TÜBİTAK design.
class _Heading extends StatelessWidget {
  const _Heading(this.text);
  final String text;

  @override
  Widget build(BuildContext context) {
    final style = Theme.of(context).textTheme.titleLarge;
    if (!tubitakDesign) return Text(text, style: style);
    return Row(
      children: [
        Container(
          width: 4,
          height: 24,
          decoration: BoxDecoration(color: Brand.redBright, borderRadius: BorderRadius.circular(2)),
        ),
        const SizedBox(width: 10),
        Expanded(child: Text(text, style: style)),
      ],
    );
  }
}

/// Spacing between the library fields in the TÜBİTAK design.
class _FieldTile extends StatelessWidget {
  const _FieldTile({required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) => tubitakDesign ? Padding(padding: const EdgeInsets.only(bottom: 8), child: child) : child;
}

class _ActionCard extends StatefulWidget {
  const _ActionCard({required this.icon, required this.title, required this.subtitle, required this.onTap, this.accent = false});
  final IconData icon;
  final String title;
  final String subtitle;
  final VoidCallback onTap;

  /// Shown in red: an action other than starting a new project.
  final bool accent;

  @override
  State<_ActionCard> createState() => _ActionCardState();
}

class _ActionCardState extends State<_ActionCard> {
  bool _hover = false;

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    if (!tubitakDesign) {
      return SizedBox(
        width: 300,
        child: Card(
          child: InkWell(
            onTap: widget.onTap,
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Row(
                children: [
                  Icon(widget.icon, size: 32, color: theme.colorScheme.primary),
                  const SizedBox(width: 16),
                  Expanded(
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(widget.title, style: theme.textTheme.titleMedium),
                        Text(widget.subtitle, style: theme.textTheme.bodySmall),
                      ],
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      );
    }
    final c = theme.colorScheme;
    final dark = c.brightness == Brightness.dark;
    return MouseRegion(
      onEnter: (_) => setState(() => _hover = true),
      onExit: (_) => setState(() => _hover = false),
      child: AnimatedContainer(
        duration: const Duration(milliseconds: 150),
        decoration: BoxDecoration(
          color: c.surface,
          borderRadius: BorderRadius.circular(12),
          border: Border.all(color: _hover ? c.secondary : c.outlineVariant, width: _hover ? 1.5 : 1),
          boxShadow: [
            BoxShadow(
              color: (dark ? Colors.black : Brand.navy).withValues(alpha: _hover ? 0.16 : 0.06),
              blurRadius: _hover ? 16 : 8,
              offset: const Offset(0, 3),
            ),
          ],
        ),
        child: Material(
          type: MaterialType.transparency,
          child: InkWell(
            borderRadius: BorderRadius.circular(12),
            hoverColor: c.secondary.withValues(alpha: 0.04),
            splashColor: c.secondary.withValues(alpha: 0.12),
            onTap: widget.onTap,
            child: Row(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Container(
                  width: 5,
                  decoration: BoxDecoration(
                    color: _hover || widget.accent ? c.secondary : c.primary,
                    borderRadius: const BorderRadius.horizontal(left: Radius.circular(11)),
                  ),
                ),
                const SizedBox(width: 14),
                Center(
                  child: Container(
                    width: 48,
                    height: 48,
                    decoration: BoxDecoration(
                      color: widget.accent ? Brand.red : (dark ? c.primaryContainer : Brand.navy),
                      borderRadius: BorderRadius.circular(12),
                    ),
                    child: Icon(widget.icon, size: 26, color: Colors.white),
                  ),
                ),
                const SizedBox(width: 14),
                Expanded(
                  child: Padding(
                    padding: const EdgeInsets.fromLTRB(0, 14, 14, 14),
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Text(
                          widget.title,
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                          style: theme.textTheme.titleMedium?.copyWith(color: c.primary, height: 1.2),
                        ),
                        const SizedBox(height: 4),
                        Text(
                          widget.subtitle,
                          maxLines: 2,
                          overflow: TextOverflow.ellipsis,
                          style: theme.textTheme.bodySmall?.copyWith(color: c.onSurfaceVariant, height: 1.3),
                        ),
                      ],
                    ),
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
