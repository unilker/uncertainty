import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'control_chart_editor.dart' show parseRuns;
import 'fields.dart';

/// Lines "name value u [ν]" of the participants' results. Returns the results
/// or null and the 1-based number of the first line that cannot be read.
(List<Map<String, dynamic>>?, int?) parseConsensusResults(String text) {
  final (parsed, bad) = parseRuns(text);
  if (parsed == null) return (null, bad);
  final (labels, rows) = parsed;
  final lineNumbers = [
    for (final (i, l) in text.split('\n').indexed)
      if (l.trim().isNotEmpty) i + 1,
  ];
  final out = <Map<String, dynamic>>[];
  for (var i = 0; i < rows.length; i++) {
    final r = rows[i];
    if (r.length < 2 || r.length > 3 || !(r[1] > 0) || (r.length == 3 && !(r[2] > 0))) return (null, lineNumbers[i]);
    out.add({'name': i < labels.length && labels[i].isNotEmpty ? labels[i] : 'Lab ${i + 1}', 'value': r[0], 'u': r[1], if (r.length == 3) 'dof': r[2]});
  }
  return (out, null);
}

String consensusResultsToText(Map<String, dynamic> cs, String lang) {
  final results = ((cs['results'] as List?) ?? const []).cast<Map<String, dynamic>>();
  return [
    for (final r in results)
      [
        r['name'].toString(),
        fmtGivenIn(r['value'] as num, lang),
        fmtGivenIn(r['u'] as num, lang),
        if (r['dof'] != null) fmtGivenIn(r['dof'] as num, lang),
      ].join('\t'),
  ].join('\n');
}

/// Editor of a consensus value (NIST Consensus Builder procedures): the
/// participants' results pasted from a spreadsheet and the procedures to
/// apply. Weights of the Linear Pool and priors other than the defaults are
/// edited in the JSON tab.
class ConsensusEditor extends StatefulWidget {
  const ConsensusEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<ConsensusEditor> createState() => _ConsensusEditorState();
}

class _ConsensusEditorState extends State<ConsensusEditor> {
  Map<String, dynamic> get _cs => widget.project['consensus'] as Map<String, dynamic>;
  int? _badLine;

  void _setData(String text) {
    final (parsed, bad) = parseConsensusResults(text);
    setState(() => _badLine = bad);
    if (parsed == null) return;
    // Keep the Linear Pool weights of participants that are still present.
    final old = {for (final r in ((_cs['results'] as List?) ?? const []).cast<Map<String, dynamic>>()) r['name']: r};
    for (final r in parsed) {
      final w = old[r['name']]?['weight'];
      if (w != null) r['weight'] = w;
    }
    _cs['results'] = parsed;
    widget.onChanged();
  }

  void _toggle(String key, bool on, {bool defaultOn = false}) {
    setState(() {
      if (on == defaultOn) {
        _cs.remove(key);
      } else {
        _cs[key] = on;
      }
    });
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final bayes = _cs['bayes'] as Map<String, dynamic>?;
    Widget sw(String key, String label, bool value, ValueChanged<bool> onChanged) => SizedBox(
      width: 300,
      child: SwitchListTile(key: Key(key), contentPadding: EdgeInsets.zero, title: Text(label), value: value, onChanged: onChanged),
    );
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            StringField(map: _cs, field: 'measurand', label: s['cs_measurand'], width: 240, onChanged: widget.onChanged),
            StringField(map: _cs, field: 'unit', label: s['unit'], width: 140, onChanged: widget.onChanged),
            NumberField(map: _cs, field: 'coverage', label: s['coverage'], width: 120, onChanged: widget.onChanged),
          ],
        ),
        Section(s['cs_procedures']),
        Wrap(
          spacing: 12,
          runSpacing: 0,
          children: [
            sw('cs-dl', s['cs_dl_on'], _cs['dersimonian_laird'] != false, (v) => _toggle('dersimonian_laird', v, defaultOn: true)),
            sw('cs-loo', s['cs_loo_on'], _cs['leave_one_out'] == true, (v) => _toggle('leave_one_out', v)),
            sw('cs-bilateral', s['cs_bilateral_on'], _cs['bilateral'] == true, (v) => _toggle('bilateral', v)),
            sw('cs-lp', s['cs_lp'], _cs['linear_pool'] == true, (v) => _toggle('linear_pool', v)),
            sw('cs-ml', s['cs_ml'], _cs['maximum_likelihood'] == true, (v) => _toggle('maximum_likelihood', v)),
            sw('cs-bayes', s['cs_bayes_on'], bayes != null, (v) {
              setState(() {
                if (v) {
                  _cs['bayes'] = <String, dynamic>{};
                } else {
                  _cs.remove('bayes');
                }
              });
              widget.onChanged();
            }),
            if (bayes != null)
              Labeled(
                label: s['cs_effects'],
                width: 300,
                child: DropdownButtonFormField<String>(
                  key: const Key('cs-effects'),
                  isExpanded: true,
                  initialValue: bayes['effects'] as String? ?? 'gaussian',
                  decoration: fieldDecoration(s['cs_effects']),
                  items: [
                    for (final e in const ['gaussian', 'laplace']) DropdownMenuItem(value: e, child: Text(s['cs_effects_$e'])),
                  ],
                  onChanged: (v) {
                    setState(() {
                      if (v == null || v == 'gaussian') {
                        bayes.remove('effects');
                      } else {
                        bayes['effects'] = v;
                      }
                    });
                    widget.onChanged();
                  },
                ),
              ),
          ],
        ),
        Section(s['cs_data']),
        Text(s['cs_data_help'], style: Theme.of(context).textTheme.bodySmall),
        TextFormField(
          key: const Key('cs-data'),
          initialValue: consensusResultsToText(_cs, s.lang),
          minLines: 5,
          maxLines: 30,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(errorText: _badLine == null ? null : '${s['pt_results_error']} $_badLine', alignLabelWithHint: true),
          onChanged: _setData,
        ),
      ],
    );
  }
}
