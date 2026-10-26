import 'dart:convert';

import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';
import '../strings.dart';
import 'entry_forms.dart';
import 'fields.dart';

/// Fields offered for each precision data type (besides the raw-data lists).
const precisionFields = <String, List<String>>{
  'series': [],
  'duplicates': [],
  'groups': [],
  'control_limits': ['limit', 'limit_percent', 'k'],
  'given': ['sd', 'sd_percent', 'dof'],
  'linear': ['intercept', 'slope', 'dilution_factor'],
  'continuous': ['sd0', 'rsd1_percent'],
};

/// Editor of a top-down project. Precision data are entered directly as raw
/// values, duplicate pairs or points; bias, recovery and further components
/// have forms, and every entry can also be edited as JSON (chapter 7 of the
/// user guide).
class TopDownEditor extends StatefulWidget {
  const TopDownEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<TopDownEditor> createState() => _TopDownEditorState();
}

class _TopDownEditorState extends State<TopDownEditor> {
  Map<String, dynamic> get _td => widget.project['topdown'] as Map<String, dynamic>;

  List<dynamic> _list(String key) => (_td[key] ??= <dynamic>[]) as List<dynamic>;

  void _edit(VoidCallback f) {
    setState(f);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final precision = _list('precision');
    final bias = _list('bias');
    final components = _list('components');
    final levels = (_td['levels'] as List?)?.map((e) => fmtGivenIn(e as num, s.lang)).join(' ') ?? '';
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        Wrap(spacing: 12, runSpacing: 8, children: [
          StringField(map: _td, field: 'quantity', label: s['quantity_name'], width: 200, keepEmpty: true, onChanged: widget.onChanged),
          StringField(map: _td, field: 'unit', label: s['unit'], width: 120, onChanged: widget.onChanged),
          NumberField(map: _td, field: 'coverage_factor', label: s['k'], width: 140, onChanged: widget.onChanged),
        ]),
        const SizedBox(height: 8),
        TextFormField(
          initialValue: levels,
          decoration: InputDecoration(labelText: s['levels'], helperText: s['levels_help'], isDense: true),
          onChanged: (t) {
            final v = parseList(t);
            if (v == null || v.isEmpty) {
              _td.remove('levels');
            } else {
              _td['levels'] = v;
            }
            widget.onChanged();
          },
        ),
        Section(s['precision'],
            trailing: FilledButton.tonalIcon(
              icon: const Icon(Icons.add),
              label: Text(s['add_precision']),
              onPressed: () => _edit(() => precision.add(<String, dynamic>{
                    'name': 'precision ${precision.length + 1}',
                    'data': <String, dynamic>{'type': 'series', 'values': <double>[], 'relative': true},
                  })),
            )),
        for (var i = 0; i < precision.length; i++)
          PrecisionCard(
            key: ObjectKey(precision[i]),
            entry: precision[i] as Map<String, dynamic>,
            strings: s,
            onChanged: widget.onChanged,
            onRemove: () => _edit(() => precision.removeAt(i)),
          ),
        Section(s['bias'],
            trailing: IconButton(
              tooltip: s['add'],
              icon: const Icon(Icons.add),
              onPressed: () => _edit(() => bias.add(<String, dynamic>{
                    'kind': 'crm',
                    'references': [
                      <String, dynamic>{'label': 'CRM', 'reference': 1.0, 'u_reference_percent': 1.0, 'measured': <double>[]}
                    ],
                  })),
            )),
        for (var i = 0; i < bias.length; i++)
          BiasCard(
            key: ObjectKey(bias[i]),
            entry: bias[i] as Map<String, dynamic>,
            strings: s,
            onChanged: widget.onChanged,
            onRemove: () => _edit(() => bias.removeAt(i)),
          ),
        Section(s['recovery'],
            trailing: _td['recovery'] == null
                ? IconButton(
                    tooltip: s['add'],
                    icon: const Icon(Icons.add),
                    onPressed: () => _edit(() => _td['recovery'] = <String, dynamic>{
                          'tests': [
                            <String, dynamic>{'type': 'reference', 'mean': 1.0, 'sd': 0.05, 'n': 10, 'reference': 1.0, 'u_reference': 0.01}
                          ],
                        }),
                  )
                : null),
        if (_td['recovery'] != null)
          RecoveryCard(
            key: ObjectKey(_td['recovery']),
            entry: _td['recovery'] as Map<String, dynamic>,
            strings: s,
            onChanged: widget.onChanged,
            onRemove: () => _edit(() => _td.remove('recovery')),
          ),
        Section(s['components'],
            trailing: IconButton(
              tooltip: s['add'],
              icon: const Icon(Icons.add),
              onPressed: () => _edit(() => components.add(<String, dynamic>{'name': 'component ${components.length + 1}', 'u_percent': 1.0})),
            )),
        for (var i = 0; i < components.length; i++)
          ComponentCard(
            key: ObjectKey(components[i]),
            entry: components[i] as Map<String, dynamic>,
            strings: s,
            onChanged: widget.onChanged,
            onRemove: () => _edit(() => components.removeAt(i)),
          ),
      ],
    );
  }
}

