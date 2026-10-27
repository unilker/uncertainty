import 'package:flutter/material.dart';

import '../app_state.dart';
import '../strings.dart';
import 'fields.dart';

/// Parameters offered for each distribution type (the engine decides which
/// combinations are valid and reports the others).
const distributionParameters = <String, List<String>>{
  'normal': ['mean', 'sd', 'expanded', 'k'],
  't': ['mean', 'scale', 'expanded', 'k', 'dof'],
  'rectangular': ['value', 'half_width', 'lower', 'upper'],
  'triangular': ['value', 'half_width', 'lower', 'upper'],
  'arcsine': ['value', 'half_width', 'lower', 'upper'],
  'trapezoidal': ['value', 'half_width', 'lower', 'upper', 'beta'],
  'curvilinear_trapezoid': ['value', 'half_width', 'lower', 'upper', 'd'],
  'exponential': ['mean'],
  'gamma': ['shape', 'scale'],
  'type_a': [],
  'type_a_pooled': ['mean', 'pooled_sd', 'pooled_dof', 'n'],
  'constant': ['value'],
};

/// Editor of the measurement model: output, intermediates, inputs,
/// correlations. For a `multivariate` section (JCGM 102) it edits a list of
/// output quantities and whether the model is implicit.
class ModelEditor extends StatefulWidget {
  const ModelEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<ModelEditor> createState() => _ModelEditorState();
}

class _ModelEditorState extends State<ModelEditor> {
  bool get _multi => widget.project.containsKey('multivariate');
  Map<String, dynamic> get _model => widget.project[_multi ? 'multivariate' : 'model'] as Map<String, dynamic>;

  List<dynamic> _list(String key) => (_model[key] ??= <dynamic>[]) as List<dynamic>;

