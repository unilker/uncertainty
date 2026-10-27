import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'control_chart_editor.dart' show parseRuns;
import 'fields.dart';

/// Lines "I  E-or-m_ref  u(m_ref)  u(I)" of the test points (tab, semicolon
/// or space separated; decimal comma accepted). Returns the rows, or null
/// and the 1-based number of the first line that cannot be read.
(List<List<double>>?, int?) parseTestPoints(String text) {
  final (parsed, bad) = parseRuns(text);
  if (parsed == null) return (null, bad);
  final (labels, rows) = parsed;
  final lineNumbers = [
    for (final (i, l) in text.split('\n').indexed)
      if (l.trim().isNotEmpty) i + 1,
  ];
  for (var i = 0; i < rows.length; i++) {
    if (labels.isNotEmpty && labels[i].isNotEmpty || rows[i].length != 4) return (null, lineNumbers[i]);
  }
  return (rows, null);
}

/// Whether the points give reference masses (else errors).
bool pointsUseReference(Map<String, dynamic> ef) {
  final points = (ef['points'] as List?) ?? const [];
  return points.isEmpty || points.every((p) => (p as Map).containsKey('reference'));
}

String testPointsToText(Map<String, dynamic> ef, String lang) {
  final reference = pointsUseReference(ef);
  return [
    for (final p in ((ef['points'] as List?) ?? const []).cast<Map>())
      [p['indication'], reference ? p['reference'] : p['error'], p['u_reference'], p['u_indication']].map((v) => fmtGivenIn(v as num?, lang)).join('\t'),
  ].join('\n');
}

/// Editor of the error approximation function of a weighing instrument
/// (EURAMET cg-18, Appendix C).
class ErrorFunctionEditor extends StatefulWidget {
  const ErrorFunctionEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<ErrorFunctionEditor> createState() => _ErrorFunctionEditorState();
}

class _ErrorFunctionEditorState extends State<ErrorFunctionEditor> {
  Map<String, dynamic> get _ef => widget.project['error_function'] as Map<String, dynamic>;
  late bool _reference = pointsUseReference(_ef);
  int? _badLine;
  String _text = '';

  @override
  void initState() {
    super.initState();
    _text = testPointsToText(_ef, 'en');
  }

  void _setData(String text) {
    _text = text;
    final (rows, bad) = parseTestPoints(text);
    setState(() => _badLine = bad);
    if (rows == null) return;
    _ef['points'] = [
      for (final r in rows) {'indication': r[0], _reference ? 'reference' : 'error': r[1], 'u_reference': r[2], 'u_indication': r[3]},
    ];
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            StringField(map: _ef, field: 'instrument', label: s['ef_instrument'], width: 320, onChanged: widget.onChanged),
            StringField(map: _ef, field: 'unit', label: s['ef_unit'], width: 140, onChanged: widget.onChanged),
            StringField(map: _ef, field: 'error_unit', label: s['ef_error_unit'], width: 140, onChanged: widget.onChanged),
            NumberField(map: _ef, field: 'error_factor', label: s['ef_error_factor'], width: 180, onChanged: widget.onChanged),
          ],
        ),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.center,
          children: [
            NumberField(map: _ef, field: 'degree', label: s['ef_degree'], integer: true, width: 140, onChanged: widget.onChanged),
            SizedBox(
              width: 260,
              child: SwitchListTile(
                key: const Key('ef-through-zero'),
                contentPadding: EdgeInsets.zero,
                title: Text(s['ef_through_zero']),
                value: _ef['through_zero'] == true,
                onChanged: (v) {
                  setState(() => _ef['through_zero'] = v);
                  widget.onChanged();
                },
              ),
            ),
            NumberField(map: _ef, field: 's_m', label: s['ef_s_m'], width: 180, onChanged: widget.onChanged),
            NumberField(map: _ef, field: 'coverage_factor', label: s['k'], width: 140, onChanged: widget.onChanged),
          ],
        ),
        Section(s['ef_points']),
        SizedBox(
          width: 360,
          child: SwitchListTile(
            key: const Key('ef-reference'),
            contentPadding: EdgeInsets.zero,
            title: Text(s['ef_reference']),
            value: _reference,
            onChanged: (v) {
              setState(() => _reference = v);
              _setData(_text);
            },
          ),
        ),
        Text(_reference ? s['ef_points_help_ref'] : s['ef_points_help_err'], style: Theme.of(context).textTheme.bodySmall),
        TextFormField(
          key: const Key('ef-data'),
          initialValue: testPointsToText(_ef, s.lang),
          minLines: 5,
          maxLines: 20,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(errorText: _badLine == null ? null : '${s['pt_results_error']} $_badLine', alignLabelWithHint: true),
          onChanged: _setData,
        ),
      ],
    );
  }
}
