import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'fields.dart';

const conformityRules = ['simple_acceptance', 'guarded_acceptance', 'guarded_rejection'];

/// Lines "name value [u]" (tab, semicolon or space separated; decimal comma
/// accepted). Returns null and the 1-based number of the first bad line.
(List<Map<String, dynamic>>?, int?) parseMeasured(String text) {
  final out = <Map<String, dynamic>>[];
  var lineNo = 0;
  for (final line in text.split('\n')) {
    lineNo++;
    if (line.trim().isEmpty) continue;
    final cells = line.trim().split(RegExp(r'\t|;')).map((c) => c.trim()).where((c) => c.isNotEmpty).toList();
    // Without tabs or semicolons the name may contain spaces: numbers are taken from the end.
    final parts = cells.length >= 2 ? cells : line.trim().split(RegExp(r'\s+'));
    final nums = <double>[];
    var i = parts.length;
    while (i > 1 && nums.length < 2 && parseNum(parts[i - 1]) != null) {
      nums.insert(0, parseNum(parts[i - 1])!);
      i--;
    }
    if (nums.isEmpty || i == 0) return (null, lineNo);
    out.add({'name': parts.sublist(0, i).join(' '), 'value': nums[0], if (nums.length > 1) 'u': nums[1]});
  }
  return (out, null);
}

String measuredToText(List<dynamic> rows, String lang) => rows
    .map((r) {
      final m = r as Map<String, dynamic>;
      return [m['name'].toString(), fmtGivenIn(m['value'] as num, lang), if (m['u'] is num) fmtGivenIn(m['u'] as num, lang)].join('\t');
    })
    .join('\n');

/// Tolerance limits and decision rule of a `conformity` section.
class ConformityFields extends StatefulWidget {
  const ConformityFields({super.key, required this.map, required this.onChanged});
  final Map<String, dynamic> map;
  final VoidCallback onChanged;

  @override
  State<ConformityFields> createState() => _ConformityFieldsState();
}

class _ConformityFieldsState extends State<ConformityFields> {
  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final m = widget.map;
    final rule = m['rule'] as String? ?? 'simple_acceptance';
    final byProbability = m.containsKey('probability');
    final tur = m['tur_method'] as String?;
    return Wrap(
      spacing: 12,
      runSpacing: 8,
      crossAxisAlignment: WrapCrossAlignment.end,
      children: [
        NumberField(key: const ValueKey('cf-lower'), map: m, field: 'lower', label: s['cf_lower'], width: 150, onChanged: widget.onChanged),
        NumberField(key: const ValueKey('cf-upper'), map: m, field: 'upper', label: s['cf_upper'], width: 150, onChanged: widget.onChanged),
        SizedBox(
          width: 260,
          child: DropdownButtonFormField<String>(
            key: const Key('cf-rule'),
            isExpanded: true,
            initialValue: rule,
            decoration: InputDecoration(labelText: s['cf_rule'], isDense: true),
            items: [
              for (final r in conformityRules)
                DropdownMenuItem(
                  value: r,
                  child: Text(s['cf_$r'], overflow: TextOverflow.ellipsis),
                ),
            ],
            onChanged: (v) {
              setState(() {
                m['rule'] = v;
                if (v == 'simple_acceptance') {
                  m.remove('r');
                  m.remove('probability');
                  m.remove('guard_band');
                }
              });
              widget.onChanged();
            },
          ),
        ),
        SizedBox(
          width: 220,
          child: DropdownButtonFormField<String>(
            key: const Key('cf-tur'),
            isExpanded: true,
            initialValue: tur ?? 'none',
            decoration: InputDecoration(labelText: s['cf_tur'], isDense: true),
            items: [
              for (final t in const ['none', 'rss', 'dobbert', 'rp10', 'test95']) DropdownMenuItem(value: t, child: Text(s['cf_tur_$t'])),
            ],
            onChanged: (v) {
              setState(() {
                if (v == null || v == 'none') {
                  m.remove('tur_method');
                  m.remove('tur_u95');
                  m.remove('tur_allow_factor_above_one');
                } else {
                  m['tur_method'] = v;
                  m['rule'] = 'simple_acceptance';
                  m.remove('r');
                  m.remove('probability');
                  m.remove('guard_band');
                  m.remove('target_consumer_risk');
                }
              });
              widget.onChanged();
            },
          ),
        ),
        if (tur != null)
          SizedBox(
            width: 220,
            child: DropdownButtonFormField<String>(
              key: const Key('cf-tur-u95'),
              isExpanded: true,
              initialValue: m['tur_u95'] as String? ?? 'k95',
              decoration: InputDecoration(labelText: s['cf_tur_u95'], isDense: true),
              items: [
                for (final t in const ['k95', 'two_u']) DropdownMenuItem(value: t, child: Text(s['cf_tur_u95_$t'])),
              ],
              onChanged: (v) {
                setState(() {
                  if (v == null || v == 'k95') {
                    m.remove('tur_u95');
                  } else {
                    m['tur_u95'] = v;
                  }
                });
                widget.onChanged();
              },
            ),
          ),
        if (rule != 'simple_acceptance' && tur == null) ...[
          SegmentedButton<bool>(
            segments: [
              ButtonSegment(value: false, label: Text(s['cf_by_r'])),
              ButtonSegment(value: true, label: Text(s['cf_by_probability'])),
            ],
            selected: {byProbability},
            onSelectionChanged: (v) {
              setState(() {
                if (v.first) {
                  m.remove('r');
                  m['probability'] = 0.95;
                } else {
                  m.remove('probability');
                  m['r'] = 1;
                }
              });
              widget.onChanged();
            },
          ),
          if (byProbability)
            NumberField(key: const ValueKey('cf-p'), map: m, field: 'probability', label: s['cf_probability'], width: 160, onChanged: widget.onChanged)
          else
            NumberField(key: const ValueKey('cf-r'), map: m, field: 'r', label: 'r (w = r·2u)', width: 140, onChanged: widget.onChanged),
        ],
      ],
    );
  }
}