  void _edit(VoidCallback f) {
    setState(f);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final inputs = _list('inputs');
    final intermediates = _list('intermediates');
    final correlations = _list('correlations');
    final implicit = _model['implicit'] == true;
    return ListView(
      padding: const EdgeInsets.fromLTRB(16, 8, 16, 96),
      children: [
        if (!_multi) ...[
          Section(s['output']),
          Wrap(
            spacing: 12,
            runSpacing: 8,
            children: [
              StringField(
                map: _model['output'] as Map<String, dynamic>,
                field: 'name',
                label: s['name'],
                width: 160,
                keepEmpty: true,
                onChanged: widget.onChanged,
              ),
              StringField(map: _model['output'] as Map<String, dynamic>, field: 'unit', label: s['unit'], width: 120, onChanged: widget.onChanged),
            ],
          ),
          const SizedBox(height: 8),
          StringField(
            map: _model['output'] as Map<String, dynamic>,
            field: 'expression',
            label: s['expression'],
            keepEmpty: true,
            monospace: true,
            onChanged: widget.onChanged,
          ),
        ] else ...[
          Section(
            s['mv_outputs'],
            trailing: IconButton(
              key: const Key('add-output'),
              tooltip: s['add'],
              icon: const Icon(Icons.add),
              onPressed: () => _edit(() {
                final outs = _list('outputs');
                outs.add(<String, dynamic>{'name': 'Y${outs.length + 1}', 'expression': '0', if (implicit) 'start': 0.0});
              }),
            ),
          ),
          SwitchListTile(
            key: const Key('mv-implicit'),
            contentPadding: EdgeInsets.zero,
            title: Text(s['mv_implicit']),
            subtitle: Text(s['mv_implicit_help']),
            value: implicit,
            onChanged: (v) => _edit(() {
              if (v) {
                _model['implicit'] = true;
                for (final o in _list('outputs')) {
                  (o as Map<String, dynamic>).putIfAbsent('start', () => 0.0);
                }
              } else {
                _model.remove('implicit');
                for (final o in _list('outputs')) {
                  (o as Map<String, dynamic>).remove('start');
                }
              }
            }),
          ),
          for (var j = 0; j < _list('outputs').length; j++)
            Padding(
              key: ObjectKey(_list('outputs')[j]),
              padding: const EdgeInsets.only(bottom: 8),
              child: Wrap(
                spacing: 12,
                runSpacing: 8,
                crossAxisAlignment: WrapCrossAlignment.center,
                children: [
                  StringField(
                    map: _list('outputs')[j] as Map<String, dynamic>,
                    field: 'name',
                    label: s['name'],
                    width: 120,
                    keepEmpty: true,
                    onChanged: widget.onChanged,
                  ),
                  StringField(map: _list('outputs')[j] as Map<String, dynamic>, field: 'unit', label: s['unit'], width: 90, onChanged: widget.onChanged),
                  SizedBox(
                    width: 420,
                    child: StringField(
                      map: _list('outputs')[j] as Map<String, dynamic>,
                      field: 'expression',
                      label: implicit ? s['mv_equation'] : s['expression'],
                      keepEmpty: true,
                      monospace: true,
                      onChanged: widget.onChanged,
                    ),
                  ),
                  if (implicit)
                    NumberField(
                      map: _list('outputs')[j] as Map<String, dynamic>,
                      field: 'start',
                      label: s['mv_start'],
                      width: 110,
                      onChanged: widget.onChanged,
                    ),
                  IconButton(
                    tooltip: s['remove'],
                    icon: const Icon(Icons.delete_outline),
                    onPressed: _list('outputs').length <= 1 ? null : () => _edit(() => _list('outputs').removeAt(j)),
                  ),
                ],
              ),
            ),
          for (final g in (_model['indications'] as List?) ?? const [])
            Card(
              child: ListTile(
                leading: const Icon(Icons.table_chart_outlined),
                title: Text('${s['mv_indications']}: ${((g as Map)['inputs'] as List).join(', ')}'),
                subtitle: Text('n = ${(g['values'] as List).length}; ${s['mv_indications_help']}'),
              ),
            ),
        ],
        Section(
          s['intermediates'],
          trailing: IconButton(
            tooltip: s['add'],
            icon: const Icon(Icons.add),
            onPressed: () => _edit(() => intermediates.add(<String, dynamic>{'name': 'z${intermediates.length + 1}', 'expression': '0'})),
          ),
        ),
        for (var i = 0; i < intermediates.length; i++)
          Padding(
            key: ObjectKey(intermediates[i]),
            padding: const EdgeInsets.only(bottom: 8),
            child: Row(
              children: [
                StringField(
                  map: intermediates[i] as Map<String, dynamic>,
                  field: 'name',
                  label: s['name'],
                  width: 140,
                  keepEmpty: true,
                  onChanged: widget.onChanged,
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: StringField(
                    map: intermediates[i] as Map<String, dynamic>,
                    field: 'expression',
                    label: s['expression'],
                    keepEmpty: true,
                    monospace: true,
                    onChanged: widget.onChanged,
                  ),
                ),
                IconButton(tooltip: s['remove'], icon: const Icon(Icons.delete_outline), onPressed: () => _edit(() => intermediates.removeAt(i))),
              ],
            ),
          ),
        Section(
          s['inputs'],
          trailing: FilledButton.tonalIcon(
            key: const Key('add-input'),
            icon: const Icon(Icons.add),
            label: Text(s['add_input']),
            onPressed: () => _edit(
              () => inputs.add(<String, dynamic>{
                'name': 'x${inputs.length + 1}',
                'distribution': <String, dynamic>{'type': 'normal', 'mean': 0.0, 'sd': 1.0},
              }),
            ),
          ),
        ),
        for (var i = 0; i < inputs.length; i++)
          InputCard(
            key: ObjectKey(inputs[i]),
            input: inputs[i] as Map<String, dynamic>,
            strings: s,
            onChanged: widget.onChanged,
            onRemove: () => _edit(() => inputs.removeAt(i)),
          ),
        Section(
          s['correlations'],
          trailing: IconButton(
            tooltip: s['add'],
            icon: const Icon(Icons.add),
            onPressed: () => _edit(() => correlations.add(<String, dynamic>{'a': '', 'b': '', 'r': 0.0})),
          ),
        ),
        for (var i = 0; i < correlations.length; i++)
          Padding(
            key: ObjectKey(correlations[i]),
            padding: const EdgeInsets.only(bottom: 8),
            child: Wrap(
              spacing: 12,
              crossAxisAlignment: WrapCrossAlignment.center,
              children: [
                StringField(map: correlations[i] as Map<String, dynamic>, field: 'a', label: 'a', width: 120, keepEmpty: true, onChanged: widget.onChanged),
                StringField(map: correlations[i] as Map<String, dynamic>, field: 'b', label: 'b', width: 120, keepEmpty: true, onChanged: widget.onChanged),
                NumberField(map: correlations[i] as Map<String, dynamic>, field: 'r', label: 'r', width: 100, onChanged: widget.onChanged),
                IconButton(tooltip: s['remove'], icon: const Icon(Icons.delete_outline), onPressed: () => _edit(() => correlations.removeAt(i))),
              ],
            ),
          ),
      ],
    );
  }
}

