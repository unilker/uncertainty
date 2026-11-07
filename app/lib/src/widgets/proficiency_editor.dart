import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'fields.dart';

/// Formats the participant results as editable lines: code, result,
/// expanded uncertainty, coverage factor, method.
String resultsToText(List<dynamic> results, String lang) {
  String n(dynamic v) => v is num ? fmtGivenIn(v, lang) : '';
  return results
      .map((r) {
        final m = r as Map<String, dynamic>;
        final value = m['value'] != null
            ? n(m['value'])
            : m['less_than'] != null
            ? '<${n(m['less_than'])}'
            : '>${n(m['greater_than'])}';
        final cells = <String>[m['lab'].toString(), value];
        if (m['expanded'] != null) {
          cells.addAll([n(m['expanded']), n(m['k'] ?? 2)]);
        } else if (m['u'] != null) {
          cells.addAll([n((m['u'] as num) * 2), '2']);
        }
        if (m['method'] != null) cells.add(m['method'].toString());
        return cells.join('\t');
      })
      .join('\n');
}

/// Parses lines "code result [U [k]] [method]" (tab, semicolon or space
/// separated; decimal comma accepted). Returns null and the 1-based number
/// of the first line that cannot be read.
(List<Map<String, dynamic>>?, int?) parseResults(String text) {
  final out = <Map<String, dynamic>>[];
  var lineNo = 0;
  for (final line in text.split('\n')) {
    lineNo++;
    if (line.trim().isEmpty) continue;
    final cells = line.trim().split(RegExp(r'\t|;|\s+')).where((c) => c.isNotEmpty).toList();
    if (cells.length < 2) return (null, lineNo);
    final r = <String, dynamic>{'lab': cells[0]};
    final v = cells[1];
    if (v.startsWith('<') || v.startsWith('>')) {
      final x = parseNum(v.substring(1));
      if (x == null) return (null, lineNo);
      r[v.startsWith('<') ? 'less_than' : 'greater_than'] = x;
    } else {
      final x = parseNum(v);
      if (x == null) return (null, lineNo);
      r['value'] = x;
    }
    var i = 2;
    if (i < cells.length && parseNum(cells[i]) != null) {
      if (r['value'] != null) r['expanded'] = parseNum(cells[i]);
      i++;
      if (i < cells.length && parseNum(cells[i]) != null) {
        if (r['value'] != null) r['k'] = parseNum(cells[i]);
        i++;
      }
    }
    if (i < cells.length) r['method'] = cells.sublist(i).join(' ');
    out.add(r);
  }
  return (out, null);
}

/// Editor of a proficiency test round (ISO 13528): participant results
/// pasted from a spreadsheet, the assigned value, σ_pt and the optional
/// homogeneity and stability data.
class ProficiencyEditor extends StatefulWidget {
  const ProficiencyEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<ProficiencyEditor> createState() => _ProficiencyEditorState();
}

class _ProficiencyEditorState extends State<ProficiencyEditor> {
  Map<String, dynamic> get _pt => widget.project['proficiency'] as Map<String, dynamic>;
  int? _badLine;

