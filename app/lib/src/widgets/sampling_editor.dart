import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'control_chart_editor.dart' show parseRuns;
import 'fields.dart';

const samplingMethods = ['anova', 'range'];

/// Lines "[label] S1A1 S1A2 S2A1 S2A2" of the balanced duplicate design.
/// Returns the labels (empty when no line has one) and the four values of
/// each target, or null and the 1-based number of the first line that
/// cannot be read.
((List<String>, List<List<double>>)?, int?) parseDuplicates(String text) {
  final (parsed, bad) = parseRuns(text);
  if (parsed == null) return (null, bad);
  final (labels, rows) = parsed;
  final lineNumbers = [
    for (final (i, l) in text.split('\n').indexed)
      if (l.trim().isNotEmpty) i + 1,
  ];
  for (var i = 0; i < rows.length; i++) {
    if (rows[i].length != 4) return (null, lineNumbers[i]);
  }
  return ((labels, rows), null);
}

String duplicatesToText(Map<String, dynamic> sp, String lang) {
  final labels = (sp['labels'] as List?)?.cast<String>() ?? const [];
  final rows = [for (final r in (sp['duplicates'] as List?) ?? const []) (r as List).cast<num>()];
  return [
    for (var i = 0; i < rows.length; i++) [if (i < labels.length && labels[i].isNotEmpty) labels[i], for (final v in rows[i]) fmtGivenIn(v, lang)].join('\t'),
  ].join('\n');
}

/// Editor of the duplicate method for the uncertainty from sampling
/// (Eurachem UfS:2019). Unbalanced designs and the modelling approach of
/// Gy's sampling theory are edited in the JSON tab.
class SamplingEditor extends StatefulWidget {
  const SamplingEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<SamplingEditor> createState() => _SamplingEditorState();
}

class _SamplingEditorState extends State<SamplingEditor> {
  Map<String, dynamic> get _sp => widget.project['sampling'] as Map<String, dynamic>;
  int? _badLine;

  void _setData(String text) {
    final (parsed, bad) = parseDuplicates(text);
    setState(() => _badLine = bad);
    if (parsed == null) return;
    final (labels, rows) = parsed;
    _sp['duplicates'] = rows;
    if (labels.isEmpty) {
      _sp.remove('labels');
    } else {
      _sp['labels'] = labels;
    }
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final duplicate = _sp['gy'] == null && (_sp['targets'] as List?)?.isNotEmpty != true;
    final method = _sp['method'] as String? ?? 'anova';
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            StringField(map: _sp, field: 'quantity', label: s['sp_quantity'], width: 320, onChanged: widget.onChanged),
            StringField(map: _sp, field: 'unit', label: s['unit'], width: 120, onChanged: widget.onChanged),
            NumberField(map: _sp, field: 'coverage_factor', label: s['k'], width: 120, onChanged: widget.onChanged),
          ],
        ),
        if (!duplicate) ...[const SizedBox(height: 16), Text(s['sp_json_only'], key: const Key('sp-json-only'))],
        if (duplicate) ...[
          Wrap(
            spacing: 12,
            runSpacing: 8,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              Labeled(
                label: s['sp_method'],
                width: 300,
                child: DropdownButtonFormField<String>(
                  key: const Key('sp-method'),
                  isExpanded: true,
                  initialValue: method,
                  decoration: fieldDecoration(s['sp_method']),
                  items: [for (final o in samplingMethods) DropdownMenuItem(value: o, child: Text(s['sp_$o']))],
                  onChanged: (v) {
                    if (v == null) return;
                    setState(() {
                      if (v == 'anova') {
                        _sp.remove('method');
                      } else {
                        _sp['method'] = v;
                      }
                    });
                    widget.onChanged();
                  },
                ),
              ),
              SizedBox(
                width: 300,
                child: SwitchListTile(
                  key: const Key('sp-log'),
                  contentPadding: EdgeInsets.zero,
                  title: Text(s['sp_log']),
                  value: _sp['log'] == true,
                  onChanged: (v) {
                    setState(() {
                      if (v) {
                        _sp['log'] = true;
                      } else {
                        _sp.remove('log');
                      }
                    });
                    widget.onChanged();
                  },
                ),
              ),
            ],
          ),
          Section(s['sp_data']),
          Text(s['sp_data_help'], style: Theme.of(context).textTheme.bodySmall),
          TextFormField(
            key: const Key('sp-data'),
            initialValue: duplicatesToText(_sp, s.lang),
            minLines: 5,
            maxLines: 20,
            style: const TextStyle(fontFamily: 'monospace'),
            decoration: InputDecoration(errorText: _badLine == null ? null : '${s['pt_results_error']} $_badLine', alignLabelWithHint: true),
            onChanged: _setData,
          ),
        ],
      ],
    );
  }
}
