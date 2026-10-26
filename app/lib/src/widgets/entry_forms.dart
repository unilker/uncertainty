import 'package:flutter/material.dart';

import '../strings.dart';
import 'fields.dart';
import 'topdown_editor.dart' show JsonEntryCard;

/// Reference fields offered for each kind of bias evaluation.
const biasReferenceFields = <String, List<String>>{
  'crm': ['reference', 'expanded', 'k', 'u_reference_percent', 'mean', 'sd_percent', 'n', 'bias_percent'],
  'proficiency_tests': ['reference', 'mean', 'sd_reproducibility_percent', 'labs', 'u_reference_percent', 'bias_percent'],
  'recovery': ['recovery_percent'],
};

/// Fields offered for each kind of recovery test.
const recoveryTestFields = <String, List<String>>{
  'reference': ['mean', 'sd', 'n', 'reference', 'u_reference', 'expanded', 'k'],
  'spiked': ['mean_after', 'sd_after', 'n_after', 'mean_before', 'sd_before', 'n_before', 'spike', 'u_spike', 'u_spike_percent'],
  'recoveries': ['mean_percent', 'sd_percent', 'n'],
};

const _integerFields = {'n', 'labs', 'n_after', 'n_before', 'multiplicity'};

/// A form with a switch to the JSON text of the same entry. Editing the JSON
/// replaces the entry's contents in place, so the form shows them afterwards.
class _FormOrJson extends StatefulWidget {
  const _FormOrJson({required this.entry, required this.strings, required this.onChanged, required this.onRemove, required this.form, required this.header});
  final Map<String, dynamic> entry;
  final S strings;
  final VoidCallback onChanged;
  final VoidCallback onRemove;
  final Widget Function() form;
  final Widget header;

  @override
  State<_FormOrJson> createState() => _FormOrJsonState();
}

class _FormOrJsonState extends State<_FormOrJson> {
  bool _json = false;
  int _rev = 0;

  @override
  Widget build(BuildContext context) {
    final s = widget.strings;
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(child: widget.header),
                TextButton(
                  onPressed: () => setState(() {
                    _json = !_json;
                    _rev++;
                  }),
                  child: Text(_json ? s['form'] : 'JSON'),
                ),
                IconButton(tooltip: s['remove'], icon: const Icon(Icons.delete_outline), onPressed: widget.onRemove),
              ],
            ),
            const SizedBox(height: 8),
            KeyedSubtree(
              key: ValueKey(_rev),
              child: _json
                  ? JsonEntryCard(
                      value: widget.entry,
                      strings: s,
                      onChanged: (v) {
                        if (v is Map<String, dynamic>) {
                          widget.entry
                            ..clear()
                            ..addAll(v);
                          widget.onChanged();
                        }
                      },
                    )
                  : widget.form(),
            ),
          ],
        ),
      ),
    );
  }
}

/// In recovery tests `sd` is the standard deviation of the results, not a
/// standard uncertainty as for a normal distribution.
String _label(String f, S s, {bool data = false}) => data && f == 'sd' ? s.parameter('sd_data') : s.parameter(f);

Widget _numbers(Map<String, dynamic> map, List<String> fields, S s, VoidCallback onChanged, {bool data = false}) {
  return Wrap(
    spacing: 12,
    runSpacing: 8,
    children: [
      for (final f in fields)
        NumberField(
          map: map,
          field: f,
          label: _label(f, s, data: data),
          integer: _integerFields.contains(f),
          width: 170,
          onChanged: onChanged,
        ),
    ],
  );
}

Map<String, dynamic> _range(Map<String, dynamic> entry) => (entry['range'] ??= <String, dynamic>{}) as Map<String, dynamic>;

Widget _rangeFields(Map<String, dynamic> entry, S s, VoidCallback onChanged) {
  void clean() {
    if ((entry['range'] as Map?)?.isEmpty ?? false) entry.remove('range');
    onChanged();
  }

  return Wrap(
    spacing: 12,
    children: [
      NumberField(map: _range(entry), field: 'min', label: s['range_min'], width: 200, onChanged: clean),
      NumberField(map: _range(entry), field: 'max', label: s['range_max'], width: 200, onChanged: clean),
    ],
  );
}

/// One bias component: CRMs, proficiency tests or recovery experiments.
class BiasCard extends StatefulWidget {
  const BiasCard({super.key, required this.entry, required this.strings, required this.onChanged, required this.onRemove});
  final Map<String, dynamic> entry;
  final S strings;
  final VoidCallback onChanged;
  final VoidCallback onRemove;

  @override
  State<BiasCard> createState() => _BiasCardState();
}

