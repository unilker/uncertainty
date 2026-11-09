import 'package:flutter/material.dart';

import '../app_state.dart';
import '../learn/explain.dart';
import '../learn/guide.dart';
import '../learn/markdown_view.dart';
import 'fields.dart';

/// Title, description, the lesson of a library example, a plain-language
/// walk-through, the technical explanation, notes and sources of a project.
class OverviewTab extends StatelessWidget {
  const OverviewTab({super.key, required this.project, required this.onChanged, this.onLink});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  /// Opens a link of the walk-through (a guide chapter).
  final void Function(String target)? onLink;

  Map<String, dynamic> _text(Map<String, dynamic> meta, String key) {
    final t = meta[key];
    if (t is Map<String, dynamic>) return t;
    final m = <String, dynamic>{'en': t is String ? t : ''};
    meta[key] = m;
    return m;
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final meta =
        (project['metadata'] ??= <String, dynamic>{
              'title': {'en': ''},
            })
            as Map<String, dynamic>;
    final title = _text(meta, 'title');
    final description = meta['description'] == null ? null : _text(meta, 'description');
    final explanation = (meta['explanation'] as List?) ?? const [];
    final notes = (meta['notes'] as List?) ?? const [];
    final sources = (meta['sources'] as List?) ?? const [];
    return ListView(
      padding: const EdgeInsets.all(16),
      children: [
        StringField(map: title, field: 'en', label: s['title_en'], onChanged: onChanged, keepEmpty: true),
        const SizedBox(height: 8),
        StringField(map: title, field: 'tr', label: s['title_tr'], onChanged: onChanged),
        if (description != null) ...[Section(s['description']), Text(s.text(description))],
        _LessonCard(id: (meta['id'] ?? '').toString()),
        const SizedBox(height: 8),
        Card(
          child: Theme(
            data: Theme.of(context).copyWith(dividerColor: Colors.transparent),
            child: ExpansionTile(
              key: const Key('learn-steps'),
              initiallyExpanded: true,
              leading: Icon(Icons.school_outlined, color: Theme.of(context).colorScheme.secondary),
              title: Text(s['learn_steps'], style: const TextStyle(fontWeight: FontWeight.w700)),
              childrenPadding: const EdgeInsets.fromLTRB(16, 0, 16, 12),
              expandedCrossAxisAlignment: CrossAxisAlignment.start,
              children: [MarkdownView(explainProject(project, s.lang, includeDescription: description == null), compact: true, onLink: onLink)],
            ),
          ),
        ),
        if (explanation.isNotEmpty) ...[
          Section(s['explanation']),
          for (var i = 0; i < explanation.length; i++) Padding(padding: const EdgeInsets.only(bottom: 6), child: Text('${i + 1}. ${s.text(explanation[i])}')),
        ],
        if (notes.isNotEmpty) ...[
          Section(s['notes']),
          for (final n in notes) Padding(padding: const EdgeInsets.only(bottom: 6), child: Text('• ${s.text(n)}')),
        ],
        if (sources.isNotEmpty) ...[
          Section(s['sources']),
          for (final src in sources)
            SelectableText([(src as Map)['document'], if (src['section'] != null) src['section'], if (src['url'] != null) src['url']].join(', ')),
        ],
      ],
    );
  }
}

/// What the library example [id] teaches, with its level (nothing for a
/// project that is not from the library).
class _LessonCard extends StatefulWidget {
  const _LessonCard({required this.id});
  final String id;

  @override
  State<_LessonCard> createState() => _LessonCardState();
}

class _LessonCardState extends State<_LessonCard> {
  late final Future<Lessons> _lessons = loadLessons();

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    return FutureBuilder<Lessons>(
      future: _lessons,
      builder: (context, snap) {
        final lesson = widget.id.isEmpty ? null : snap.data?.forId(widget.id);
        if (lesson == null) return const SizedBox.shrink();
        final c = Theme.of(context).colorScheme;
        return Padding(
          padding: const EdgeInsets.only(top: 12),
          child: Container(
            key: const Key('learn-lesson'),
            padding: const EdgeInsets.all(14),
            decoration: BoxDecoration(
              color: c.secondary.withValues(alpha: 0.06),
              borderRadius: BorderRadius.circular(12),
              border: Border(left: BorderSide(color: c.secondary, width: 4)),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    Icon(Icons.lightbulb_outline, color: c.secondary),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(s['learn_lesson'], style: Theme.of(context).textTheme.titleMedium?.copyWith(color: c.primary)),
                    ),
                    LevelChip(level: lesson.level),
                  ],
                ),
                const SizedBox(height: 4),
                Text(lesson.titleIn(s.lang), style: const TextStyle(fontWeight: FontWeight.w600)),
                const SizedBox(height: 6),
                MarkdownView(lesson.inLang(s.lang), compact: true),
              ],
            ),
          ),
        );
      },
    );
  }
}

/// A small badge with the level of an example.
class LevelChip extends StatelessWidget {
  const LevelChip({super.key, required this.level});
  final int level;

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final c = Theme.of(context).colorScheme;
    final color = switch (level) {
      1 => const Color(0xFF1B7F4C),
      2 => c.primary,
      _ => c.secondary,
    };
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 2),
      decoration: BoxDecoration(
        border: Border.all(color: color),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Text(
        levelName(level, s.lang),
        style: TextStyle(color: color, fontSize: 12, fontWeight: FontWeight.w600),
      ),
    );
  }
}