  void _edit(VoidCallback f) {
    setState(f);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final assigned = (_pt['assigned_value'] ??= <String, dynamic>{'method': 'algorithm_a'}) as Map<String, dynamic>;
    final method = assigned['method'] as String? ?? 'algorithm_a';
    final sdPt = _pt['sd_pt'] as Map<String, dynamic>?;
    final sdMethod = sdPt?['method'] as String? ?? 'none';
    final hom = _pt['homogeneity'] as Map<String, dynamic>?;
    final stab = _pt['stability'] as Map<String, dynamic>?;
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          children: [
            StringField(map: _pt, field: 'quantity', label: s['quantity_name'], width: 200, keepEmpty: true, onChanged: widget.onChanged),
            StringField(map: _pt, field: 'unit', label: s['unit'], width: 120, onChanged: widget.onChanged),
          ],
        ),
        Section(s['pt_results']),
        TextFormField(
          key: const Key('pt-results'),
          initialValue: resultsToText((_pt['results'] as List?) ?? const [], s.lang),
          minLines: 6,
          maxLines: 20,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(
            helperText: s['pt_results_help'],
            helperMaxLines: 3,
            errorText: _badLine == null ? null : '${s['pt_results_error']} $_badLine',
            alignLabelWithHint: true,
          ),
          onChanged: (t) {
            final (rows, bad) = parseResults(t);
            setState(() => _badLine = bad);
            if (rows != null) {
              _pt['results'] = rows;
              widget.onChanged();
            }
          },
        ),
        Section(s['pt_assigned']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            SizedBox(
              width: 280,
              child: DropdownButtonFormField<String>(
                key: const Key('pt-assigned-method'),
                isExpanded: true,
                initialValue: method,
                items: [
                  for (final m in ['algorithm_a', 'median', 'mean', 'reference'])
                    DropdownMenuItem(
                      value: m,
                      child: Text(s['pt_$m'], overflow: TextOverflow.ellipsis),
                    ),
                ],
                onChanged: (v) => _edit(() {
                  assigned
                    ..clear()
                    ..['method'] = v;
                  if (v == 'reference') {
                    assigned['value'] = 1.0;
                    assigned['expanded'] = 0.01;
                    assigned['k'] = 2;
                  }
                }),
              ),
            ),
            if (method == 'reference') ...[
              NumberField(key: const ValueKey('av-value'), map: assigned, field: 'value', label: s['pt_value'], width: 140, onChanged: widget.onChanged),
              NumberField(key: const ValueKey('av-U'), map: assigned, field: 'expanded', label: s['pt_expanded'], width: 200, onChanged: widget.onChanged),
              NumberField(key: const ValueKey('av-k'), map: assigned, field: 'k', label: 'k', width: 80, onChanged: widget.onChanged),
            ],
          ],
        ),
        Section(s['pt_sd_pt']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            SizedBox(
              width: 280,
              child: DropdownButtonFormField<String>(
                isExpanded: true,
                initialValue: sdMethod,
                items: [
                  for (final m in ['none', 'robust', 'made', 'niqr', 'sd', 'given'])
                    DropdownMenuItem(
                      value: m,
                      child: Text(s['pt_$m'], overflow: TextOverflow.ellipsis),
                    ),
                ],
                onChanged: (v) => _edit(() {
                  if (v == 'none') {
                    _pt.remove('sd_pt');
                  } else {
                    _pt['sd_pt'] = <String, dynamic>{'method': v, if (v == 'given') 'percent': 10};
                  }
                }),
              ),
            ),
            if (sdMethod == 'given') ...[
              NumberField(key: const ValueKey('sd-value'), map: sdPt!, field: 'value', label: s['pt_value'], width: 140, onChanged: widget.onChanged),
              NumberField(key: const ValueKey('sd-percent'), map: sdPt, field: 'percent', label: s['pt_percent'], width: 140, onChanged: widget.onChanged),
            ],
          ],
        ),
        Section(s['pt_homogeneity']),
        TextFormField(
          initialValue: ((hom?['items'] as List?) ?? const []).map((r) => (r as List).map((e) => fmtGivenIn(e as num, s.lang)).join('\t')).join('\n'),
          minLines: 3,
          maxLines: 12,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(helperText: s['pt_homogeneity_help'], alignLabelWithHint: true),
          onChanged: (t) {
            final rows = parseRows(t, min: 2);
            if (t.trim().isEmpty) {
              _pt.remove('homogeneity');
            } else if (rows != null) {
              final h = (_pt['homogeneity'] ??= <String, dynamic>{}) as Map<String, dynamic>;
              h['items'] = rows;
            } else {
              return;
            }
            widget.onChanged();
          },
        ),
        if (hom != null)
          Padding(
            padding: const EdgeInsets.only(top: 8),
            child: Wrap(
              spacing: 12,
              runSpacing: 8,
              crossAxisAlignment: WrapCrossAlignment.end,
              children: [
                NumberField(map: hom, field: 'sd_pt_percent_of_mean', label: s['pt_hom_sd_percent'], width: 200, onChanged: widget.onChanged),
                SizedBox(
                  width: 300,
                  child: SwitchListTile(
                    key: const Key('pt-bayes'),
                    contentPadding: EdgeInsets.zero,
                    title: Text(s['pt_bayes_on']),
                    value: hom['bayes'] != null,
                    onChanged: (v) {
                      setState(() {
                        if (v) {
                          hom['bayes'] = <String, dynamic>{};
                        } else {
                          hom.remove('bayes');
                        }
                      });
                      widget.onChanged();
                    },
                  ),
                ),
                if (hom['bayes'] != null) ...[
                  NumberField(
                    key: const ValueKey('pt-tau-scale'),
                    map: hom['bayes'] as Map<String, dynamic>,
                    field: 'tau_scale',
                    label: s['pt_tau_scale'],
                    width: 220,
                    onChanged: widget.onChanged,
                  ),
                  NumberField(
                    key: const ValueKey('pt-sigma-scale'),
                    map: hom['bayes'] as Map<String, dynamic>,
                    field: 'sigma_scale',
                    label: s['pt_sigma_scale'],
                    width: 220,
                    onChanged: widget.onChanged,
                  ),
                ],
              ],
            ),
          ),
        if (hom?['bayes'] != null) Text(s['pt_bayes_help'], style: Theme.of(context).textTheme.bodySmall),
        Section(s['pt_stability']),
        TextFormField(
          initialValue: ((stab?['values'] as List?) ?? const []).map((e) => fmtGivenIn(e as num, s.lang)).join('\n'),
          minLines: 2,
          maxLines: 8,
          style: const TextStyle(fontFamily: 'monospace'),
          onChanged: (t) {
            final v = parseList(t);
            if (t.trim().isEmpty) {
              _pt.remove('stability');
            } else if (v != null && v.isNotEmpty) {
              final st = (_pt['stability'] ??= <String, dynamic>{}) as Map<String, dynamic>;
              st['values'] = v;
            } else {
              return;
            }
            widget.onChanged();
          },
        ),
      ],
    );
  }
}