class _BiasCardState extends State<BiasCard> {
  List<dynamic> get _refs => (widget.entry['references'] ??= <dynamic>[]) as List<dynamic>;

  void _edit(VoidCallback f) {
    setState(f);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = widget.strings;
    final e = widget.entry;
    final kind = (e['kind'] as String?) ?? 'crm';
    return _FormOrJson(
      entry: e,
      strings: s,
      onChanged: widget.onChanged,
      onRemove: widget.onRemove,
      header: Wrap(
        spacing: 12,
        runSpacing: 8,
        crossAxisAlignment: WrapCrossAlignment.center,
        children: [
          StringField(map: e, field: 'name', label: s['name'], width: 160, onChanged: widget.onChanged),
          SizedBox(
            width: 260,
            child: DropdownButtonFormField<String>(
              initialValue: biasReferenceFields.containsKey(kind) ? kind : null,
              isExpanded: true,
              decoration: InputDecoration(labelText: s['type'], isDense: true),
              items: [for (final k in biasReferenceFields.keys) DropdownMenuItem(value: k, child: Text(s['bias_$k']))],
              onChanged: (k) => _edit(() => e['kind'] = k),
            ),
          ),
          FilterChip(
            label: Text(e['relative'] == false ? s['absolute'] : s['relative']),
            selected: e['relative'] != false,
            onSelected: (v) => _edit(() => v ? e.remove('relative') : e['relative'] = false),
          ),
        ],
      ),
      form: () => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (var i = 0; i < _refs.length; i++)
            Padding(
              key: ObjectKey(_refs[i]),
              padding: const EdgeInsets.only(bottom: 12),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Expanded(
                    child: Wrap(
                      spacing: 12,
                      runSpacing: 8,
                      children: [
                        StringField(map: _refs[i] as Map<String, dynamic>, field: 'label', label: s['label'], width: 120, onChanged: widget.onChanged),
                        _numbers(_refs[i] as Map<String, dynamic>, biasReferenceFields[kind] ?? const [], s, widget.onChanged),
                        if (kind == 'crm')
                          SizedBox(
                            width: 260,
                            child: ListField(map: _refs[i] as Map<String, dynamic>, field: 'measured', label: s['measured'], onChanged: widget.onChanged),
                          ),
                      ],
                    ),
                  ),
                  IconButton(tooltip: s['remove'], icon: const Icon(Icons.delete_outline), onPressed: () => _edit(() => _refs.removeAt(i))),
                ],
              ),
            ),
          Wrap(
            spacing: 12,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              TextButton.icon(
                icon: const Icon(Icons.add),
                label: Text(s['add_reference']),
                onPressed: () => _edit(() => _refs.add(<String, dynamic>{'label': '${_refs.length + 1}'})),
              ),
              NumberField(map: e, field: 'u_reference_percent', label: s['shared_u_ref'], width: 230, onChanged: widget.onChanged),
            ],
          ),
          _rangeFields(e, s, widget.onChanged),
        ],
      ),
    );
  }
}

/// The recovery section: tests and the options of the significance test.
class RecoveryCard extends StatefulWidget {
  const RecoveryCard({super.key, required this.entry, required this.strings, required this.onChanged, required this.onRemove});
  final Map<String, dynamic> entry;
  final S strings;
  final VoidCallback onChanged;
  final VoidCallback onRemove;

  @override
  State<RecoveryCard> createState() => _RecoveryCardState();
}

class _RecoveryCardState extends State<RecoveryCard> {
  List<dynamic> get _tests => (widget.entry['tests'] ??= <dynamic>[]) as List<dynamic>;