/// One precision component with direct entry of its raw data.
class PrecisionCard extends StatefulWidget {
  const PrecisionCard({super.key, required this.entry, required this.strings, required this.onChanged, required this.onRemove});
  final Map<String, dynamic> entry;
  final S strings;
  final VoidCallback onChanged;
  final VoidCallback onRemove;

  @override
  State<PrecisionCard> createState() => _PrecisionCardState();
}

class _PrecisionCardState extends State<PrecisionCard> {
  Map<String, dynamic> get _data => widget.entry['data'] as Map<String, dynamic>;

  Map<String, dynamic> get _range => (widget.entry['range'] ??= <String, dynamic>{}) as Map<String, dynamic>;

  void _setType(String type) {
    final next = <String, dynamic>{'type': type};
    if (type == 'series') next['values'] = <double>[];
    if (type == 'duplicates') next['pairs'] = <List<double>>[];
    if (type == 'groups') next['groups'] = <dynamic>[];
    if (type == 'series' || type == 'duplicates' || type == 'groups') next['relative'] = _data['relative'] ?? false;
    setState(() => widget.entry['data'] = next);
    widget.onChanged();
  }

  void _cleanRange() {
    if ((widget.entry['range'] as Map?)?.isEmpty ?? false) widget.entry.remove('range');
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = widget.strings;
    final type = _data['type'] as String? ?? 'given';
    final hasRelative = type == 'series' || type == 'duplicates' || type == 'groups';
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Row(children: [
            Expanded(
              child: Wrap(spacing: 12, runSpacing: 8, crossAxisAlignment: WrapCrossAlignment.center, children: [
                StringField(map: widget.entry, field: 'name', label: s['name'], width: 200, keepEmpty: true, onChanged: widget.onChanged),
                SizedBox(
                  width: 240,
                  child: DropdownButtonFormField<String>(
                    initialValue: precisionFields.containsKey(type) ? type : null,
                    isExpanded: true,
                    decoration: InputDecoration(labelText: s['data_type'], isDense: true),
                    items: [
                      for (final t in precisionFields.keys) DropdownMenuItem(value: t, child: Text(s.precisionType(t))),
                    ],
                    onChanged: (t) => t == null ? null : _setType(t),
                  ),
                ),
                if (hasRelative)
                  FilterChip(
                    label: Text(_data['relative'] == true ? s['relative'] : s['absolute']),
                    selected: _data['relative'] == true,
                    onSelected: (v) {
                      setState(() => _data['relative'] = v);
                      widget.onChanged();
                    },
                  ),
              ]),
            ),
            IconButton(tooltip: s['remove'], icon: const Icon(Icons.delete_outline), onPressed: widget.onRemove),
          ]),
          const SizedBox(height: 8),
          KeyedSubtree(
            key: ValueKey(type),
            child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              if (type == 'series') ListField(map: _data, field: 'values', label: s['values'], onChanged: widget.onChanged),
              if (type == 'duplicates') PairsField(map: _data, field: 'pairs', label: s['pairs'], onChanged: widget.onChanged),
              if (type == 'linear') PairsField(map: _data, field: 'points', label: s['points'], onChanged: widget.onChanged),
              if (type == 'groups')
                JsonEntryCard(
                  value: _data['groups'] ?? <dynamic>[],
                  strings: s,
                  onChanged: (v) {
                    _data['groups'] = v;
                    widget.onChanged();
                  },
                ),
              const SizedBox(height: 8),
              Wrap(spacing: 12, runSpacing: 8, children: [
                for (final p in precisionFields[type] ?? const <String>[])
                  NumberField(map: _data, field: p, label: s.parameter(p), width: 170, onChanged: widget.onChanged),
                NumberField(map: _range, field: 'min', label: s['range_min'], width: 200, onChanged: _cleanRange),
                NumberField(map: _range, field: 'max', label: s['range_max'], width: 200, onChanged: _cleanRange),
              ]),
            ]),
          ),
        ]),
      ),
    );
  }
}

/// A JSON entry edited as text; applied when it parses.
class JsonEntryCard extends StatefulWidget {
  const JsonEntryCard({super.key, required this.value, required this.strings, required this.onChanged, this.onRemove});
  final dynamic value;
  final S strings;
  final ValueChanged<dynamic> onChanged;
  final VoidCallback? onRemove;

  @override
  State<JsonEntryCard> createState() => _JsonEntryCardState();
}

class _JsonEntryCardState extends State<JsonEntryCard> {
  String? _error;

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Row(crossAxisAlignment: CrossAxisAlignment.start, children: [
          Expanded(
            child: TextFormField(
              initialValue: const JsonEncoder.withIndent('  ').convert(widget.value),
              minLines: 3,
              maxLines: 16,
              style: const TextStyle(fontFamily: 'monospace', fontSize: 13),
              decoration: InputDecoration(labelText: widget.strings['entry_json'], errorText: _error, alignLabelWithHint: true),
              onChanged: (t) {
                try {
                  final v = jsonDecode(t);
                  setState(() => _error = null);
                  widget.onChanged(v);
                } on FormatException catch (e) {
                  setState(() => _error = e.message);
                }
              },
            ),
          ),
          if (widget.onRemove != null)
            IconButton(tooltip: widget.strings['remove'], icon: const Icon(Icons.delete_outline), onPressed: widget.onRemove),
        ]),
      ),
    );
  }
}
