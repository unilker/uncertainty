import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'fields.dart';

/// Lines "laboratory value value …" (tab or semicolon separated, or spaces
/// with a laboratory name without spaces; decimal comma accepted). Returns
/// the results by laboratory in the order given, or null and the 1-based
/// number of the first line that cannot be read.
(List<MapEntry<String, List<double>>>?, int?) parseCells(String text) {
  final out = <MapEntry<String, List<double>>>[];
  var lineNo = 0;
  for (final line in text.split('\n')) {
    lineNo++;
    if (line.trim().isEmpty) continue;
    final t = line.trim();
    final parts = t.contains(RegExp(r'\t|;')) ? t.split(RegExp(r'\t|;')).map((c) => c.trim()).toList() : t.split(RegExp(r'\s+'));
    final name = parts.first;
    if (name.isEmpty || out.any((e) => e.key == name)) return (null, lineNo);
    final values = <double>[];
    for (final c in parts.skip(1)) {
      if (c.isEmpty || c == '-' || c == '—') continue;
      final v = parseNum(c);
      if (v == null) return (null, lineNo);
      values.add(v.toDouble());
    }
    out.add(MapEntry(name, values));
  }
  return (out, null);
}

String cellsToText(List<String> labs, List<dynamic> results, String lang) => [
  for (var i = 0; i < labs.length; i++) [labs[i], for (final v in (i < results.length ? results[i] as List : const [])) fmtGivenIn(v as num, lang)].join('\t'),
].join('\n');

/// Lines "laboratory; level; result; reason" with level and result optional
/// (an empty level means all levels, an empty result the whole cell).
(List<Map<String, dynamic>>?, int?) parseExclusions(String text) {
  final out = <Map<String, dynamic>>[];
  var lineNo = 0;
  for (final line in text.split('\n')) {
    lineNo++;
    if (line.trim().isEmpty) continue;
    final c = line.split(RegExp(r'\t|;')).map((x) => x.trim()).toList();
    if (c.first.isEmpty) return (null, lineNo);
    final x = <String, dynamic>{'laboratory': c.first};
    if (c.length > 1 && c[1].isNotEmpty) x['level'] = c[1];
    if (c.length > 2 && c[2].isNotEmpty) {
      final r = int.tryParse(c[2]);
      if (r == null || r < 1) return (null, lineNo);
      x['result'] = r;
    }
    if (c.length > 3 && c.sublist(3).join('; ').isNotEmpty) x['reason'] = c.sublist(3).join('; ');
    out.add(x);
  }
  return (out, null);
}

String exclusionsToText(List<dynamic> rows, String lang) => rows
    .map((r) {
      final m = r as Map<String, dynamic>;
      final reason = m['reason'];
      final text = reason is Map ? (reason[lang] ?? reason['en'] ?? '').toString() : (reason ?? '').toString();
      return [m['laboratory'].toString(), (m['level'] ?? '').toString(), (m['result'] ?? '').toString(), text].join('; ').replaceAll(RegExp(r'(; )+$'), '');
    })
    .join('\n');

/// Editor of an interlaboratory precision experiment (ISO 5725-2): the
/// results of the laboratories at each level (Form A), the method, and the
/// results left out after scrutiny.
class PrecisionEditor extends StatefulWidget {
  const PrecisionEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<PrecisionEditor> createState() => _PrecisionEditorState();
}

class _PrecisionEditorState extends State<PrecisionEditor> {
  Map<String, dynamic> get _pe => widget.project['precision_experiment'] as Map<String, dynamic>;
  List<dynamic> get _levels => _pe['levels'] as List;

  /// The parsed cells of each level, kept while a level is being edited.
  final Map<int, List<MapEntry<String, List<double>>>> _parsed = {};
  final Map<int, int?> _bad = {};
  int? _badExclusion;
  int _revision = 0;

  List<MapEntry<String, List<double>>> _cellsOf(int j) {
    final labs = (_pe['laboratories'] as List).cast<String>();
    final results = (_levels[j] as Map<String, dynamic>)['results'] as List;
    return [
      for (var i = 0; i < labs.length; i++) MapEntry(labs[i], [for (final v in (i < results.length ? results[i] as List : const [])) (v as num).toDouble()]),
    ];
  }

