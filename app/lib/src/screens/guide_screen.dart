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
    chapters.add(GuideChapter(examplesChapter, examplesGuide(lessons, library, lang)));
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

/// The generated chapter that lists the examples of the library by level,
/// with the lesson of each group.
String examplesGuide(Lessons lessons, List<LibraryEntry> library, String lang) {
  final tr = lang == 'tr';
  final b = StringBuffer();
  b.writeln(tr ? '# 13. Örneklerle öğrenmek' : '# 13. Learning with the examples');
  b.writeln();
  b.writeln(
    tr
        ? 'Kütüphanedeki ${library.length} örneğin her biri yayımlanmış bir kılavuzdan, standarttan ya da makaleden alınmıştır ve program her değişiklikte onları yeniden hesaplar. Aşağıda örnekler zorluk düzeyine göre gruplanmıştır. Her grubun başında o grubun ne öğrettiği yazar; bir örneğin adına tıklayınca örnek açılır. Açılan örneğin **Genel** sekmesinde, örneğin adım adım açıklaması vardır.'
        : 'Each of the ${library.length} examples of the library comes from a published guide, standard or paper, and the program recomputes them at every change. Below, the examples are grouped by level. Each group starts with what it teaches; click the name of an example to open it. The **Overview** tab of the opened example explains it step by step.',
  );
  b.writeln();
  b.writeln(
    tr
        ? '> **İpucu:** Yeni başlıyorsanız Başlangıç düzeyindeki ilk gruplarla başlayın, her örnekte önce **Hesapla** düğmesine basın, sonra sonuçları açıklamayla karşılaştırın.'
        : '> **Tip:** If you are new, start with the first groups of the Beginner level; in each example press **Calculate** first, then compare the results with the explanation.',
  );
  final byLesson = <Lesson, List<LibraryEntry>>{};
  final other = <LibraryEntry>[];
  for (final e in library) {
    final l = lessons.forId(e.id);
    if (l == null) {
      other.add(e);
    } else {
      byLesson.putIfAbsent(l, () => []).add(e);
    }
  }
  String title(LibraryEntry e) {
    final t = e.metadata['title'];
    return (t is Map ? (t[lang] ?? t['en']) : t).toString();
  }

  for (final level in [1, 2, 3]) {
    final group = lessons.items.where((l) => l.level == level && byLesson.containsKey(l)).toList();
    if (group.isEmpty) continue;
    b.writeln();
    b.writeln('## ${tr ? 'Düzey' : 'Level'} $level: ${levelName(level, lang)} {#level-$level}');
    for (final l in group) {
      b.writeln();
      b.writeln('### ${l.titleIn(lang)}');
      b.writeln();
      b.writeln(l.inLang(lang));
      b.writeln();
      for (final e in byLesson[l]!) {
        b.writeln('- [${title(e)}](example:${e.id})');
      }
    }
  }
  if (other.isNotEmpty) {
    b.writeln();
    b.writeln(tr ? '## Diğer örnekler' : '## Other examples');
    b.writeln();
    for (final e in other) {
      b.writeln('- [${title(e)}](example:${e.id})');
    }
  }
  return b.toString();
}