  void _edit(VoidCallback f) {
    setState(f);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = widget.strings;
    final e = widget.entry;
    return _FormOrJson(
      entry: e,
      strings: s,
      onChanged: widget.onChanged,
      onRemove: widget.onRemove,
      header: Wrap(
        spacing: 12,
        runSpacing: 8,
        crossAxisAlignment: WrapCrossAlignment.center,
        children: [
          StringField(map: e, field: 'name', label: s['name'], width: 160, onChanged: widget.onChanged),
          SizedBox(
            width: 300,
            child: DropdownButtonFormField<String>(
              initialValue: (e['correction'] as String?) ?? 'if_significant',
              isExpanded: true,
              decoration: InputDecoration(labelText: s['correction'], isDense: true),
              items: [
                for (final c in ['if_significant', 'always', 'never']) DropdownMenuItem(value: c, child: Text(s['corr_$c'])),
              ],
              onChanged: (c) => _edit(() => c == 'if_significant' ? e.remove('correction') : e['correction'] = c),
            ),
          ),
          FilterChip(
            label: Text(s['include_sd']),
            selected: e['include_sd_of_recoveries'] == true,
            onSelected: (v) => _edit(() => v ? e['include_sd_of_recoveries'] = true : e.remove('include_sd_of_recoveries')),
          ),
        ],
      ),
      form: () => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          for (var i = 0; i < _tests.length; i++) _testRow(context, i),
          Wrap(
            spacing: 12,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              TextButton.icon(
                icon: const Icon(Icons.add),
                label: Text(s['add_test']),
                onPressed: () => _edit(() => _tests.add(<String, dynamic>{'type': 'reference', 'label': '${_tests.length + 1}'})),
              ),
              NumberField(map: e, field: 'confidence', label: s['confidence'], width: 150, onChanged: widget.onChanged),
              NumberField(map: e, field: 'dof', label: s['dof'], width: 150, onChanged: widget.onChanged),
            ],
          ),
          _rangeFields(e, s, widget.onChanged),
        ],
      ),
    );
  }

  Widget _testRow(BuildContext context, int i) {
    final s = widget.strings;
    final t = _tests[i] as Map<String, dynamic>;
    final type = (t['type'] as String?) ?? 'reference';
    return Padding(
      key: ObjectKey(t),
      padding: const EdgeInsets.only(bottom: 12),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Expanded(
            child: Wrap(
              spacing: 12,
              runSpacing: 8,
              children: [
                StringField(map: t, field: 'label', label: s['label'], width: 120, onChanged: widget.onChanged),
                SizedBox(
                  width: 320,
                  child: DropdownButtonFormField<String>(
                    initialValue: recoveryTestFields.containsKey(type) ? type : null,
                    isExpanded: true,
                    decoration: InputDecoration(labelText: s['type'], isDense: true),
                    items: [for (final k in recoveryTestFields.keys) DropdownMenuItem(value: k, child: Text(s['test_$k']))],
                    onChanged: (k) => _edit(() {
                      final label = t['label'];
                      t
                        ..clear()
                        ..['type'] = k;
                      if (label != null) t['label'] = label;
                    }),
                  ),
                ),
                KeyedSubtree(key: ValueKey(type), child: _numbers(t, recoveryTestFields[type] ?? const [], s, widget.onChanged, data: true)),
                if (type == 'reference' || type == 'recoveries')
                  SizedBox(
                    key: ValueKey('list-$type'),
                    width: 260,
                    child: ListField(
                      map: t,
                      field: type == 'reference' ? 'measured' : 'values_percent',
                      label: type == 'reference' ? s['measured'] : s['values_percent'],
                      onChanged: widget.onChanged,
                    ),
                  ),
              ],
            ),
          ),
          IconButton(tooltip: s['remove'], icon: const Icon(Icons.delete_outline), onPressed: () => _edit(() => _tests.removeAt(i))),
        ],
      ),
    );
  }
}

/// A further component of a top-down budget.
class ComponentCard extends StatefulWidget {
  const ComponentCard({super.key, required this.entry, required this.strings, required this.onChanged, required this.onRemove});
  final Map<String, dynamic> entry;
  final S strings;
  final VoidCallback onChanged;
  final VoidCallback onRemove;

  @override
  State<ComponentCard> createState() => _ComponentCardState();
}

class _ComponentCardState extends State<ComponentCard> {
  @override
  Widget build(BuildContext context) {
    final s = widget.strings;
    final e = widget.entry;
    return _FormOrJson(
      entry: e,
      strings: s,
      onChanged: widget.onChanged,
      onRemove: widget.onRemove,
      header: StringField(map: e, field: 'name', label: s['name'], width: 220, keepEmpty: true, onChanged: widget.onChanged),
      form: () => Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Wrap(
            spacing: 12,
            runSpacing: 8,
            children: [
              _numbers(e, const ['u', 'u_percent', 'half_width', 'half_width_percent', 'multiplicity'], s, widget.onChanged),
              SizedBox(
                width: 200,
                child: DropdownButtonFormField<String>(
                  initialValue: (e['distribution'] as String?) ?? 'rectangular',
                  decoration: InputDecoration(labelText: s['distribution'], isDense: true),
                  items: [
                    for (final d in ['rectangular', 'triangular']) DropdownMenuItem(value: d, child: Text(s.distribution(d))),
                  ],
                  onChanged: (d) {
                    setState(() => d == 'rectangular' ? e.remove('distribution') : e['distribution'] = d);
                    widget.onChanged();
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 8),
          _rangeFields(e, s, widget.onChanged),
        ],
      ),
    );
  }
}
