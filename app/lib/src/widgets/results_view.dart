import 'package:flutter/material.dart';

import '../format.dart';
import '../strings.dart';
import 'charts.dart';
import 'fields.dart';

double? _d(dynamic v) => v is num ? v.toDouble() : (v == 'inf' ? double.infinity : null);

/// Results returned by the engine for a model or top-down project.
class ResultsView extends StatelessWidget {
  const ResultsView({super.key, required this.response, required this.stale, required this.strings});
  final Map<String, dynamic>? response;
  final bool stale;
  final S strings;

  @override
  Widget build(BuildContext context) {
    final s = strings;
    final r = response;
    if (r == null) return Center(child: Text(s['not_run']));
    final results = r['results'] as Map<String, dynamic>;
    final reported = (r['reported'] as List?)?.cast<String>() ?? const [];
    final unit = (results['output']?['unit'] as String?) ?? '';
    final children = <Widget>[
      if (stale) Text(s['unsaved'], style: TextStyle(color: Theme.of(context).colorScheme.error)),
      if (reported.isNotEmpty)
        Card(
          color: Theme.of(context).colorScheme.primaryContainer,
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
              Text(s['reported'], style: Theme.of(context).textTheme.titleMedium),
              const SizedBox(height: 8),
              for (final line in reported) SelectableText(line, key: const Key('reported')),
            ]),
          ),
        ),
    ];
    if (results['gum'] != null) children.addAll(_gum(context, results, unit));
    if (results['mcm'] != null) children.addAll(_mcm(context, results));
    if (results['topdown'] != null) children.addAll(_topdown(context, results['topdown'] as Map<String, dynamic>, unit));
    final checks = (results['checks'] as List?) ?? const [];
    if (checks.isNotEmpty) {
      children.add(Section(s['checks']));
      children.add(_table(context, ['', s['computed'], s['expected'], '±', ''], [
        for (final c in checks.cast<Map<String, dynamic>>())
          [
            c['what'].toString(),
            fmtNum(_d(c['computed']), 6, s.lang),
            fmtNum(_d(c['expected']), 6, s.lang),
            fmtNum(_d(c['tolerance']), 3, s.lang),
            c['passed'] == true ? s['passed'] : s['failed'],
          ]
      ]));
    }
    return ListView(padding: const EdgeInsets.fromLTRB(16, 8, 16, 96), children: children);
  }

  List<Widget> _gum(BuildContext context, Map<String, dynamic> results, String unit) {
    final s = strings;
    final g = results['gum'] as Map<String, dynamic>;
    final u = unit.isEmpty ? '' : ' $unit';
    final budget = (g['budget'] as List).cast<Map<String, dynamic>>();
    final warnings = (g['warnings'] as List?)?.cast<String>() ?? const [];
    return [
      Section(s['gum']),
      _table(context, ['', ''], [
        [s['estimate'], '${fmtNum(_d(g['estimate']), 6, s.lang)}$u'],
        [s['combined'], '${fmtNum(_d(g['standard_uncertainty']), 3, s.lang)}$u'],
        [s['eff_dof'], fmtNum(_d(g['effective_dof']), 3, s.lang)],
        [s['k'], fmtNum(_d(g['coverage_factor']), 3, s.lang)],
        [s['p'], fmtPct(_d(g['coverage_probability']), 4, s.lang)],
        [s['expanded'], '${fmtNum(_d(g['expanded_uncertainty']), 3, s.lang)}$u'],
      ]),
      Section(s['budget']),
      _table(context, [s['quantity'], s['estimate'], s['std_unc'], s['dof'], s['sensitivity'], s['contribution'], s['share']], [
        for (final b in budget)
          [
            b['name'].toString(),
            fmtNum(_d(b['estimate']), 6, s.lang),
            fmtNum(_d(b['standard_uncertainty']), 3, s.lang),
            fmtNum(_d(b['dof']), 3, s.lang),
            fmtNum(_d(b['sensitivity']), 3, s.lang),
            fmtNum(_d(b['contribution']), 3, s.lang),
            fmtPct(_d(b['variance_share']), 3, s.lang),
          ]
      ]),
      Section(s['contributions_chart']),
      SharesChart(rows: [for (final b in budget) MapEntry(b['name'].toString(), _d(b['variance_share']) ?? 0)], lang: s.lang),
      if (warnings.isNotEmpty) ...[
        Section(s['warnings']),
        for (final w in warnings) Text('• $w'),
      ],
    ];
  }

  List<Widget> _mcm(BuildContext context, Map<String, dynamic> results) {
    final s = strings;
    final m = results['mcm'] as Map<String, dynamic>;
    final g = results['gum'] as Map<String, dynamic>?;
    final v = results['validation'] as Map<String, dynamic>?;
    final shortest = (m['shortest_interval'] as List).map((e) => _d(e)!).toList();
    final symmetric = (m['symmetric_interval'] as List).map((e) => _d(e)!).toList();
    final h = m['histogram'] as Map<String, dynamic>?;
    final pdf = (results['gum_pdf'] as List?)?.map((p) => (p as List).map((e) => _d(e)!).toList()).toList();
    final pct = fmtPct(_d(m['coverage_probability']), 4, s.lang);
    return [
      Section(s['mcm']),
      _table(context, ['', ''], [
        [s['trials'], '${m['trials']} (${s['seed']}: ${m['seed']})'],
        [s['estimate'], fmtNum(_d(m['estimate']), 6, s.lang)],
        [s['std_unc'], fmtNum(_d(m['standard_uncertainty']), 3, s.lang)],
        ['${s['shortest']} ($pct)', '[${fmtNum(shortest[0], 6, s.lang)}; ${fmtNum(shortest[1], 6, s.lang)}]'],
        ['${s['symmetric']} ($pct)', '[${fmtNum(symmetric[0], 6, s.lang)}; ${fmtNum(symmetric[1], 6, s.lang)}]'],
        if (v != null)
          [
            s['validation'],
            '${v['validated'] == true ? s['validated'] : s['not_validated']} '
                '(n_dig = ${v['ndig']}, ${s['tolerance']} δ = ${fmtNum(_d(v['tolerance']), 3, s.lang)}; '
                'd_low = ${fmtNum(_d(v['d_low']), 3, s.lang)}, d_high = ${fmtNum(_d(v['d_high']), 3, s.lang)})',
          ],
      ]),
      if (h != null) ...[
        Section(s['histogram']),
        HistogramChart(
          lower: _d(h['lower'])!,
          binWidth: _d(h['bin_width'])!,
          counts: (h['counts'] as List).map((e) => (e as num).toInt()).toList(),
          trials: (m['trials'] as num).toInt(),
          gumPdf: pdf,
          mcmInterval: shortest,
          gumInterval: g == null ? null : (g['interval'] as List).map((e) => _d(e)!).toList(),
          lang: s.lang,
        ),
      ],
    ];
  }

  List<Widget> _topdown(BuildContext context, Map<String, dynamic> t, String unit) {
    final s = strings;
    final u = unit.isEmpty ? '' : ' $unit';
    final budget = (t['budget'] as List).cast<Map<String, dynamic>>();
    final levels = (t['levels'] as List).cast<Map<String, dynamic>>();
    String uOf(Map<String, dynamic> b) {
      final model = b['model'] as Map<String, dynamic>?;
      if (model != null) {
        return model['type'] == 'linear'
            ? '${fmtNum(_d(model['dilution_factor']), 3, s.lang)}·(${fmtNum(_d(model['intercept']), 4, s.lang)} + ${fmtNum(_d(model['slope']), 4, s.lang)}·c/F)'
            : '√(${fmtNum(_d(model['sd0']), 4, s.lang)}² + (c·${fmtPct(_d(model['rsd1']), 4, s.lang)})²)';
      }
      return b['relative'] == true ? fmtPct(_d(b['u']), 3, s.lang) : '${fmtNum(_d(b['u']), 3, s.lang)}$u';
    }

    String range(Map<String, dynamic> b) {
      final lo = _d(b['min']);
      final hi = _d(b['max']);
      if (lo == null && hi == null) return '';
      if (lo == null) return '< ${fmtGivenIn(hi, s.lang)}';
      if (hi == null) return '≥ ${fmtGivenIn(lo, s.lang)}';
      return '[${fmtGivenIn(lo, s.lang)}; ${fmtGivenIn(hi, s.lang)})';
    }

    final rec = t['recovery'] as Map<String, dynamic>?;
    final biases = (t['bias'] as List?)?.cast<Map<String, dynamic>>() ?? const [];
    final absolute = levels.any((l) => l['level'] != null);
    final k = fmtNum(_d(t['coverage_factor']), 3, s.lang);
    return [
      Section(s['budget']),
      _table(context, [s['name'], s['type'], s['std_unc'], '×', s['range']], [
        for (final b in budget) [b['name'].toString(), b['relative'] == true ? s['rel_kind'] : s['absolute'], uOf(b), '${b['multiplicity']}', range(b)]
      ]),
      for (final b in biases)
        Padding(
          padding: const EdgeInsets.only(top: 8),
          child: Text('${b['name']}: ${s['rms_bias']} ${b['relative'] == true ? fmtPct(_d(b['rms_bias']), 3, s.lang) : fmtNum(_d(b['rms_bias']), 3, s.lang)}, '
              '${s['u_ref']} ${b['relative'] == true ? fmtPct(_d(b['mean_u_reference']), 3, s.lang) : fmtNum(_d(b['mean_u_reference']), 3, s.lang)}'),
        ),
      if (rec != null)
        Padding(
          padding: const EdgeInsets.only(top: 8),
          child: Text('${s['mean_recovery']} ${fmtPct(_d(rec['mean']), 4, s.lang)}, u = ${fmtPct(_d(rec['u_used']), 3, s.lang)}; '
              '|1 − R̄|/u = ${fmtNum(_d(rec['statistic']), 3, s.lang)}, t = ${fmtNum(_d(rec['critical']), 3, s.lang)}: '
              '${rec['corrected'] == true ? s['significant'] : s['not_significant']}'),
        ),
      Section(s['results']),
      absolute
          ? _table(context, [s['level'], s['result'], 'u_c', 'U (k = $k)', s['rel_expanded']], [
              for (final l in levels)
                [
                  fmtGivenIn(_d(l['level']), s.lang),
                  fmtNum(_d(l['value']), 5, s.lang),
                  fmtNum(_d(l['standard_uncertainty']), 3, s.lang),
                  fmtNum(_d(l['expanded_uncertainty']), 3, s.lang),
                  fmtPct(_d(l['relative_expanded']), 3, s.lang),
                ]
            ])
          : _table(context, [s['rel_standard'], '${s['rel_expanded']} (k = $k)'], [
              for (final l in levels) [fmtPct(_d(l['relative_standard']), 3, s.lang), fmtPct(_d(l['relative_expanded']), 3, s.lang)]
            ]),
      for (final l in levels.take(4)) ...[
        Section(l['level'] == null ? s['contributions_chart'] : '${s['contributions_chart']} — ${fmtGivenIn(_d(l['level']), s.lang)}$u'),
        SharesChart(
          rows: [for (final sh in (l['shares'] as List).cast<Map<String, dynamic>>()) MapEntry(sh['name'].toString(), _d(sh['share']) ?? 0)],
          lang: s.lang,
        ),
      ],
    ];
  }

  Widget _table(BuildContext context, List<String> head, List<List<String>> rows) {
    final style = Theme.of(context).textTheme.bodyMedium?.copyWith(fontFeatures: const [FontFeature.tabularFigures()]);
    return SingleChildScrollView(
      scrollDirection: Axis.horizontal,
      child: DataTable(
        headingRowHeight: head.every((h) => h.isEmpty) ? 0 : 40,
        dataRowMinHeight: 30,
        dataRowMaxHeight: double.infinity,
        columns: [for (final h in head) DataColumn(label: Text(h))],
        rows: [
          for (final r in rows) DataRow(cells: [for (final c in r) DataCell(Text(c, style: style))]),
        ],
      ),
    );
  }
}
