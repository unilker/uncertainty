import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import 'fields.dart';

const chartKinds = ['x', 'range', 'relative_range'];
const limitBases = ['statistical', 'target', 'tolerance'];

/// Lines "[label] value [value …]" (tab, semicolon or space separated;
/// decimal comma accepted): one run per line, with an optional date or label
/// before the values. Returns the labels (empty when no line has one) and
/// the values of each run, or null and the 1-based number of the first line
/// that cannot be read.
((List<String>, List<List<double>>)?, int?) parseRuns(String text) {
  final labels = <String>[];
  final runs = <List<double>>[];
  var lineNo = 0;
  for (final line in text.split('\n')) {
    lineNo++;
    final t = line.trim();
    if (t.isEmpty) continue;
    final parts = (t.contains(RegExp(r'\t|;')) ? t.split(RegExp(r'\t|;')) : t.split(RegExp(r'\s+'))).map((c) => c.trim()).where((c) => c.isNotEmpty).toList();
    var label = '';
    if (parts.isNotEmpty && parseNum(parts.first) == null) label = parts.removeAt(0);
    final values = <double>[];
    for (final c in parts) {
      final v = parseNum(c);
      if (v == null) return (null, lineNo);
      values.add(v.toDouble());
    }
    if (values.isEmpty) return (null, lineNo);
    labels.add(label);
    runs.add(values);
  }
  return ((labels.every((l) => l.isEmpty) ? <String>[] : labels, runs), null);
}

String runsToText(Map<String, dynamic> cc, String lang) {
  final labels = (cc['labels'] as List?)?.cast<String>() ?? const [];
  final runs = cc['runs'] is List && (cc['runs'] as List).isNotEmpty
      ? [for (final r in cc['runs'] as List) (r as List).cast<num>()]
      : [
          for (final v in (cc['values'] as List?) ?? const []) <num>[v as num],
        ];
  return [
    for (var i = 0; i < runs.length; i++) [if (i < labels.length && labels[i].isNotEmpty) labels[i], for (final v in runs[i]) fmtGivenIn(v, lang)].join('\t'),
  ].join('\n');
}

/// Editor of a control chart (Nordtest TR 569): the control values or the
/// replicate results of each run, the central line and the basis of the
/// control limits.
class ControlChartEditor extends StatefulWidget {
  const ControlChartEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<ControlChartEditor> createState() => _ControlChartEditorState();
}

class _ControlChartEditorState extends State<ControlChartEditor> {
  Map<String, dynamic> get _cc => widget.project['control_chart'] as Map<String, dynamic>;
  Map<String, dynamic> get _limits => (_cc['limits'] ??= <String, dynamic>{}) as Map<String, dynamic>;
  int? _badLine;
  bool _badExclude = false;
  int _revision = 0;

  /// Fields of each basis; switching the basis drops the others.
  static const _fields = {
    'statistical': ['sd', 'sd_percent', 'sd_from'],
    'target': ['sd', 'sd_percent', 'expanded_percent', 'repeatability_limit'],
    'tolerance': ['tolerance', 'tolerance_percent'],
  };

  String get _center => _cc['central_line'] is Map ? ((_cc['central_line'] as Map).keys.first as String) : 'none';

