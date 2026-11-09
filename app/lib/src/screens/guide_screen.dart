import 'package:flutter/material.dart';
import 'package:flutter/services.dart' show Clipboard, ClipboardData;

import '../app_state.dart';
import '../learn/guide.dart';
import '../learn/markdown_view.dart';
import '../library.dart';
import 'project_screen.dart';

/// The beginner's guide "Uncertainty from scratch": chapter list, search,
/// chapter text, previous and next. [initial] is a chapter id, optionally
/// with an anchor ("09-laboratory-data#topdown").
class GuideScreen extends StatefulWidget {
  const GuideScreen({super.key, this.initial});
  final String? initial;

  @override
  State<GuideScreen> createState() => _GuideScreenState();
}

class _GuideData {
  _GuideData(this.chapters, this.lessons, this.library);
  final List<GuideChapter> chapters;
  final Lessons lessons;
  final List<LibraryEntry> library;
}

class _GuideScreenState extends State<GuideScreen> {
  String? _lang;
  late Future<_GuideData> _data;
  late String _current = (widget.initial ?? guideChapters.first).split('#').first;
  String? _anchor;
  String _query = '';
  final _scroll = ScrollController();
  final _anchors = <String, GlobalKey>{};

  @override
  void initState() {
    super.initState();
    final i = widget.initial;
    if (i != null && i.contains('#')) _anchor = i.split('#').last;
  }

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    final lang = AppScope.of(context).lang;
    if (lang != _lang) {
      _lang = lang;
      _data = _load(lang);
    }
  }

  @override
  void dispose() {
    _scroll.dispose();
    super.dispose();
  }

  Future<_GuideData> _load(String lang) async {
    final chapters = await loadGuide(lang);
    final lessons = await loadLessons();
    final library = await loadLibrary();
    chapters.add(GuideChapter(examplesChapter, examplesGuide(lessons, [for (final e in library) e.project], lang)));
    return _GuideData(chapters, lessons, library);
  }

  void _open(String target, _GuideData data) {
    if (target.startsWith('example:')) {
      final id = target.substring('example:'.length);
      final e = data.library.where((x) => x.id == id).firstOrNull;
      if (e != null) {
        Navigator.of(context).push(MaterialPageRoute(builder: (_) => ProjectScreen(project: deepCopy(e.project), fromLibrary: true)));
      }
      return;
    }
    if (target.startsWith('http')) {
      // No browser plug-in: the address is copied for the user.
      Clipboard.setData(ClipboardData(text: target));
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('${AppScope.of(context).s['link_copied']}: $target')));
      return;
    }
    final parts = target.split('#');
    setState(() {
      _current = parts.first;
      _anchor = parts.length > 1 ? parts[1] : null;
      _anchors.clear();
    });
    _afterOpen();
  }

  void _afterOpen() {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (!mounted) return;
      final key = _anchor == null ? null : _anchors[_anchor];
      if (key?.currentContext != null) {
        Scrollable.ensureVisible(key!.currentContext!, duration: const Duration(milliseconds: 250));
      } else if (_scroll.hasClients) {
        _scroll.jumpTo(0);
      }
    });
  }

  Widget _toc(_GuideData data, {VoidCallback? onPicked}) {
    final s = AppScope.of(context).s;
    final c = Theme.of(context).colorScheme;
    final q = _query.trim().toLowerCase();
    final shown = q.isEmpty ? data.chapters : data.chapters.where((ch) => plainText(ch.text).toLowerCase().contains(q)).toList();
    return Column(
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(12, 12, 12, 8),
          child: TextField(
            key: const Key('guide-search'),
            decoration: InputDecoration(prefixIcon: const Icon(Icons.search), hintText: s['guide_search'], isDense: true),
            onChanged: (v) => setState(() => _query = v),
          ),
        ),
        Expanded(
          child: ListView(
            children: [
              for (final ch in shown)
                ListTile(
                  dense: true,
                  selected: ch.id == _current,
                  selectedTileColor: c.primaryContainer,
                  selectedColor: c.onPrimaryContainer,
                  title: Text(ch.title, style: TextStyle(fontWeight: ch.id == _current ? FontWeight.w700 : FontWeight.w500)),
                  onTap: () {
                    onPicked?.call();
                    _open(ch.id, data);
                  },
                ),
              if (shown.isEmpty) Padding(padding: const EdgeInsets.all(16), child: Text(s['guide_nothing'])),
            ],
          ),
        ),
      ],
    );
  }

  Widget _content(_GuideData data) {
    final s = AppScope.of(context).s;
    final index = data.chapters.indexWhere((ch) => ch.id == _current);
    final ch = data.chapters[index < 0 ? 0 : index];
    final prev = index > 0 ? data.chapters[index - 1] : null;
    final next = index >= 0 && index < data.chapters.length - 1 ? data.chapters[index + 1] : null;
    return SelectionArea(
      child: SingleChildScrollView(
        key: PageStorageKey('guide-${ch.id}'),
        controller: _scroll,
        padding: const EdgeInsets.fromLTRB(24, 16, 24, 40),
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 880),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                MarkdownView(ch.text, key: ValueKey('${ch.id}-$_lang'), anchors: _anchors, onLink: (t) => _open(t, data)),
                const SizedBox(height: 24),
                Row(
                  children: [
                    if (prev != null)
                      Flexible(
                        child: OutlinedButton.icon(
                          icon: const Icon(Icons.arrow_back),
                          label: Text('${s['previous']}: ${prev.title}', overflow: TextOverflow.ellipsis),
                          onPressed: () => _open(prev.id, data),
                        ),
                      ),
                    const Spacer(),
                    if (next != null)
                      Flexible(
                        child: FilledButton.icon(
                          key: const Key('guide-next'),
                          icon: const Icon(Icons.arrow_forward),
                          iconAlignment: IconAlignment.end,
                          label: Text('${s['next']}: ${next.title}', overflow: TextOverflow.ellipsis),
                          onPressed: () => _open(next.id, data),
                        ),
                      ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final app = AppScope.of(context);
    final s = app.s;
    return FutureBuilder<_GuideData>(
      future: _data,
      builder: (context, snap) {
        final data = snap.data;
        return LayoutBuilder(
          builder: (context, box) {
            final wide = box.maxWidth >= 900;
            return Scaffold(
              appBar: AppBar(
                title: Text(s['guide']),
                actions: [
                  TextButton(
                    style: TextButton.styleFrom(foregroundColor: Theme.of(context).appBarTheme.foregroundColor),
                    onPressed: app.toggleLanguage,
                    child: Text(app.lang == 'en' ? 'TR' : 'EN'),
                  ),
                ],
              ),
              drawer: !wide && data != null
                  ? Drawer(
                      child: SafeArea(
                        child: Builder(builder: (ctx) => _toc(data, onPicked: () => Navigator.pop(ctx))),
                      ),
                    )
                  : null,
              body: data == null
                  ? Center(child: snap.hasError ? Text('${snap.error}') : const CircularProgressIndicator())
                  : wide
                  ? Row(
                      children: [
                        SizedBox(
                          width: 300,
                          child: Material(color: Theme.of(context).colorScheme.surface, child: _toc(data)),
                        ),
                        const VerticalDivider(width: 1),
                        Expanded(child: _content(data)),
                      ],
                    )
                  : _content(data),
            );
          },
        );
      },
    );
  }
}
