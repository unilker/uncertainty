import 'package:flutter/material.dart';

import '../app_state.dart';
import 'conformity_editor.dart';
import 'fields.dart';

/// GUM, Monte Carlo and reporting options of a project.
class SettingsEditor extends StatefulWidget {
  const SettingsEditor({super.key, required this.project, required this.onChanged});
  final Map<String, dynamic> project;
  final VoidCallback onChanged;

  @override
  State<SettingsEditor> createState() => _SettingsEditorState();
}

class _SettingsEditorState extends State<SettingsEditor> {
  Map<String, dynamic> get _analysis => (widget.project['analysis'] ??= <String, dynamic>{}) as Map<String, dynamic>;
  Map<String, dynamic> get _reporting => (widget.project['reporting'] ??= <String, dynamic>{}) as Map<String, dynamic>;

  void _edit(VoidCallback f) {
    setState(f);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    final isModel = widget.project.containsKey('model');
    final children = <Widget>[];
    if (isModel) {
      final gumOn = _analysis['gum'] != null || !_analysis.containsKey('gum');
      final mcmOn = _analysis['mcm'] != null || !_analysis.containsKey('mcm');
      children.addAll([
        Section(s['gum']),
        SwitchListTile(
          contentPadding: EdgeInsets.zero,
          title: Text(s['gum_on']),
          value: gumOn,
          onChanged: (v) => _edit(() => _analysis['gum'] = v ? <String, dynamic>{} : null),
        ),
      ]);
      if (gumOn) {
        final gum = (_analysis['gum'] ??= <String, dynamic>{}) as Map<String, dynamic>;
        final byFactor = gum.containsKey('coverage_factor');
        children.addAll([
          SegmentedButton<bool>(
            segments: [
              ButtonSegment(value: false, label: Text(s['by_probability'])),
              ButtonSegment(value: true, label: Text(s['by_factor'])),
            ],
            selected: {byFactor},
            onSelectionChanged: (v) => _edit(() {
              if (v.first) {
                gum.remove('coverage_probability');
                gum.remove('dominant_rectangular');
                gum['coverage_factor'] = 2;
              } else {
                gum.remove('coverage_factor');
                gum['coverage_probability'] = 0.95;
              }
            }),
          ),
          const SizedBox(height: 8),
          KeyedSubtree(
            key: ValueKey(byFactor),
            child: byFactor
                ? NumberField(map: gum, field: 'coverage_factor', label: s['k'], onChanged: widget.onChanged)
                : NumberField(map: gum, field: 'coverage_probability', label: s['p'], onChanged: widget.onChanged),
          ),
          if (!byFactor)
            SwitchListTile(
              key: const Key('dominant-rectangular'),
              contentPadding: EdgeInsets.zero,
              title: Text(s['dominant_rectangular']),
              subtitle: Text(s['dominant_rectangular_help']),
              value: gum['dominant_rectangular'] == true,
              onChanged: (v) => _edit(() => v ? gum['dominant_rectangular'] = true : gum.remove('dominant_rectangular')),
            ),
          SwitchListTile(
            contentPadding: EdgeInsets.zero,
            title: Text(s['higher_order']),
            value: gum['higher_order'] == true,
            onChanged: (v) => _edit(() => gum['higher_order'] = v),
          ),
          SwitchListTile(
            contentPadding: EdgeInsets.zero,
            title: Text(s['kragten']),
            value: gum['kragten'] == true,
            onChanged: (v) => _edit(() => v ? gum['kragten'] = true : gum.remove('kragten')),
          ),
        ]);
      }
      children.addAll([
        Section(s['mcm']),
        SwitchListTile(
          contentPadding: EdgeInsets.zero,
          title: Text(s['mcm_on']),
          value: mcmOn,
          onChanged: (v) => _edit(() => _analysis['mcm'] = v ? <String, dynamic>{'trials': 1000000} : null),
        ),
      ]);
      if (mcmOn) {
        final mcm = (_analysis['mcm'] ??= <String, dynamic>{}) as Map<String, dynamic>;
        final adaptive = mcm['adaptive'] != null;
        children.addAll([
          SwitchListTile(
            contentPadding: EdgeInsets.zero,
            title: Text(s['adaptive']),
            value: adaptive,
            onChanged: (v) => _edit(() {
              if (v) {
                mcm['adaptive'] = <String, dynamic>{'ndig': 2};
              } else {
                mcm.remove('adaptive');
              }
            }),
          ),
          Wrap(
            spacing: 12,
            runSpacing: 8,
            children: [
              if (!adaptive) NumberField(map: mcm, field: 'trials', label: s['trials'], integer: true, onChanged: widget.onChanged),
              if (adaptive)
                NumberField(map: mcm['adaptive'] as Map<String, dynamic>, field: 'ndig', label: 'n_dig', integer: true, onChanged: widget.onChanged),
              NumberField(map: mcm, field: 'seed', label: s['seed'], integer: true, onChanged: widget.onChanged),
              SizedBox(
                width: 260,
                child: DropdownButtonFormField<String>(
                  isExpanded: true,
                  initialValue: (mcm['interval'] as String?) ?? 'shortest',
                  decoration: InputDecoration(labelText: s['interval'], isDense: true),
                  items: [
                    DropdownMenuItem(value: 'shortest', child: Text(s['shortest'])),
                    DropdownMenuItem(value: 'symmetric', child: Text(s['symmetric'])),
                  ],
                  onChanged: (v) => _edit(() => mcm['interval'] = v),
                ),
              ),
            ],
          ),
        ]);
      }
    }
    children.addAll([
      Section(s['reporting']),
      Wrap(
        spacing: 12,
        runSpacing: 8,
        children: [
          NumberField(map: _reporting, field: 'significant_digits', label: s['sig_digits'], integer: true, onChanged: widget.onChanged),
          SizedBox(
            width: 260,
            child: DropdownButtonFormField<String>(
              isExpanded: true,
              initialValue: (_reporting['rounding'] as String?) ?? 'half_even',
              decoration: InputDecoration(labelText: s['rounding'], isDense: true),
              items: [
                for (final r in ['half_even', 'half_up', 'up']) DropdownMenuItem(value: r, child: Text(s[r])),
              ],
              onChanged: (v) => _edit(() => _reporting['rounding'] = v),
            ),
          ),
        ],
      ),
    ]);
    if (isModel) {
      final cf = widget.project['conformity'] as Map<String, dynamic>?;
      children.addAll([
        Section(s['cf_title']),
        SwitchListTile(
          key: const Key('cf-switch'),
          contentPadding: EdgeInsets.zero,
          title: Text(s['cf_on']),
          value: cf != null,
          onChanged: (v) => _edit(() {
            if (v) {
              widget.project['conformity'] = <String, dynamic>{};
            } else {
              widget.project.remove('conformity');
            }
          }),
        ),
        if (cf != null) ...[
          ConformityFields(map: cf, onChanged: widget.onChanged),
          const SizedBox(height: 8),
          SizedBox(
            width: 320,
            child: DropdownButtonFormField<String>(
              isExpanded: true,
              initialValue: (cf['distribution'] as String?) ?? 'normal',
              decoration: InputDecoration(labelText: s['cf_distribution'], isDense: true),
              items: [
                for (final d in ['normal', 't']) DropdownMenuItem(value: d, child: Text(s['cf_dist_$d'])),
              ],
              onChanged: (v) => _edit(() => cf['distribution'] = v),
            ),
          ),
        ],
      ]);
    }
    return ListView(padding: const EdgeInsets.fromLTRB(16, 8, 16, 96), children: children);
  }
}