  /// Rebuilds the laboratory list (in order of first appearance) and aligns
  /// the results of every level to it.
  void _rebuild() {
    final cells = [for (var j = 0; j < _levels.length; j++) _parsed[j] ?? _cellsOf(j)];
    final labs = <String>[];
    for (final level in cells) {
      for (final e in level) {
        if (!labs.contains(e.key)) labs.add(e.key);
      }
    }
    _pe['laboratories'] = labs;
    for (var j = 0; j < _levels.length; j++) {
      final byLab = {for (final e in cells[j]) e.key: e.value};
      (_levels[j] as Map<String, dynamic>)['results'] = [for (final l in labs) byLab[l] ?? <double>[]];
    }
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final labs = (_pe['laboratories'] as List).cast<String>();
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            StringField(map: _pe, field: 'quantity', label: s['quantity_name'], width: 240, onChanged: widget.onChanged),
            StringField(map: _pe, field: 'unit', label: s['unit'], width: 120, onChanged: widget.onChanged),
            Labeled(
              label: s['pe_method'],
              width: 280,
              child: DropdownButtonFormField<String>(
                key: const Key('pe-method'),
                isExpanded: true,
                initialValue: _pe['method'] as String? ?? 'anova',
                decoration: fieldDecoration(s['pe_method']),
                items: [
                  for (final m in ['anova', 'reml'])
                    DropdownMenuItem(
                      value: m,
                      child: Text(s['pe_$m'], overflow: TextOverflow.ellipsis),
                    ),
                ],
                onChanged: (v) {
                  if (v == 'anova') {
                    _pe.remove('method');
                  } else {
                    _pe['method'] = v;
                  }
                  widget.onChanged();
                },
              ),
            ),
            NumberField(map: _pe, field: 'form_decimals', label: s['pe_decimals'], integer: true, width: 220, onChanged: widget.onChanged),
          ],
        ),
        Section(s['pe_levels']),
        Text(s['pe_levels_help'], style: Theme.of(context).textTheme.bodySmall),
        for (var j = 0; j < _levels.length; j++)
          Card(
            key: ValueKey('pe-level-$j-$_revision'),
            child: Padding(
              padding: const EdgeInsets.all(12),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      StringField(map: _levels[j] as Map<String, dynamic>, field: 'name', label: s['pe_level'], width: 160, onChanged: widget.onChanged),
                      const Spacer(),
                      IconButton(
                        tooltip: s['remove'],
                        icon: const Icon(Icons.delete_outline),
                        onPressed: _levels.length <= 1
                            ? null
                            : () => setState(() {
                                _levels.removeAt(j);
                                _parsed.clear();
                                _bad.clear();
                                _revision++;
                                _rebuild();
                              }),
                      ),
                    ],
                  ),
                  TextFormField(
                    key: Key('pe-cells-$j'),
                    initialValue: cellsToText(labs, (_levels[j] as Map<String, dynamic>)['results'] as List, s.lang),
                    minLines: 3,
                    maxLines: 20,
                    style: const TextStyle(fontFamily: 'monospace'),
                    decoration: InputDecoration(errorText: _bad[j] == null ? null : '${s['pt_results_error']} ${_bad[j]}', alignLabelWithHint: true),
                    onChanged: (t) {
                      final (cells, bad) = parseCells(t);
                      setState(() => _bad[j] = bad);
                      if (cells == null) return;
                      _parsed[j] = cells;
                      _rebuild();
                    },
                  ),
                ],
              ),
            ),
          ),
        Align(
          alignment: Alignment.centerLeft,
          child: TextButton.icon(
            icon: const Icon(Icons.add),
            label: Text(s['pe_add_level']),
            onPressed: () => setState(() {
              final names = _levels.map((l) => (l as Map)['name'].toString()).toSet();
              var k = _levels.length + 1;
              while (names.contains('$k')) {
                k++;
              }
              _levels.add(<String, dynamic>{
                'name': '$k',
                'results': [for (final _ in labs) <double>[]],
              });
              _revision++;
              widget.onChanged();
            }),
          ),
        ),
        Section(s['pe_exclude']),
        TextFormField(
          key: const Key('pe-exclude'),
          initialValue: exclusionsToText((_pe['exclude'] as List?) ?? const [], s.lang),
          minLines: 2,
          maxLines: 10,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(
            helperText: s['pe_exclude_help'],
            helperMaxLines: 3,
            errorText: _badExclusion == null ? null : '${s['pt_results_error']} $_badExclusion',
          ),
          onChanged: (t) {
            final (rows, bad) = parseExclusions(t);
            setState(() => _badExclusion = bad);
            if (rows == null) return;
            if (rows.isEmpty) {
              _pe.remove('exclude');
            } else {
              _pe['exclude'] = rows;
            }
            widget.onChanged();
          },
        ),
      ],
    );
  }
}