/// One input quantity with its distribution.
class InputCard extends StatefulWidget {
  const InputCard({super.key, required this.input, required this.strings, required this.onChanged, required this.onRemove});
  final Map<String, dynamic> input;
  final S strings;
  final VoidCallback onChanged;
  final VoidCallback onRemove;

  @override
  State<InputCard> createState() => _InputCardState();
}

class _InputCardState extends State<InputCard> {
  Map<String, dynamic> get _dist => widget.input['distribution'] as Map<String, dynamic>;

  void _setType(String type) {
    final old = _dist;
    final central = old['mean'] ?? old['value'];
    final params = distributionParameters[type] ?? const [];
    final next = <String, dynamic>{'type': type};
    if (central != null) {
      if (params.contains('mean')) next['mean'] = central;
      if (params.contains('value')) next['value'] = central;
    }
    setState(() => widget.input['distribution'] = next);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = widget.strings;
    final type = _dist['type'] as String? ?? 'normal';
    final params = distributionParameters[type] ?? const [];
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(12),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Expanded(
                  child: Wrap(
                    spacing: 12,
                    runSpacing: 8,
                    crossAxisAlignment: WrapCrossAlignment.end,
                    children: [
                      StringField(map: widget.input, field: 'name', label: s['name'], width: 140, keepEmpty: true, onChanged: widget.onChanged),
                      StringField(map: widget.input, field: 'unit', label: s['unit'], width: 100, onChanged: widget.onChanged),
                      SizedBox(
                        width: 230,
                        child: DropdownButtonFormField<String>(
                          initialValue: distributionParameters.containsKey(type) ? type : null,
                          isExpanded: true,
                          decoration: InputDecoration(labelText: s['distribution'], isDense: true),
                          items: [for (final t in distributionParameters.keys) DropdownMenuItem(value: t, child: Text(s.distribution(t)))],
                          onChanged: (t) => t == null ? null : _setType(t),
                        ),
                      ),
                      NumberField(map: widget.input, field: 'dof', label: s['dof'], width: 140, onChanged: widget.onChanged),
                    ],
                  ),
                ),
                IconButton(tooltip: s['remove'], icon: const Icon(Icons.delete_outline), onPressed: widget.onRemove),
              ],
            ),
            const SizedBox(height: 8),
            KeyedSubtree(
              key: ValueKey(type),
              child: type == 'type_a'
                  ? ListField(map: _dist, field: 'observations', label: s['observations'], onChanged: widget.onChanged)
                  : Wrap(
                      spacing: 12,
                      runSpacing: 8,
                      children: [
                        for (final p in params)
                          NumberField(map: _dist, field: p, label: s.parameter(p), integer: p == 'n', width: 170, onChanged: widget.onChanged),
                      ],
                    ),
            ),
          ],
        ),
      ),
    );
  }
}
