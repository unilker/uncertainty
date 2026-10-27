import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'fields.dart';

/// Formats named rows of numbers as editable lines: name, then the values of
/// [fields] (calibration points: x, u_x, y, u_y; samples: y, u_y).
String namedRowsToText(List<dynamic> rows, List<String> fields, String lang) => rows
    .map((r) {
      final m = r as Map<String, dynamic>;
      return [m['name'].toString(), for (final f in fields) m[f] is num ? fmtGivenIn(m[f] as num, lang) : ''].join('\t');
    })
    .join('\n');

/// Parses lines "name v₁ … vₖ" (tab, semicolon or space separated; decimal
/// comma accepted) into maps with the keys [fields]. A line of numbers only
/// is named [prefix] followed by its number. Returns null and the 1-based
/// number of the first line that cannot be read.
(List<Map<String, dynamic>>?, int?) parseNamedRows(String text, List<String> fields, String prefix) {
  final out = <Map<String, dynamic>>[];
  var lineNo = 0;
  for (final line in text.split('\n')) {
    lineNo++;
    if (line.trim().isEmpty) continue;
    var cells = line.trim().split(RegExp(r'\t|;|\s+')).where((c) => c.isNotEmpty).toList();
    final named = cells.length == fields.length + 1 && parseNum(cells[0]) == null;
    if (!named && cells.length != fields.length) return (null, lineNo);
    final name = named ? cells[0] : '$prefix${out.length + 1}';
    if (named) cells = cells.sublist(1);
    final r = <String, dynamic>{'name': name};
    for (var i = 0; i < fields.length; i++) {
      final v = parseNum(cells[i]);
      if (v == null) return (null, lineNo);
      r[fields[i]] = v;
    }
    out.add(r);
  }
  return (out, null);
}

const calibrationFunctions = ['linear', 'quadratic', 'cubic', 'power', 'exponential'];

/// Editor of a calibration with uncertainties in both variables (ISO 6143):
/// the calibration points, the analysis function and the responses of the
/// mixtures to be analysed.
class CalibrationEditor extends StatefulWidget {
  const CalibrationEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<CalibrationEditor> createState() => _CalibrationEditorState();
}

class _CalibrationEditorState extends State<CalibrationEditor> {
  Map<String, dynamic> get _cal => widget.project['calibration'] as Map<String, dynamic>;
  int? _badPoint;
  int? _badSample;
  int? _badCov;

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final cov = (_cal['covariances'] as List? ?? const []).cast<Map<String, dynamic>>();
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          children: [
            StringField(map: _cal, field: 'analyte', label: s['cal_analyte'], width: 200, keepEmpty: true, onChanged: widget.onChanged),
            StringField(map: _cal, field: 'unit', label: s['cal_unit'], width: 140, onChanged: widget.onChanged),
            StringField(map: _cal, field: 'response_unit', label: s['cal_response_unit'], width: 160, onChanged: widget.onChanged),
          ],
        ),
        Section(s['cal_points']),
        TextFormField(
          key: const Key('cal-points'),
          initialValue: namedRowsToText((_cal['points'] as List?) ?? const [], const ['x', 'u_x', 'y', 'u_y'], s.lang),
          minLines: 4,
          maxLines: 20,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(
            helperText: s['cal_points_help'],
            helperMaxLines: 3,
            errorText: _badPoint == null ? null : '${s['pt_results_error']} $_badPoint',
            alignLabelWithHint: true,
          ),
          onChanged: (t) {
            final (rows, bad) = parseNamedRows(t, const ['x', 'u_x', 'y', 'u_y'], 'M');
            setState(() => _badPoint = bad);
            if (rows != null) {
              _cal['points'] = rows;
              widget.onChanged();
            }
          },
        ),
        Section(s['cal_function']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.center,
          children: [
            SizedBox(
              width: 280,
              child: DropdownButtonFormField<String>(
                key: const Key('cal-function'),
                isExpanded: true,
                initialValue: _cal['function'] as String? ?? 'linear',
                items: [
                  for (final f in calibrationFunctions)
                    DropdownMenuItem(
                      value: f,
                      child: Text(s['cal_$f'], overflow: TextOverflow.ellipsis),
                    ),
                ],
                onChanged: (v) {
                  setState(() => _cal['function'] = v);
                  widget.onChanged();
                },
              ),
            ),
            NumberField(map: _cal, field: 'coverage_factor', label: 'k', width: 80, onChanged: widget.onChanged),
            NumberField(map: _cal, field: 'target_uncertainty', label: s['cal_target'], width: 220, onChanged: widget.onChanged),
          ],
        ),
        Section(s['cal_samples']),
        TextFormField(
          key: const Key('cal-samples'),
          initialValue: namedRowsToText((_cal['samples'] as List?) ?? const [], const ['y', 'u_y'], s.lang),
          minLines: 2,
          maxLines: 12,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(
            helperText: s['cal_samples_help'],
            errorText: _badSample == null ? null : '${s['pt_results_error']} $_badSample',
            alignLabelWithHint: true,
          ),
          onChanged: (t) {
            final (rows, bad) = parseNamedRows(t, const ['y', 'u_y'], 'S');
            setState(() => _badSample = bad);
            if (rows == null) return;
            if (rows.isEmpty) {
              _cal.remove('samples');
            } else {
              _cal['samples'] = rows;
            }
            widget.onChanged();
          },
        ),
        Section(s['cal_covariances']),
        TextFormField(
          initialValue: cov.map((c) => [...(c['between'] as List).map((e) => '$e'), fmtGivenIn(c['value'] as num, s.lang)].join('\t')).join('\n'),
          minLines: 1,
          maxLines: 8,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(
            helperText: s['cal_covariances_help'],
            helperMaxLines: 2,
            errorText: _badCov == null ? null : '${s['pt_results_error']} $_badCov',
            alignLabelWithHint: true,
          ),
          onChanged: (t) {
            final out = <Map<String, dynamic>>[];
            var lineNo = 0;
            for (final line in t.split('\n')) {
              lineNo++;
              if (line.trim().isEmpty) continue;
              final c = line.trim().split(RegExp(r'\t|;|\s+')).where((x) => x.isNotEmpty).toList();
              final v = c.length == 3 ? parseNum(c[2]) : null;
              if (v == null) {
                setState(() => _badCov = lineNo);
                return;
              }
              out.add({
                'between': [c[0], c[1]],
                'value': v,
              });
            }
            setState(() => _badCov = null);
            if (out.isEmpty) {
              _cal.remove('covariances');
            } else {
              _cal['covariances'] = out;
            }
            widget.onChanged();
          },
        ),
      ],
    );
  }
}
