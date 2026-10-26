import 'package:flutter/material.dart';

import '../app_state.dart';
import 'fields.dart';

/// Title, description, explanation, notes and sources of a project.
class OverviewTab extends StatelessWidget {
  const OverviewTab({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

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
    final meta = (project['metadata'] ??= <String, dynamic>{'title': {'en': ''}}) as Map<String, dynamic>;
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
        if (description != null) ...[
          Section(s['description']),
          Text(s.text(description)),
        ],
        if (explanation.isNotEmpty) ...[
          Section(s['explanation']),
          for (var i = 0; i < explanation.length; i++)
            Padding(
              padding: const EdgeInsets.only(bottom: 6),
              child: Text('${i + 1}. ${s.text(explanation[i])}'),
            ),
        ],
        if (notes.isNotEmpty) ...[
          Section(s['notes']),
          for (final n in notes)
            Padding(padding: const EdgeInsets.only(bottom: 6), child: Text('• ${s.text(n)}')),
        ],
        if (sources.isNotEmpty) ...[
          Section(s['sources']),
          for (final src in sources)
            SelectableText([
              (src as Map)['document'],
              if (src['section'] != null) src['section'],
              if (src['url'] != null) src['url'],
            ].join(', ')),
        ],
      ],
    );
  }
}