  void _setData(String text) {
    final (parsed, bad) = parseRuns(text);
    setState(() => _badLine = bad);
    if (parsed == null) return;
    final (labels, runs) = parsed;
    _cc.remove('values');
    _cc.remove('runs');
    if (runs.every((r) => r.length == 1)) {
      if (runs.isNotEmpty) _cc['values'] = [for (final r in runs) r.first];
    } else {
      _cc['runs'] = runs;
    }
    if (labels.isEmpty) {
      _cc.remove('labels');
    } else {
      _cc['labels'] = labels;
    }
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final kind = _cc['chart'] as String? ?? 'x';
    final basis = _limits['basis'] as String? ?? 'statistical';
    final center = _center;
    final centerMap = _cc['central_line'] is Map ? (_cc['central_line'] as Map).cast<String, dynamic>() : <String, dynamic>{};
    Widget dropdown(
      String key,
      String label,
      String value,
      List<String> options,
      String Function(String) text,
      void Function(String) set, {
      double width = 260,
    }) => Labeled(
      label: label,
      width: width,
      child: DropdownButtonFormField<String>(
        key: Key(key),
        isExpanded: true,
        initialValue: value,
        decoration: fieldDecoration(label),
        items: [
          for (final o in options)
            DropdownMenuItem(
              value: o,
              child: Text(text(o), overflow: TextOverflow.ellipsis),
            ),
        ],
        onChanged: (v) {
          if (v == null) return;
          setState(() => set(v));
          widget.onChanged();
        },
      ),
    );
    final limitFields = <Widget>[
      if (basis == 'statistical') ...[
        NumberField(key: ValueKey('qc-sd-$basis-$_revision'), map: _limits, field: 'sd', label: s['qc_sd_given'], width: 240, onChanged: widget.onChanged),
        NumberField(
          key: ValueKey('qc-sdp-$basis-$_revision'),
          map: _limits,
          field: 'sd_percent',
          label: s['qc_sd_percent'],
          width: 220,
          onChanged: widget.onChanged,
        ),
        if (kind != 'x')
          dropdown('qc-sd-from', s['qc_sd_from'], _limits['sd_from'] as String? ?? 'mean_range', ['mean_range', 'pooled'], (o) => s['qc_$o'], (v) {
            if (v == 'mean_range') {
              _limits.remove('sd_from');
            } else {
              _limits['sd_from'] = v;
            }
          }),
      ],
      if (basis == 'target') ...[
        NumberField(key: ValueKey('qc-sd-$basis-$_revision'), map: _limits, field: 'sd', label: s['qc_sd_target'], width: 200, onChanged: widget.onChanged),
        NumberField(
          key: ValueKey('qc-sdp-$basis-$_revision'),
          map: _limits,
          field: 'sd_percent',
          label: s['qc_sd_percent'],
          width: 220,
          onChanged: widget.onChanged,
        ),
        if (kind == 'x')
          NumberField(
            key: ValueKey('qc-up-$_revision'),
            map: _limits,
            field: 'expanded_percent',
            label: s['qc_expanded_percent'],
            width: 220,
            onChanged: widget.onChanged,
          )
        else
          NumberField(
            key: ValueKey('qc-rl-$_revision'),
            map: _limits,
            field: 'repeatability_limit',
            label: s['qc_repeatability_limit'],
            width: 260,
            onChanged: widget.onChanged,
          ),
      ],
      if (basis == 'tolerance') ...[
        NumberField(key: ValueKey('qc-tol-$_revision'), map: _limits, field: 'tolerance', label: s['qc_tol'], width: 160, onChanged: widget.onChanged),
        NumberField(
          key: ValueKey('qc-tolp-$_revision'),
          map: _limits,
          field: 'tolerance_percent',
          label: s['qc_tol_percent'],
          width: 160,
          onChanged: widget.onChanged,
        ),
      ],
    ];
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            StringField(map: _cc, field: 'quantity', label: s['qc_sample'], width: 280, onChanged: widget.onChanged),
            StringField(map: _cc, field: 'unit', label: s['unit'], width: 120, onChanged: widget.onChanged),
            dropdown('qc-chart', s['qc_chart'], kind, chartKinds, (o) => s['qc_$o'], (v) {
              if (v == 'x') {
                _cc.remove('chart');
              } else {
                _cc['chart'] = v;
              }
              if (v == 'x') _limits.remove('sd_from');
              _revision++;
            }),
            if (kind != 'x') NumberField(map: _cc, field: 'replicates', label: s['qc_replicates'], integer: true, width: 200, onChanged: widget.onChanged),
          ],
        ),
        Section(s['qc_data']),
        Text(s['qc_data_help'], style: Theme.of(context).textTheme.bodySmall),
        TextFormField(
          key: const Key('qc-data'),
          initialValue: runsToText(_cc, s.lang),
          minLines: 5,
          maxLines: 20,
          style: const TextStyle(fontFamily: 'monospace'),
          decoration: InputDecoration(errorText: _badLine == null ? null : '${s['pt_results_error']} $_badLine', alignLabelWithHint: true),
          onChanged: _setData,
        ),
        Section(s['qc_center']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            dropdown('qc-center', s['qc_center'], center, ['none', 'mean', 'reference'], (o) => s['qc_center_$o'], (v) {
              final value = centerMap.values.isEmpty ? null : centerMap.values.first;
              if (v == 'none') {
                _cc.remove('central_line');
              } else {
                _cc['central_line'] = <String, dynamic>{v: value ?? 0};
                if (v == 'reference') _cc.remove('reference');
              }
              _revision++;
            }),
            if (center != 'none')
              NumberField(
                key: ValueKey('qc-cl-$_revision'),
                map: centerMap,
                field: center,
                label: s['qc_center_value'],
                width: 160,
                onChanged: () {
                  if (centerMap[center] is num) _cc['central_line'] = <String, dynamic>{center: centerMap[center]};
                  widget.onChanged();
                },
              ),
            if (center != 'reference' && kind == 'x')
              NumberField(key: ValueKey('qc-ref-$_revision'), map: _cc, field: 'reference', label: s['qc_reference'], width: 260, onChanged: widget.onChanged),
          ],
        ),
        Section(s['qc_basis']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            dropdown('qc-basis', s['qc_basis'], basis, limitBases, (o) => s['qc_$o'], (v) {
              for (final f in _fields.values.expand((x) => x)) {
                if (!_fields[v]!.contains(f)) _limits.remove(f);
              }
              if (v == 'statistical') {
                _limits.remove('basis');
              } else {
                _limits['basis'] = v;
              }
              if (v == 'tolerance') {
                _limits.remove('action_only');
                _limits.remove('action_factor');
              }
              _revision++;
            }),
            ...limitFields,
          ],
        ),
        if (basis != 'tolerance')
          Wrap(
            spacing: 12,
            runSpacing: 8,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              SizedBox(
                width: 280,
                child: SwitchListTile(
                  key: const Key('qc-action-only'),
                  contentPadding: EdgeInsets.zero,
                  title: Text(s['qc_action_only']),
                  value: _limits['action_only'] == true,
                  onChanged: (v) {
                    setState(() {
                      if (v) {
                        _limits['action_only'] = true;
                      } else {
                        _limits.remove('action_only');
                        _limits.remove('action_factor');
                      }
                    });
                    widget.onChanged();
                  },
                ),
              ),
              if (_limits['action_only'] == true)
                NumberField(
                  key: ValueKey('qc-af-$_revision'),
                  map: _limits,
                  field: 'action_factor',
                  label: s['qc_action_factor'],
                  width: 220,
                  onChanged: widget.onChanged,
                ),
            ],
          ),
        Section(s['qc_period']),
        Wrap(
          spacing: 12,
          runSpacing: 8,
          crossAxisAlignment: WrapCrossAlignment.end,
          children: [
            NumberField(map: _cc, field: 'establish', label: s['qc_establish'], integer: true, width: 260, onChanged: widget.onChanged),
            Labeled(
              label: s['qc_exclude'],
              width: 260,
              child: TextFormField(
                key: const Key('qc-exclude'),
                initialValue: ((_cc['exclude'] as List?) ?? const []).join(', '),
                decoration: fieldDecoration(s['qc_exclude'], helperText: s['qc_exclude_help'], errorText: _badExclude ? s['pt_results_error'] : null),
                onChanged: (t) {
                  final parts = t.split(RegExp(r'[,;\s]+')).where((x) => x.isNotEmpty).toList();
                  final runs = [for (final p in parts) int.tryParse(p)];
                  setState(() => _badExclude = runs.any((r) => r == null || r < 1));
                  if (_badExclude) return;
                  if (runs.isEmpty) {
                    _cc.remove('exclude');
                  } else {
                    _cc['exclude'] = runs;
                  }
                  widget.onChanged();
                },
              ),
            ),
          ],
        ),
        SwitchListTile(
          key: const Key('qc-review'),
          contentPadding: EdgeInsets.zero,
          title: Text(s['qc_review']),
          subtitle: Text(s['qc_review_help']),
          value: _cc['review'] is Map,
          onChanged: (v) {
            setState(() {
              if (v) {
                _cc['review'] = <String, dynamic>{};
              } else {
                _cc.remove('review');
              }
            });
            widget.onChanged();
          },
        ),
      ],
    );
  }
}