/// Editor of a conformity project of its own (JCGM 106): property,
/// tolerance, decision rule, measuring system, measured values and the
/// optional production process for global risks.
class ConformityEditor extends StatefulWidget {
  const ConformityEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<ConformityEditor> createState() => _ConformityEditorState();
}

class _ConformityEditorState extends State<ConformityEditor> {
  Map<String, dynamic> get _cf => widget.project['conformity'] as Map<String, dynamic>;
  int? _bad;

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final process = _cf['process'] as Map<String, dynamic>?;
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          children: [
            StringField(map: _cf, field: 'quantity', label: s['quantity_name'], width: 200, onChanged: widget.onChanged),
            StringField(map: _cf, field: 'unit', label: s['unit'], width: 120, onChanged: widget.onChanged),
          ],
        ),
        Section(s['cf_tolerance']),
        ConformityFields(map: _cf, onChanged: widget.onChanged),
        Section(s['cf_system']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          children: [
            NumberField(map: _cf, field: 'u', label: s['cf_u'], width: 160, onChanged: widget.onChanged),
            NumberField(map: _cf, field: 'u_relative', label: s['cf_u_relative'], width: 200, onChanged: widget.onChanged),
            NumberField(map: _cf, field: 'dof', label: s['cf_dof'], width: 160, onChanged: widget.onChanged),
          ],
        ),
        Section(s['cf_results']),
        TextFormField(
          key: const Key('cf-results'),
          initialValue: measuredToText((_cf['results'] as List?) ?? const [], s.lang),
          minLines: 3,
          maxLines: 16,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(
            helperText: s['cf_results_help'],
            errorText: _bad == null ? null : '${s['pt_results_error']} $_bad',
            alignLabelWithHint: true,
          ),
          onChanged: (t) {
            final (rows, bad) = parseMeasured(t);
            setState(() => _bad = bad);
            if (rows == null) return;
            if (rows.isEmpty) {
              _cf.remove('results');
            } else {
              _cf['results'] = rows;
            }
            widget.onChanged();
          },
        ),
        Section(s['cf_process']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            SizedBox(
              width: 200,
              child: DropdownButtonFormField<String>(
                isExpanded: true,
                initialValue: process?['distribution'] as String? ?? 'none',
                items: [
                  for (final d in ['none', 'normal', 'gamma', 'lognormal']) DropdownMenuItem(value: d, child: Text(s['cf_process_$d'])),
                ],
                onChanged: (v) {
                  setState(() {
                    if (v == 'none') {
                      _cf.remove('process');
                      _cf.remove('target_consumer_risk');
                      _cf.remove('posterior');
                    } else {
                      final p = (_cf['process'] ??= <String, dynamic>{}) as Map<String, dynamic>;
                      p['distribution'] = v;
                      // A lognormal PDF is given by the mean and standard deviation of ln Y.
                      if (v == 'lognormal') {
                        p.remove('mean');
                        p.remove('sd');
                        p.putIfAbsent('meanlog', () => 0.0);
                        p.putIfAbsent('sdlog', () => 0.1);
                      } else {
                        p.remove('meanlog');
                        p.remove('sdlog');
                        p.putIfAbsent('mean', () => 1.0);
                        p.putIfAbsent('sd', () => 0.1);
                      }
                    }
                  });
                  widget.onChanged();
                },
              ),
            ),
            if (process != null) ...[
              if (process['distribution'] == 'lognormal') ...[
                NumberField(key: const ValueKey('cf-pml'), map: process, field: 'meanlog', label: s['cf_meanlog'], width: 140, onChanged: widget.onChanged),
                NumberField(key: const ValueKey('cf-psl'), map: process, field: 'sdlog', label: s['cf_sdlog'], width: 140, onChanged: widget.onChanged),
              ] else ...[
                NumberField(key: const ValueKey('cf-pm'), map: process, field: 'mean', label: s['pt_mean'], width: 140, onChanged: widget.onChanged),
                NumberField(key: const ValueKey('cf-ps'), map: process, field: 'sd', label: s['sd'], width: 140, onChanged: widget.onChanged),
              ],
              SizedBox(
                width: 320,
                child: SwitchListTile(
                  key: const Key('cf-posterior'),
                  contentPadding: EdgeInsets.zero,
                  title: Text(s['cf_posterior']),
                  value: _cf['posterior'] == true,
                  onChanged: (v) {
                    setState(() => v ? _cf['posterior'] = true : _cf.remove('posterior'));
                    widget.onChanged();
                  },
                ),
              ),
              NumberField(
                map: _cf,
                field: 'target_consumer_risk',
                label: s['cf_target'],
                width: 220,
                onChanged: () {
                  // The guard band then follows from the required risk.
                  if (_cf.containsKey('target_consumer_risk')) {
                    _cf.remove('r');
                    _cf.remove('probability');
                  }
                  widget.onChanged();
                },
              ),
            ],
          ],
        ),
      ],
    );
  }
}
