import 'dart:math' as math;

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
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(s['reported'], style: Theme.of(context).textTheme.titleMedium),
                const SizedBox(height: 8),
                for (final (i, line) in reported.indexed) SelectableText(line, key: i == 0 ? const Key('reported') : null),
              ],
            ),
          ),
        ),
    ];
    final warnings = (r['warnings'] as List?)?.cast<String>() ?? const [];
    if (results['gum'] != null) children.addAll(_gum(context, results, unit, warnings));
    if (results['mcm'] != null) children.addAll(_mcm(context, results));
    if (results['topdown'] != null) children.addAll(_topdown(context, results['topdown'] as Map<String, dynamic>, unit));
    if (results['proficiency'] != null) {
      children.addAll(_proficiency(context, results['proficiency'] as Map<String, dynamic>, unit));
    }
    if (results['calibration'] != null) {
      children.addAll(_calibration(context, results['calibration'] as Map<String, dynamic>, unit));
    }
    if (results['conformity'] != null) {
      children.addAll(_conformity(context, results['conformity'] as Map<String, dynamic>));
    }
    if (results['multivariate'] != null) {
      children.addAll(_multivariate(context, results['multivariate'] as Map<String, dynamic>));
    }
    if (results['precision_experiment'] != null) {
      children.addAll(_precision(context, results['precision_experiment'] as Map<String, dynamic>, unit));
    }
    if (results['control_chart'] != null) {
      children.addAll(_controlChart(context, results['control_chart'] as Map<String, dynamic>, unit));
    }
    if (results['multicomponent_conformity'] != null) {
      children.addAll(_multicomponent(context, results['multicomponent_conformity'] as Map<String, dynamic>));
    }
    if (results['consensus'] != null) {
      children.addAll(_consensus(context, results['consensus'] as Map<String, dynamic>, unit));
    }
    if (results['error_function'] != null) {
      children.addAll(_errorFunction(context, results['error_function'] as Map<String, dynamic>));
    }
    if (results['sampling'] != null) {
      children.addAll(_sampling(context, results['sampling'] as Map<String, dynamic>, unit));
    }
    final checks = (results['checks'] as List?) ?? const [];
    if (checks.isNotEmpty) {
      children.add(Section(s['checks']));
      children.add(
        _table(
          context,
          ['', s['computed'], s['expected'], '±', ''],
          [
            for (final c in checks.cast<Map<String, dynamic>>())
              [
                c['what'].toString(),
                fmtNum(_d(c['computed']), 6, s.lang),
                fmtNum(_d(c['expected']), 6, s.lang),
                fmtNum(_d(c['tolerance']), 3, s.lang),
                c['passed'] == true ? s['passed'] : s['failed'],
              ],
          ],
        ),
      );
    }
    return ListView(key: const Key('results-list'), padding: const EdgeInsets.fromLTRB(16, 8, 16, 96), children: children);
  }

  List<Widget> _gum(BuildContext context, Map<String, dynamic> results, String unit, List<String> warnings) {
    final s = strings;
    final g = results['gum'] as Map<String, dynamic>;
    final u = unit.isEmpty ? '' : ' $unit';
    final budget = (g['budget'] as List).cast<Map<String, dynamic>>();
    final kragten = g['kragten'] as Map<String, dynamic>?;
    final kc = (kragten?['contributions'] as List?)?.map(_d).toList();
    return [
      if (warnings.isNotEmpty)
        Card(
          color: Theme.of(context).colorScheme.tertiaryContainer,
          child: Padding(
            padding: const EdgeInsets.all(12),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text(s['warnings'], style: Theme.of(context).textTheme.titleSmall),
                for (final w in warnings) Text('• $w', key: const Key('warning')),
              ],
            ),
          ),
        ),
      Section(s['gum']),
      _table(
        context,
        ['', ''],
        [
          [s['estimate'], '${fmtEstimate(_d(g['estimate']), _d(g['standard_uncertainty']), s.lang)}$u'],
          [s['combined'], '${fmtNum(_d(g['standard_uncertainty']), 3, s.lang)}$u'],
          [s['eff_dof'], fmtNum(_d(g['effective_dof']), 3, s.lang)],
          [s['k'], fmtNum(_d(g['coverage_factor']), 3, s.lang)],
          [s['p'], fmtPct(_d(g['coverage_probability']), 4, s.lang)],
          [s['expanded'], '${fmtNum(_d(g['expanded_uncertainty']), 3, s.lang)}$u'],
          if (kragten != null) ['Kragten u_c', '${fmtNum(_d(kragten['combined_standard_uncertainty']), 3, s.lang)}$u'],
        ],
      ),
      Section(s['budget']),
      _table(
        context,
        [s['quantity'], s['estimate'], s['std_unc'], s['dof'], s['sensitivity'], s['contribution'], s['share'], if (kc != null) 'Kragten Δᵢ'],
        [
          for (final (i, b) in budget.indexed)
            [
              b['name'].toString(),
              fmtEstimate(_d(b['estimate']), _d(b['standard_uncertainty']), s.lang),
              fmtNum(_d(b['standard_uncertainty']), 3, s.lang),
              fmtNum(_d(b['dof']), 3, s.lang),
              fmtNum(_d(b['sensitivity']), 3, s.lang),
              fmtNum(_d(b['contribution']), 3, s.lang),
              fmtPct(_d(b['variance_share']), 3, s.lang),
              if (kc != null) fmtNum(kc[i], 3, s.lang),
            ],
        ],
      ),
      Section(s['contributions_chart']),
      SharesChart(rows: [for (final b in budget) MapEntry(b['name'].toString(), _d(b['variance_share']) ?? 0)], lang: s.lang),
    ];
  }

  List<Widget> _consensus(BuildContext context, Map<String, dynamic> r, String rawUnit) {
    final s = strings;
    final unit = rawUnit.isEmpty ? '' : ' $rawUnit';
    final dl = r['dersimonian_laird'] as Map<String, dynamic>;
    final b = dl['bootstrap'] as Map<String, dynamic>?;
    final lp = r['linear_pool'] as Map<String, dynamic>?;
    final ml = r['maximum_likelihood'] as Map<String, dynamic>?;
    final by = r['bayes'] as Map<String, dynamic>?;
    String n(dynamic x, [int sig = 7]) => fmtNum(_d(x), sig, s.lang);
    String interval(Map<String, dynamic> m) {
      final i = (m['interval'] as List).map((e) => _d(e)).toList();
      return '[${n(i[0])}, ${n(i[1])}]$unit (${fmtPct(_d(m['coverage']), 3, s.lang)})';
    }

    final set = r['degrees_of_equivalence'] as Map<String, dynamic>?;
    final loo = set?['version'] == 'leave_one_out';
    final doe = ((loo ? set!['unilateral'] : b?['degrees_of_equivalence']) as List?)?.cast<Map<String, dynamic>>() ?? const [];
    final bil = (set?['bilateral'] as List?)?.cast<Map<String, dynamic>>() ?? const [];
    final sig = bil.where((x) => x['significant'] == true).toList();
    final dl0 = loo ? 'D*' : 'D';
    final results = (r['results'] as List).cast<Map<String, dynamic>>();
    return [
      Section(s['cs_title']),
      _table(
        context,
        ['', ''],
        [
          [s['cs_consensus'], '${n(dl['mean'])}$unit'],
          [s['cs_u_dl'], '${n(dl['standard_uncertainty'], 3)}$unit'],
          [s['cs_tau'], '${n(dl['tau'], 3)}$unit'],
          ['Cochran Q', '${n(dl['q'], 4)} (p = ${n(dl['q_p_value'], 2)})'],
          [s['cs_kh'], '${n(dl['u_knapp_hartung'], 3)}$unit'],
          if (b != null) [s['cs_boot_u'], '${n(b['standard_uncertainty'], 3)}$unit'],
          if (b != null) [s['cs_interval'], interval(b)],
          if (lp != null) [s['cs_lp'], '${n(lp['mean'])}$unit, u = ${n(lp['standard_uncertainty'], 3)}$unit'],
          if (lp != null) [s['cs_interval'], interval(lp)],
          if (by != null) [s['cs_bayes'], '${n(by['mean'])}$unit, u = ${n(by['standard_uncertainty'], 3)}$unit'],
          if (by != null) [s['cs_interval'], interval(by)],
          if (by != null) [s['cs_tau_median'], '${n(by['tau_median'], 3)}$unit'],
          if (ml != null) [s['cs_ml'], '${n(ml['mean'])}$unit, u = ${n(ml['standard_uncertainty'], 3)}$unit'],
          if (ml != null) [s['cs_tau'], '${n(ml['tau'], 3)}$unit, u = ${n(ml['u_tau'], 3)}$unit'],
        ],
      ),
      if (doe.isNotEmpty) Section(loo ? s['cs_doe_loo'] : s['cs_doe']),
      if (doe.isNotEmpty)
        _table(
          context,
          [s['cs_participant'], s['cs_weight'], dl0, 'U95 %($dl0)'],
          [
            for (var j = 0; j < doe.length; j++)
              [doe[j]['name'].toString(), fmtPct(_d(results[j]['weight']), 3, s.lang), n(doe[j]['d'], 3), n(doe[j]['expanded'], 3)],
          ],
        ),
      if (bil.isNotEmpty) Section(s['cs_bilateral']),
      if (bil.isNotEmpty)
        Text('${s['cs_bilateral_sig']} ${sig.length} / ${bil.length}', key: const Key('cs-bilateral-count')),
      if (sig.isNotEmpty)
        _table(
          context,
          ['A', 'B', 'B_AB$unit', 'U95 %$unit'],
          [
            for (final x in sig) [x['a'].toString(), x['b'].toString(), n(x['d'], 3), n(x['expanded'], 3)],
          ],
        ),
      ..._doeByProcedure(context, r, unit, dl0),
    ];
  }

  /// D (U95 %) of DerSimonian–Laird, the Linear Pool and Bayes side by side,
  /// when the Linear Pool or Bayes degrees of equivalence were computed.
  List<Widget> _doeByProcedure(BuildContext context, Map<String, dynamic> r, String unit, String dl0) {
    final s = strings;
    String n(dynamic x) => fmtNum(_d(x), 3, s.lang);
    List<Map<String, dynamic>>? uni(String key) =>
        ((r[key] as Map<String, dynamic>?)?['unilateral'] as List?)?.cast<Map<String, dynamic>>();
    final dlSet = r['degrees_of_equivalence'] as Map<String, dynamic>?;
    final dl = uni('degrees_of_equivalence') ??
        ((r['dersimonian_laird'] as Map<String, dynamic>)['bootstrap'] as Map<String, dynamic>?)?['degrees_of_equivalence']
            ?.cast<Map<String, dynamic>>();
    final lp = uni('linear_pool_degrees_of_equivalence');
    final by = uni('bayes_degrees_of_equivalence');
    if (lp == null && by == null) return const [];
    final cols = <(String, List<Map<String, dynamic>>)>[
      if (dl != null) ('DerSimonian–Laird', dl),
      if (lp != null) (s['cs_lp'], lp),
      if (by != null) ('Bayes', by),
    ];
    final names = cols.first.$2.map((e) => e['name'].toString()).toList();
    String sigText(String key, String label) {
      final b = ((r[key] as Map<String, dynamic>?)?['bilateral'] as List?)?.cast<Map<String, dynamic>>();
      if (b == null || b.isEmpty) return '';
      return '$label ${b.where((x) => x['significant'] == true).length}/${b.length}';
    }
    final counts = [
      if (dlSet != null) sigText('degrees_of_equivalence', 'DerSimonian–Laird'),
      sigText('linear_pool_degrees_of_equivalence', s['cs_lp']),
      sigText('bayes_degrees_of_equivalence', 'Bayes'),
    ].where((t) => t.isNotEmpty).join('; ');
    return [
      Section(s['cs_by_procedure']),
      _table(
        context,
        [s['cs_participant'], for (final c in cols) '${c.$1}: $dl0 (U95 %)$unit'],
        [
          for (var j = 0; j < names.length; j++)
            [names[j], for (final c in cols) '${n(c.$2[j]['d'])} (${n(c.$2[j]['expanded'])})'],
        ],
      ),
      if (counts.isNotEmpty) Text('${s['cs_bilateral_sig']} $counts', key: const Key('cs-bilateral-by-procedure')),
    ];
  }

  List<Widget> _multicomponent(BuildContext context, Map<String, dynamic> r) {
    final s = strings;
    final names = (r['components'] as List).join(', ');
    final rows = <List<String>>[
      for (final x in (r['results'] as List).cast<Map<String, dynamic>>())
        [
          x['name'].toString(),
          '${x['accepted'] == true ? s['cf_accepted'] : s['cf_rejected']}; '
              '${x['accepted'] == true ? s['mc_consumer'] : s['mc_producer']} ${fmtPct(_d(x['risk']), 3, s.lang)}',
        ],
    ];
    final g = r['global'] as Map<String, dynamic>?;
    return [
      Section('${s['mc_title']} ($names)'),
      _table(
        context,
        ['', ''],
        [
          ...rows,
          if (g != null) [s['mc_global_consumer'], fmtPct(_d(g['consumer_risk']), 3, s.lang)],
          if (g != null) [s['mc_global_producer'], fmtPct(_d(g['producer_risk']), 3, s.lang)],
        ],
      ),
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
    final bd = results['bounded'] as Map<String, dynamic>?;
    final bm = bd?['mcm'] as Map<String, dynamic>?;
    final gi = (bd?['gum_interval'] as List?)?.map((e) => _d(e)!).toList();
    return [
      if (bd != null && bd['gum_truncated'] == true && gi != null) ...[
        Section(s['bounded']),
        Text('${s['bounded_gum']}: [${fmtNum(gi[0], 6, s.lang)}; ${fmtNum(gi[1], 6, s.lang)}]', key: const Key('bounded-gum')),
      ],
      if (bm != null)
        _table(
          context,
          [s['bounded_mcm'], ''],
          [
            [s['bounded_fraction'], fmtPct(_d(bm['fraction']), 4, s.lang)],
            [s['estimate'], fmtEstimate(_d(bm['estimate']), _d(bm['standard_uncertainty']), s.lang)],
            [s['std_unc'], fmtNum(_d(bm['standard_uncertainty']), 3, s.lang)],
            [
              '${s['shortest']} ($pct)',
              '[${fmtNum(_d((bm['shortest_interval'] as List)[0]), 6, s.lang)}; ${fmtNum(_d((bm['shortest_interval'] as List)[1]), 6, s.lang)}]',
            ],
          ],
        ),
      Section(s['mcm']),
      _table(
        context,
        ['', ''],
        [
          [s['trials'], '${m['trials']} (${s['seed']}: ${m['seed']})'],
          [s['estimate'], fmtEstimate(_d(m['estimate']), _d(m['standard_uncertainty']), s.lang)],
          [s['std_unc'], fmtNum(_d(m['standard_uncertainty']), 3, s.lang)],
          ['${s['shortest']} ($pct)', '[${fmtNum(shortest[0], 6, s.lang)}; ${fmtNum(shortest[1], 6, s.lang)}]'],
          ['${s['symmetric']} ($pct)', '[${fmtNum(symmetric[0], 6, s.lang)}; ${fmtNum(symmetric[1], 6, s.lang)}]'],
          if (m['expanded_about_estimate'] != null) ['${s['about_estimate']} ($pct)', 'U = ${fmtNum(_d(m['expanded_about_estimate']), 3, s.lang)}'],
          if (v != null)
            [
              s['validation'],
              '${v['validated'] == true ? s['validated'] : s['not_validated']} '
                  '(n_dig = ${v['ndig']}, ${s['tolerance']} δ = ${fmtNum(_d(v['tolerance']), 3, s.lang)}; '
                  'd_low = ${fmtNum(_d(v['d_low']), 3, s.lang)}, d_high = ${fmtNum(_d(v['d_high']), 3, s.lang)})',
            ],
        ],
      ),
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
      _table(
        context,
        [s['name'], s['type'], s['std_unc'], '×', s['range']],
        [
          for (final b in budget) [b['name'].toString(), b['relative'] == true ? s['rel_kind'] : s['absolute'], uOf(b), '${b['multiplicity']}', range(b)],
        ],
      ),
      for (final b in biases)
        Padding(
          padding: const EdgeInsets.only(top: 8),
          child: Text(
            '${b['name']}: ${s['rms_bias']} ${b['relative'] == true ? fmtPct(_d(b['rms_bias']), 3, s.lang) : fmtNum(_d(b['rms_bias']), 3, s.lang)}, '
            '${s['u_ref']} ${b['relative'] == true ? fmtPct(_d(b['mean_u_reference']), 3, s.lang) : fmtNum(_d(b['mean_u_reference']), 3, s.lang)}',
          ),
        ),
      if (rec != null)
        Padding(
          padding: const EdgeInsets.only(top: 8),
          child: Text(
            '${s['mean_recovery']} ${fmtPct(_d(rec['mean']), 4, s.lang)}, u = ${fmtPct(_d(rec['u_used']), 3, s.lang)}; '
            '|1 − R̄|/u = ${fmtNum(_d(rec['statistic']), 3, s.lang)}, t = ${fmtNum(_d(rec['critical']), 3, s.lang)}: '
            '${rec['corrected'] == true ? s['significant'] : s['not_significant']}',
          ),
        ),
      Section(s['results']),
      absolute
          ? _table(
              context,
              [s['level'], s['result'], 'u_c', 'U (k = $k)', s['rel_expanded']],
              [
                for (final l in levels)
                  [
                    fmtGivenIn(_d(l['level']), s.lang),
                    fmtNum(_d(l['value']), 5, s.lang),
                    fmtNum(_d(l['standard_uncertainty']), 3, s.lang),
                    fmtNum(_d(l['expanded_uncertainty']), 3, s.lang),
                    fmtPct(_d(l['relative_expanded']), 3, s.lang),
                  ],
              ],
            )
          : _table(
              context,
              [s['rel_standard'], '${s['rel_expanded']} (k = $k)'],
              [
                for (final l in levels) [fmtPct(_d(l['relative_standard']), 3, s.lang), fmtPct(_d(l['relative_expanded']), 3, s.lang)],
              ],
            ),
      for (final l in levels.take(4)) ...[
        Section(l['level'] == null ? s['contributions_chart'] : '${s['contributions_chart']} — ${fmtGivenIn(_d(l['level']), s.lang)}$u'),
        SharesChart(
          rows: [for (final sh in (l['shares'] as List).cast<Map<String, dynamic>>()) MapEntry(sh['name'].toString(), _d(sh['share']) ?? 0)],
          lang: s.lang,
        ),
      ],
    ];
  }

  List<Widget> _proficiency(BuildContext context, Map<String, dynamic> r, String unit) {
    final s = strings;
    final u = unit.isEmpty ? '' : ' $unit';
    String n(dynamic v, [int sig = 4]) => v == null ? '—' : fmtNum(_d(v), sig, s.lang);
    String z(dynamic v) {
      final d = _d(v);
      if (d == null) return '—';
      final t = d.toStringAsFixed(2);
      return s.lang == 'tr' ? t.replaceAll('.', ',') : t;
    }

    final out = <Widget>[];
    final sm = r['summary'] as Map<String, dynamic>?;
    if (sm != null) {
      final robust = sm['robust'] as Map<String, dynamic>?;
      out.addAll([
        Section(s['pt_summary']),
        _table(
          context,
          ['', ''],
          [
            [s['pt_p'], '${sm['p']}'],
            [s['pt_median'], '${n(sm['median'])}$u'],
            ['MADe', '${n(sm['made'])}$u'],
            ['nIQR', '${n(sm['niqr'])}$u'],
            if (robust != null) [s['pt_robust_mean'], '${n(robust['mean'])}$u'],
            if (robust != null) [s['pt_robust_sd'], '${n(robust['sd'])}$u (${robust['iterations']} ${s['pt_iterations']})'],
            [s['pt_mean'], '${n(sm['mean'])}$u'],
            [s['sd'], '${n(sm['sd'])}$u'],
          ],
        ),
      ]);
    }
    final a = r['assigned'] as Map<String, dynamic>?;
    if (a != null) {
      out.addAll([
        Section(s['pt_assigned']),
        _table(
          context,
          ['', ''],
          [
            ['x_pt', '${n(a['value'])}$u'],
            [s['pt_u_assigned'], '${n(a['u'], 3)}$u'],
            ['U(x_pt) (k = ${n(r['coverage_factor'], 3)})', '${n(a['expanded'], 3)}$u'],
            if (r['sd_pt'] != null) ['σ_pt', '${n(r['sd_pt'], 3)}$u'],
            if (r['delta_e'] != null) ['δ_E', '${n(r['delta_e'], 3)}$u'],
          ],
        ),
        if (r['u_negligible'] != null) Text(r['u_negligible'] == true ? s['pt_negligible'] : s['pt_not_negligible']),
      ]);
    }
    final parts = (r['participants'] as List? ?? const []).cast<Map<String, dynamic>>();
    if (parts.isNotEmpty) {
      final hasU = parts.any((p) => p['u'] != null);
      String signal(Map<String, dynamic> p) => switch (p['z_signal']) {
        'acceptable' => s['pt_acceptable'],
        'warning' => s['pt_warning'],
        'action' => s['pt_action'],
        _ => p['censored'] != null ? s['pt_censored'] : '—',
      };
      out.addAll([
        Section(s['pt_scores']),
        _table(
          context,
          [s['pt_lab'], s['result'], if (hasU) 'u(x)', 'D %', 'z', 'z′', if (hasU) 'ζ', if (hasU) 'E_n', s['pt_signal']],
          [
            for (final p in parts)
              [
                p['lab'].toString(),
                p['value'] != null ? fmtGivenIn(_d(p['value']), s.lang) : (p['censored'] ?? '—').toString(),
                if (hasU) n(p['u'], 2),
                p['d_percent'] == null ? '—' : fmtNum(_d(p['d_percent']), 3, s.lang),
                z(p['z']),
                z(p['z_prime']),
                if (hasU) z(p['zeta']),
                if (hasU) '${z(p['en'])}${p['en_satisfactory'] == false ? ' ⚠' : ''}',
                signal(p),
              ],
          ],
        ),
      ]);
      final zs = [
        for (final p in parts)
          if (_d(p['z']) != null) MapEntry(p['lab'].toString(), _d(p['z'])!),
      ];
      if (zs.isNotEmpty) out.addAll([Section(s['pt_z_chart']), ZScoreChart(rows: zs, lang: s.lang)]);
    }
    final h = r['homogeneity'] as Map<String, dynamic>?;
    if (h != null) {
      out.addAll([
        Section(s['pt_hom_result']),
        _table(
          context,
          ['', ''],
          [
            ['g × m', '${h['items']} × ${h['replicates']}'],
            ['x̄', '${n(h['mean'], 5)}$u'],
            ['s_x', '${n(h['s_x'], 3)}$u'],
            ['s_w', '${n(h['s_w'], 3)}$u'],
            ['s_s', '${n(h['s_s'], 3)}$u'],
            if (h['u_bb_star'] != null) ['u*_bb', '${n(h['u_bb_star'], 3)}$u'],
            if (h['criterion'] != null)
              [s['pt_criterion'], '${n(h['criterion'], 3)}$u — ${h['sufficient'] == true ? s['pt_sufficient'] : s['pt_not_sufficient']}'],
          ],
        ),
      ]);
      final b = r['between_unit'] as Map<String, dynamic>?;
      if (b != null) {
        String row(Map<String, dynamic> p) {
          final hpd = p['hpd'] as List;
          return '${n(p['mean'], 3)}$u (${s['pt_median']} ${n(p['median'], 3)}, ${s['pt_mode']} ${n(p['mode'], 3)}); '
              '95 %: [${n(hpd[0], 3)}, ${n(hpd[1], 3)}]$u';
        }

        out.addAll([
          Section(s['pt_bayes']),
          _table(
            context,
            ['', ''],
            [
              ['τ', row(b['tau'] as Map<String, dynamic>)],
              ['σ', row(b['sigma'] as Map<String, dynamic>)],
              ['µ', '${n(b['mu_mean'], 6)}$u ± ${n(b['mu_sd'], 3)}$u'],
            ],
          ),
        ]);
      }
    }
    final st = r['stability'] as Map<String, dynamic>?;
    if (st != null) {
      out.addAll([
        Section(s['pt_stab_result']),
        Text(
          '${s['pt_difference']} ${n(st['difference'], 3)}$u; ${s['pt_criterion']} ${n(st['criterion'], 3)}$u — '
          '${st['sufficient'] == true ? s['pt_sufficient'] : s['pt_not_sufficient']}',
        ),
      ]);
    }
    return out;
  }

  List<Widget> _calibration(BuildContext context, Map<String, dynamic> r, String unit) {
    final s = strings;
    final u = unit.isEmpty ? '' : ' $unit';
    String n(dynamic v, [int sig = 5]) => v == null ? '—' : fmtNum(_d(v), sig, s.lang);
    const subs = ['₀', '₁', '₂', '₃'];
    final params = (r['params'] as List).map(_d).toList();
    final cov = (r['covariance'] as List).map((row) => (row as List).map(_d).toList()).toList();
    final dof = r['dof'] as int? ?? 0;
    final admissible = r['admissible'] == true;
    final out = <Widget>[
      Section('${s['cal_parameters']} — ${s['cal_${r['function']}']}'),
      _table(
        context,
        ['', s['value'], 'u'],
        [
          for (var j = 0; j < params.length; j++) ['b${subs[j.clamp(0, 3)]}', n(params[j]), n(math.sqrt(math.max(0, cov[j][j] ?? 0)))],
        ],
      ),
      Section(s['cal_fit']),
      _table(
        context,
        ['', ''],
        [
          ['S_res', '${n(r['s_res'])} (${s['cal_limit']} 2(n − N − 1) = ${2 * dof})'],
          ['Γ', n(r['gamma'], 3)],
        ],
      ),
      Text(admissible ? s['cal_admissible'] : s['cal_not_admissible'], style: admissible ? null : TextStyle(color: Theme.of(context).colorScheme.error)),
    ];
    final cands = (r['candidates'] as List? ?? const []).cast<Map<String, dynamic>>();
    if (cands.length > 1) {
      out.addAll([
        Section(s['cal_compare']),
        _table(
          context,
          ['', 'Γ', 'S_res'],
          [
            for (final c in cands) [s['cal_${c['function']}'], c['gamma'] == null ? '—' : n(c['gamma'], 3), c['s_res'] == null ? '—' : n(c['s_res'])],
          ],
        ),
      ]);
    }
    final samples = (r['samples'] as List? ?? const []).cast<Map<String, dynamic>>();
    if (samples.isNotEmpty) {
      out.addAll([
        Section(s['cal_results']),
        _table(
          context,
          ['', 'y', 'u(y)', 'x', 'u(x)', 'U(x) (k = ${n(r['coverage_factor'], 3)})', ''],
          [
            for (final x in samples)
              [
                x['name'].toString(),
                fmtGivenIn(_d(x['y']), s.lang),
                fmtGivenIn(_d(x['u_y']), s.lang),
                '${n(x['x'])}$u',
                '${n(x['u'])}$u',
                '${n(x['expanded'], 3)}$u',
                x['in_range'] == false ? s['cal_out_of_range'] : '',
              ],
          ],
        ),
      ]);
    }
    out.addAll([Section(s['cal_bound']), Text('u(x_lo) = ${n(r['u_lo'], 3)}$u, u(x_hi) = ${n(r['u_hi'], 3)}$u')]);
    final dev = (r['weighted_deviations'] as List? ?? const []).cast<Map<String, dynamic>>();
    final names = (r['point_names'] as List?)?.cast<String>();
    if (dev.isNotEmpty) {
      out.addAll([
        Section(s['cal_deviations']),
        ZScoreChart(
          rows: [
            for (var i = 0; i < dev.length; i++) ...[
              MapEntry('${names?[i] ?? 'M${i + 1}'} x', _d(dev[i]['x']) ?? 0),
              MapEntry('${names?[i] ?? 'M${i + 1}'} y', _d(dev[i]['y']) ?? 0),
            ],
          ],
          lang: s.lang,
          limits: const [2.0],
        ),
      ]);
    }
    final drift = r['drift'] as Map<String, dynamic>?;
    if (drift != null) {
      final d = (drift['differences'] as List).map(_d).toList();
      final c = (drift['critical'] as List).map(_d).toList();
      out.addAll([
        Section('${s['cal_drift']} — ${drift['reference']}: ${drift['passed'] == true ? s['cal_passed'] : s['cal_failed']}'),
        _table(
          context,
          ['', s['value'], '≤'],
          [
            for (final (i, l) in ['|ȳ₁ − y_c|', '|y_c − ȳ₂|', '|ȳ₁ − ȳ₂|'].indexed) [l, n(d[i], 3), n(c[i], 3)],
          ],
        ),
      ]);
    }
    final val = (r['validation'] as List? ?? const []).cast<Map<String, dynamic>>();
    if (val.isNotEmpty) {
      out.addAll([
        Section(s['cal_validation']),
        _table(
          context,
          ['', 'x_obs', 'x_ref', '|Δ|', '2√(u² + u²)', ''],
          [
            for (final v in val)
              [
                v['name'].toString(),
                n(v['x_obs']),
                fmtGivenIn(_d(v['x_ref']), s.lang),
                n(v['difference'], 3),
                n(v['limit'], 3),
                v['passed'] == true ? s['cal_passed'] : s['cal_failed'],
              ],
          ],
        ),
      ]);
    }
    return out;
  }

  List<Widget> _conformity(BuildContext context, Map<String, dynamic> r) {
    final s = strings;
    final u = r['unit'] == null ? '' : ' ${r['unit']}';
    String n(dynamic v, [int sig = 5]) => v == null ? '—' : '${fmtNum(_d(v), sig, s.lang)}$u';
    String p(dynamic v) {
      final d = _d(v);
      if (d == null) return '—';
      if (d > 0.9999) return '> ${fmtPct(0.9999, 4, s.lang)}';
      if (d < 0.0001) return '< ${fmtPct(0.0001, 1, s.lang)}';
      return fmtPct(d, d > 0.99 ? 2 + (-math.log(1 - d) / math.ln10).floor() : 2, s.lang);
    }

    final tol = r['tolerance'] as Map<String, dynamic>;
    final acc = r['acceptance'] as Map<String, dynamic>;
    String interval(Map<String, dynamic> m) => switch ((m['lower'], m['upper'])) {
      (null, null) => '—',
      (final l?, null) => '≥ ${n(l)}',
      (null, final h?) => '≤ ${n(h)}',
      (final l, final h) => '${n(l)} … ${n(h)}',
    };
    final out = <Widget>[
      Section(s['cf_title']),
      _table(
        context,
        ['', ''],
        [
          [s['cf_tolerance'], interval(tol)],
          [s['cf_rule'], s['cf_${r['rule']}']],
          [s['cf_acceptance'], interval(acc)],
          if (r['capability_index'] != null) [s['cf_capability'], fmtNum(_d(r['capability_index']), 3, s.lang)],
        ],
      ),
    ];
    final rows = (r['results'] as List? ?? const []).cast<Map<String, dynamic>>();
    if (rows.isNotEmpty) {
      final mcm = rows.any((x) => x['conformance_probability_mcm'] != null);
      out.add(
        _table(
          context,
          ['', s['value'], 'u', 'p_c', if (mcm) 'p_c (MCM)', ''],
          [
            for (final x in rows)
              [
                x['name'].toString(),
                n(x['value']),
                n(x['u'], 3),
                p(x['conformance_probability']),
                if (mcm) p(x['conformance_probability_mcm']),
                x['accepted'] == true ? s['cf_accepted'] : s['cf_rejected'],
              ],
          ],
        ),
      );
    }
    final g = r['global'] as Map<String, dynamic>?;
    if (g != null) {
      out.addAll([
        Section(s['cf_global']),
        _table(
          context,
          ['', ''],
          [
            [s['cf_conforming'], p(g['conforming'])],
            [s['cf_accepted_items'], p(g['accepted'])],
            [s['cf_consumer'], p(g['consumer_risk'])],
            [s['cf_producer'], p(g['producer_risk'])],
            if (r['r'] != null) ['r', fmtNum(_d(r['r']), 3, s.lang)],
          ],
        ),
      ]);
    }
    return out;
  }

  List<Widget> _precision(BuildContext context, Map<String, dynamic> r, String unit) {
    final s = strings;
    final u = unit.isEmpty ? '' : ' $unit';
    String n(dynamic v, [int sig = 4]) => v == null ? '—' : fmtNum(_d(v), sig, s.lang);
    final levels = (r['levels'] as List).cast<Map<String, dynamic>>();
    final reml = r['method'] == 'reml';
    final out = <Widget>[
      Section('${s['pe_precision']} — ${s[reml ? 'pe_reml' : 'pe_anova']}'),
      _table(
        context,
        [s['pe_level'], 'p', 'm̂', 's_r', 's_R', 's_L', if (reml) 's(m̂)', 'A_r', 'A_R'],
        [
          for (final l in levels)
            [
              l['name'].toString(),
              '${l['p']}',
              '${n(l['m'], 5)}$u',
              '${n(l['s_r'])}$u',
              '${n(l['s_R'])}$u',
              '${n(l['s_L'])}$u',
              if (reml) '${n(l['u_m'], 3)}$u',
              n(l['A_r'], 2),
              n(l['A_R'], 2),
            ],
        ],
      ),
    ];
    // Stragglers and outliers found in the scrutiny of all results.
    final flags = <List<String>>[];
    String outcome(dynamic o) => o == 'outlier' ? '${s['pe_outlier']} **' : '${s['pe_straggler']} *';
    for (final l in levels) {
      final sc = l['scrutiny'] as Map<String, dynamic>;
      for (final c in (sc['cochran'] as List? ?? const []).cast<Map<String, dynamic>>()) {
        if (c['outcome'] != 'correct') {
          flags.add([l['name'].toString(), s['pe_cochran'], c['laboratory'].toString(), n(c['statistic'], 3), outcome(c['outcome'])]);
        }
      }
      final g = sc['grubbs'] as Map<String, dynamic>?;
      if (g != null) {
        for (final k in ['low', 'high', 'repeated']) {
          final t = g[k] as Map<String, dynamic>?;
          if (t != null && t['outcome'] != 'correct') {
            flags.add([l['name'].toString(), s['pe_grubbs'], t['laboratory'].toString(), n(t['statistic'], 3), outcome(t['outcome'])]);
          }
        }
        for (final k in ['double_low', 'double_high']) {
          final t = g[k] as Map<String, dynamic>?;
          if (t != null && t['outcome'] != 'correct') {
            flags.add([l['name'].toString(), s['pe_grubbs'], (t['laboratories'] as List).join(', '), n(t['statistic'], 3), outcome(t['outcome'])]);
          }
        }
      }
    }
    out.add(Section(s['pe_flags']));
    out.add(flags.isEmpty ? Text(s['pe_none']) : _table(context, [s['pe_level'], '', '', '', ''], flags));
    final excluded = [
      for (final l in levels)
        for (final x in (l['excluded'] as List? ?? const [])) '${l['name']}: $x',
    ];
    if (excluded.isNotEmpty) out.add(Text('${s['pe_excluded']}: ${excluded.join('; ')}'));
    final rel = (r['relationships'] as List? ?? const []).cast<Map<String, dynamic>>();
    if (rel.isNotEmpty) {
      out.addAll([
        Section(s['pe_relationships']),
        _table(
          context,
          ['', '', for (final l in levels) l['name'].toString()],
          [
            for (final x in rel)
              [
                x['of'].toString(),
                x['relationship'].toString(),
                for (var i = 0; i < levels.length; i++) x['fitted'] == null ? '—' : n((x['fitted'] as List)[i], 3),
              ],
          ],
        ),
      ]);
    }
    return out;
  }

  List<Widget> _errorFunction(BuildContext context, Map<String, dynamic> r) {
    final s = strings;
    String n(dynamic v, [int sig = 4]) => v == null ? '—' : fmtNum(_d(v), sig, s.lang);
    final k = _d(r['coverage_factor']) ?? 2;
    final powers = ((r['function'] as Map)['powers'] as List).cast<int>();
    final coef = (r['coefficients'] as List).map(_d).toList();
    final uCoef = (r['u_coefficients'] as List).map(_d).toList();
    final failures = (r['residual_failures'] as List).cast<int>();
    final points = (r['points'] as List).cast<Map<String, dynamic>>();
    final minSm = r['min_s_m'] as Map<String, dynamic>;
    final readings = (r['readings'] as List? ?? const []).cast<Map<String, dynamic>>();
    final passed = r['chi_squared_passed'] == true;
    return [
      Section(s['ef_results']),
      _table(
        context,
        [s['ef_coefficient'], 'â', 'u(â)'],
        [
          for (var i = 0; i < powers.length; i++) ['a${powers[i]}', n(coef[i], 5), n(uCoef[i], 3)],
        ],
      ),
      Text('${passed ? s['ef_chi_ok'] : s['ef_chi_fail']}: χ² = ${n(r['chi_squared'], 3)}, ν = ${r['dof']}; s_m = ${n(r['s_m'], 3)}', key: const Key('ef-chi')),
      Text('${s['ef_min_s_m']}: ${n(minSm['chi_squared'], 3)} / ${n(minSm['residuals'], 3)}'),
      _table(
        context,
        ['I', 'E', 'E_appr', s['ef_residual'], 'U(E_appr)', 'C2.2-2b'],
        [
          for (final (j, p) in points.indexed)
            [
              n(p['indication'], 7),
              n(p['error'], 3),
              n(p['approximated'], 3),
              n(p['residual'], 3),
              n(k * (_d(p['u_approximated']) ?? 0), 3),
              failures.contains(j) ? '✗' : '✓',
            ],
        ],
      ),
      if (readings.isNotEmpty) ...[
        Section(s['ef_readings']),
        _table(
          context,
          ['R', 'E_appr', 'U (k = ${n(k, 3)})'],
          [
            for (final x in readings) [n(x['reading'], 7), n(x['error'], 4), n(x['expanded'], 3)],
          ],
        ),
      ],
      if (r['in_use'] is Map) ..._inUse(context, (r['in_use'] as Map).cast<String, dynamic>(), n),
    ];
  }

  List<Widget> _inUse(BuildContext context, Map<String, dynamic> u, String Function(dynamic, [int]) n) {
    final s = strings;
    List<double?> l(String key) => ((u[key] as List?) ?? const []).map(_d).toList();
    final lower = l('lower'), at = l('expanded_at_lower'), slope = l('slope'), gl = l('global_at_lower'), gs = l('global_slope');
    final req = _d(u['requirement']);
    return [
      Section(s['ef_in_use']),
      Text('β_W² = ${n(u['beta_squared'])}; U(W) = ${n(u['coverage_factor'], 3)}·√(α_W² + β_W²R²)'),
      _table(
        context,
        ['#', s['ef_from'], 'U(W)', s['ef_slope'], 'U_gl(W)'],
        [
          for (var i = 0; i < lower.length; i++) ['${i + 1}', n(lower[i], 7), n(at[i]), n(slope[i]), '${n(gl[i])} + ${n(gs[i])}·(R − ${n(lower[i], 7)})'],
        ],
      ),
      if (req != null)
        Text(
          '${s['ef_min_weight']} (Req = ${fmtPct(req, 3, s.lang)}): R_min = ${n(u['minimum_weight'], 3)}; SF = ${n(u['safety_factor'], 3)}: R_min,SF = ${n(u['minimum_weight_sf'], 3)}',
          key: const Key('ef-min-weight'),
        ),
    ];
  }

  List<Widget> _sampling(BuildContext context, Map<String, dynamic> r, String unit) {
    final s = strings;
    final u = unit.isEmpty ? '' : ' $unit';
    String n(dynamic v, [int sig = 4]) => v == null ? '—' : fmtNum(_d(v), sig, s.lang);
    String pct(dynamic v) => v == null ? '' : fmtPct((_d(v) ?? double.nan) / 100, 3, s.lang);
    final k = _d(r['coverage_factor']) ?? 2;
    final kText = fmtNum(k, 3, s.lang);
    if (r['design'] == 'gy') {
      final steps = (r['steps'] as List).cast<Map<String, dynamic>>();
      return [
        Section(s['sp_gy']),
        Text('${s['sp_constitution']} = ${n(r['constitution_factor'])} g cm⁻³'),
        _table(
          context,
          [s['sp_step'], 'C', 's_r', 'U′ (k = $kText)'],
          [
            for (final st in steps) [st['name'].toString(), n(st['sampling_constant']), n(st['rsd']), pct(100 * k * (_d(st['rsd']) ?? 0))],
            [s['sp_sampling'], '', n(r['rsd_sampling']), pct(r['expanded_relative_sampling'])],
            if (r['analytical_rsd'] != null) [s['sp_analytical'], '', n(r['analytical_rsd']), pct(100 * k * (_d(r['analytical_rsd']) ?? 0))],
            [s['sp_total'], '', n(r['rsd_total']), pct(r['expanded_relative_total'])],
          ],
        ),
      ];
    }
    final mean = _d(r['mean']) ?? double.nan;
    final share = (r['percent_of_total_variance'] as List).map(_d).toList();
    final exp = r['expanded_relative'] as Map<String, dynamic>;
    String rel(dynamic sd) => pct(100 * (_d(sd) ?? double.nan) / mean);
    final variances = ((r['calculation'] as Map<String, dynamic>)['variance_estimates'] as List).map(_d).toList();
    final out = <Widget>[
      Section(s['sp_results']),
      Text('${s['sp_mean']}: ${n(r['mean'], 5)}$u'),
      _table(
        context,
        ['', 's', s['sp_rel'], 'U′ (k = $kText)', s['sp_share']],
        [
          [s['sp_between'], '${n(r['s_between_target'])}$u', rel(r['s_between_target']), pct(exp['between_target']), pct(share[0])],
          [s['sp_sampling'], '${n(r['s_sampling'])}$u', rel(r['s_sampling']), pct(exp['sampling']), pct(share[1])],
          [s['sp_analysis'], '${n(r['s_analysis'])}$u', rel(r['s_analysis']), pct(exp['analysis']), pct(share[2])],
          [s['sp_measurement'], '${n(r['s_measurement'])}$u', rel(r['s_measurement']), pct(exp['measurement']), pct((share[1] ?? 0) + (share[2] ?? 0))],
          [s['sp_total'], '${n(r['s_total'])}$u', rel(r['s_total']), '', ''],
        ],
      ),
      if (variances.any((v) => v != null && v < 0)) Text(s['sp_negative'], key: const Key('sp-negative')),
    ];
    final g = r['log'] as Map<String, dynamic>?;
    if (g != null) {
      final f = g['factor'] as Map<String, dynamic>;
      out.addAll([
        Section(s['sp_log_title']),
        _table(
          context,
          ['', 's_G', 'F_U'],
          [
            [s['sp_between'], n(g['s_between_target']), ''],
            [s['sp_sampling'], n(g['s_sampling']), n(f['sampling'])],
            [s['sp_analysis'], n(g['s_analysis']), n(f['analysis'])],
            [s['sp_measurement'], n(g['s_measurement']), n(f['measurement'])],
          ],
        ),
        Text("u′ = √(exp(s_G²) − 1) = ${n(g['relative_measurement'])}"),
      ]);
    }
    return out;
  }

  List<Widget> _controlChart(BuildContext context, Map<String, dynamic> r, String unit) {
    final s = strings;
    final u = r['chart'] == 'relative_range' ? ' %' : (unit.isEmpty ? '' : ' $unit');
    String n(dynamic v, [int sig = 4]) => v == null ? '—' : fmtNum(_d(v), sig, s.lang);
    final lim = r['limits'] as Map<String, dynamic>;
    final values = [for (final v in (r['values'] as List? ?? const [])) _d(v) ?? double.nan];
    final daily = (r['daily'] as List? ?? const []).cast<String>();
    final labels = (r['labels'] as List? ?? const []).cast<String>();
    final out = <Widget>[
      Section('${s['qc_limits']} — ${s['qc_${r['chart']}']}'),
      _table(
        context,
        ['', s['qc_lower'], s['qc_upper']],
        [
          [s['qc_center'], '${n(r['center'], 5)}$u', ''],
          if (lim['upper_warning'] != null)
            [s['qc_wl'], lim['lower_warning'] == null ? '—' : '${n(lim['lower_warning'], 5)}$u', '${n(lim['upper_warning'], 5)}$u'],
          [s['qc_al'], lim['lower_action'] == null ? '—' : '${n(lim['lower_action'], 5)}$u', '${n(lim['upper_action'], 5)}$u'],
          if (r['sd'] != null) ['s', '${n(r['sd'])}$u', ''],
        ],
      ),
    ];
    if (values.isNotEmpty) {
      out.add(
        ControlChart(
          key: const Key('control-chart'),
          values: values,
          outOfControl: [for (final d in daily) d == 'outside_action' || d == 'two_of_three'],
          center: _d(r['center'])!,
          upperAction: _d(lim['upper_action'])!,
          lowerAction: _d(lim['lower_action']),
          upperWarning: _d(lim['upper_warning']),
          lowerWarning: _d(lim['lower_warning']),
          establish: r['establish'] as int?,
          lang: s.lang,
        ),
      );
      final ok = daily.where((d) => d == 'in_control').length;
      final w = daily.where((d) => d == 'warning').length;
      out.add(Section(s['qc_daily']));
      out.add(
        Text(
          s['qc_summary'].replaceAll('{n}', '${values.length}').replaceAll('{ok}', '$ok').replaceAll('{w}', '$w').replaceAll('{oc}', '${r['out_of_control']}'),
          key: const Key('qc-summary'),
        ),
      );
      final flagged = [
        for (var i = 0; i < daily.length; i++)
          if (daily[i] != 'in_control') i,
      ];
      if (flagged.isNotEmpty) {
        out.add(
          _table(
            context,
            [s['qc_run_no'], if (labels.isNotEmpty) '', s['qc_value'], s['qc_status']],
            [
              for (final i in flagged) ['${i + 1}', if (labels.isNotEmpty) labels[i], '${fmtNum(values[i], 5, s.lang)}$u', s['qc_${daily[i]}']],
            ],
          ),
        );
      }
      final signals = (r['statistical_signals'] as List? ?? const []).cast<Map<String, dynamic>>();
      final flaggedSignals = [
        for (var i = 0; i < signals.length; i++)
          if (signals[i]['trend'] == true || signals[i]['run'] == true)
            '${i + 1} (${[if (signals[i]['trend'] == true) s['qc_trend'], if (signals[i]['run'] == true) s['qc_runs']].join(', ')})',
      ];
      out.add(Text('${s['qc_signals']}: ${flaggedSignals.isEmpty ? s['qc_none'] : flaggedSignals.join('; ')}'));
    }
    final bias = r['bias_test'] as Map<String, dynamic>?;
    final stats = r['statistics'] as Map<String, dynamic>?;
    if (bias != null && stats != null) {
      final sig = (_d(bias['t']) ?? 0) > (_d(bias['t_critical']) ?? double.infinity);
      out.add(
        Text(
          '${s['qc_bias']}: x̄ = ${n(stats['mean'], 5)}$u, T = ${n(bias['reference'], 5)}$u, t = ${n(bias['t'], 3)} '
          '(${s['qc_critical']} ${n(bias['t_critical'], 3)}): ${sig ? s['qc_significant'] : s['qc_not_significant']}',
        ),
      );
    }
    final rv = r['review'] as Map<String, dynamic>?;
    if (rv != null) {
      String sum(Map<String, dynamic> x) => 'n = ${x['n']}, ${x['mean'] == null ? '' : 'x̄ = ${n(x['mean'], 5)}$u, '}s = ${n(x['sd'])}$u';
      String verdict(dynamic b) => b == true ? s['qc_significant'] : s['qc_not_significant'];
      out.addAll([
        Section(s['qc_review_r']),
        _table(
          context,
          ['', ''],
          [
            [s['qc_previous'], sum(rv['previous'] as Map<String, dynamic>)],
            [s['qc_current'], sum(rv['current'] as Map<String, dynamic>)],
            ['F', '${n(rv['F'])} (${s['qc_critical']} ${n(rv['F_critical'], 3)}): ${verdict(rv['sd_significant'])}'],
            if (rv['t'] != null) ['t', '${n(rv['t'])} (${s['qc_critical']} ${n(rv['t_critical'], 3)}): ${verdict(rv['mean_significant'])}'],
            if (rv['outside_warning'] != null) [s['qc_outside_wl'], '${rv['outside_warning']}'],
          ],
        ),
      ]);
    }
    final pr = r['precision'] as Map<String, dynamic>?;
    if (pr != null) {
      out.addAll([
        Section(s['qc_precision']),
        _table(
          context,
          ['', ''],
          [
            ['s_r (ANOVA)', '${n(pr['s_r'])}$u'],
            ['s_I (ANOVA)', '${n(pr['s_between'])}$u'],
            ['s_Rw (ANOVA)', '${n(pr['s_rw'])}$u'],
            ['s_r (Eq. 10)', '${n(pr['s_r_pooled_eq10'])}$u'],
            if (pr['s_rw_simplified'] != null) [s['qc_simplified'], '${n(pr['s_rw_simplified'])}$u'],
          ],
        ),
      ]);
    }
    return out;
  }

  List<Widget> _multivariate(BuildContext context, Map<String, dynamic> r) {
    final s = strings;
    final outputs = (r['outputs'] as List).cast<Map<String, dynamic>>();
    final names = [for (final o in outputs) o['name'].toString()];
    String n(dynamic v, [int sig = 4]) => v == null ? '—' : fmtNum(_d(v), sig, s.lang);
    String unit(int j) => outputs[j]['unit'] == null ? '' : ' ${outputs[j]['unit']}';
    List<Widget> block(String title, Map<String, dynamic> m) {
      final y = (m['estimate'] as List).map(_d).toList();
      final u = (m['standard_uncertainties'] as List).map(_d).toList();
      final corr = (m['correlation'] as List).map((row) => (row as List).map(_d).toList()).toList();
      return [
        Section(title),
        _table(
          context,
          ['', s['value'], 'u'],
          [
            for (var j = 0; j < names.length; j++) [names[j], '${n(y[j], 7)}${unit(j)}', '${n(u[j], 3)}${unit(j)}'],
          ],
        ),
        if (names.length > 1) ...[
          Text(s['mv_correlation'], style: Theme.of(context).textTheme.titleSmall),
          _table(
            context,
            ['', ...names],
            [
              for (var i = 0; i < names.length; i++) [names[i], for (var j = 0; j < names.length; j++) fmtNum(corr[i][j], 3, s.lang)],
            ],
          ),
          Text('${s['mv_regions']}: k_p = ${n(m['k_ellipsoid'], 3)}, k_q = ${n(m['k_rectangle'], 3)}'),
        ],
      ];
    }

    final out = <Widget>[];
    if (r['gum'] != null) out.addAll(block(s['mv_gum'], r['gum'] as Map<String, dynamic>));
    if (r['mcm'] != null) {
      final m = r['mcm'] as Map<String, dynamic>;
      out.addAll(block('${s['mv_mcm']} (M = ${m['trials']})', m));
    }
    final v = r['validation'] as Map<String, dynamic>?;
    if (v != null) {
      final ok = v['ellipsoid_validated'] == true;
      out.add(
        Padding(
          padding: const EdgeInsets.only(top: 8),
          child: Text(ok ? s['mv_validated'] : s['mv_not_validated'], style: ok ? null : TextStyle(color: Theme.of(context).colorScheme.error)),
        ),
      );
    }
    return out;
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
