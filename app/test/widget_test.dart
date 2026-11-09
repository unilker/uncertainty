// Tests of the interface. The engine library must be built first:
//   cargo build --release -p uncertainty-ffi

import 'dart:convert';
import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:uncertainty_app/main.dart';
import 'package:uncertainty_app/src/app_state.dart';
import 'package:uncertainty_app/src/engine.dart';
import 'package:uncertainty_app/src/format.dart';
import 'package:uncertainty_app/src/learn/explain.dart';
import 'package:uncertainty_app/src/learn/guide.dart';
import 'package:uncertainty_app/src/widgets/charts.dart';
import 'package:uncertainty_app/src/widgets/calibration_editor.dart';
import 'package:uncertainty_app/src/widgets/conformity_editor.dart';
import 'package:uncertainty_app/src/widgets/consensus_editor.dart';
import 'package:uncertainty_app/src/widgets/control_chart_editor.dart';
import 'package:uncertainty_app/src/widgets/error_function_editor.dart';
import 'package:uncertainty_app/src/widgets/precision_editor.dart';
import 'package:uncertainty_app/src/widgets/proficiency_editor.dart';
import 'package:uncertainty_app/src/widgets/sampling_editor.dart';

/// Pumps until [finder] matches, letting real asynchronous work (asset
/// loading, the engine isolate) progress in between.
Future<void> pumpUntilFound(WidgetTester tester, Finder finder, {int attempts = 100}) async {
  for (var i = 0; i < attempts; i++) {
    await tester.runAsync(() => Future<void>.delayed(const Duration(milliseconds: 100)));
    await tester.pump();
    if (finder.evaluate().isNotEmpty) break;
  }
  await tester.pumpAndSettle();
}

/// Scrolls the home screen down to the library and waits until it is loaded.
Future<void> showLibrary(WidgetTester tester) async {
  await tester.pump();
  await tester.scrollUntilVisible(find.byKey(const Key('search')), 300, scrollable: find.byType(Scrollable).first);
  await pumpUntilFound(tester, find.textContaining(RegExp(r'Example library \([1-9]|Örnek kütüphanesi \([1-9]')));
}

/// Types [query] into the search field of the library, which may be below
/// the cards of the home screen.
Future<void> searchLibrary(WidgetTester tester, String query) async {
  await tester.scrollUntilVisible(find.byKey(const Key('search')), 300, scrollable: find.byType(Scrollable).first);
  await tester.enterText(find.byKey(const Key('search')), query);
}

void main() {
  setUpAll(() {
    final name = Platform.isWindows
        ? 'uncertainty_ffi.dll'
        : Platform.isMacOS
        ? 'libuncertainty_ffi.dylib'
        : 'libuncertainty_ffi.so';
    engineLibraryPath = Platform.environment['UNCERTAINTY_FFI'] ?? '${Directory.current.path}/../target/release/$name';
  });

  group('format', () {
    test('numbers and percentages', () {
      expect(fmtNum(1002.69972, 6, 'en'), '1002.70');
      expect(fmtNum(1002.69972, 6, 'tr'), '1002,70');
      expect(fmtNum(9.999, 3, 'en'), '10.0');
      expect(fmtNum(2.5e-7, 2, 'en'), '2.5 × 10⁻⁷');
      expect(fmtEstimate(50000838, 31.7, 'en'), '50000838');
      expect(fmtEstimate(1002.69972, 0.86, 'tr'), '1002,70');
      expect(fmtNum(double.infinity, 3, 'en'), '∞');
      expect(fmtPct(0.9545, 4, 'tr'), '%95,45');
      expect(fmtPct(0.594, 3, 'en'), '59.4 %');
    });

    test('parsing typed data', () {
      expect(parseNum('0,05'), 0.05);
      expect(parseList('1.5\n2,5  3'), [1.5, 2.5, 3.0]);
      expect(parseList('1, 2, 3'), [1.0, 2.0, 3.0]);
      expect(parseList('1 x'), isNull);
      expect(parsePairs('7.46\t7.25\n9,01 9,17\n'), [
        [7.46, 7.25],
        [9.01, 9.17],
      ]);
      expect(parsePairs('1 2 3'), isNull);
    });

    test('test points of an error function need four values', () {
      final (rows, bad) = parseTestPoints('0\t0\t0\t0,0416\n50.000067 50.000006 0.0142 0.0512');
      expect(bad, isNull);
      expect(rows![1], [50.000067, 50.000006, 0.0142, 0.0512]);
      expect(parseTestPoints('1 2 3').$2, 1);
      expect(parseTestPoints('x 1 2 3 4').$2, 1);
      final ef = {
        'points': [
          {'indication': 1.5, 'error': 0.2, 'u_reference': 0.01, 'u_indication': 0.02},
        ],
      };
      expect(pointsUseReference(ef), isFalse);
      expect(testPointsToText(ef, 'tr'), '1,5\t0,2\t0,01\t0,02');
    });

    test('consensus results: name, value, u and optional dof', () {
      final (r, bad) = parseConsensusResults('UME\t2,150\t0,030\nNIST; 1.984; 0.020; 12\n');
      expect(bad, isNull);
      expect(r!.length, 2);
      expect(r[0], {'name': 'UME', 'value': 2.15, 'u': 0.03});
      expect(r[1]['dof'], 12);
      expect(parseConsensusResults('A 1 0\nB 2 0.1').$2, 1);
      expect(parseConsensusResults('A 1 0.1\nB 2').$2, 2);
      expect(consensusResultsToText({'results': r}, 'tr'), 'UME\t2,15\t0,03\nNIST\t1,984\t0,02\t12');
    });

    test('duplicate samples need four values per target', () {
      final (r, bad) = parseDuplicates('A4\t787\t769\t811\t780\n\nB7; 338; 327; 651; 563');
      expect(bad, isNull);
      expect(r!.$1, ['A4', 'B7']);
      expect(r.$2[1], [338, 327, 651, 563]);
      expect(parseDuplicates('1 2 3 4\n\n1 2 3').$2, 3);
      expect(
        duplicatesToText({
          'duplicates': [
            [1.5, 2, 3, 4],
          ],
          'labels': ['x'],
        }, 'tr'),
        'x\t1,5\t2\t3\t4',
      );
    });

    test('control-chart runs with optional labels', () {
      final (r1, bad1) = parseRuns('4-Dec\t1,02\n5-Dec; 0.98\n');
      expect(bad1, isNull);
      expect(r1!.$1, ['4-Dec', '5-Dec']);
      expect(r1.$2, [
        [1.02],
        [0.98],
      ]);
      final (r2, _) = parseRuns('7.1 7.1 7.0\n6.9 6.7 6.8');
      expect(r2!.$1, isEmpty);
      expect(r2.$2[1], [6.9, 6.7, 6.8]);
      expect(parseRuns('1.0\nday x').$2, 2);
      expect(
        runsToText({
          'values': [1.5, 2],
          'labels': ['a', 'b'],
        }, 'tr'),
        'a\t1,5\nb\t2',
      );
    });
  });

  group('engine', () {
    test('templates evaluate and errors are reported', () {
      const e = Engine();
      expect(e.version()['version'], isNotEmpty);
      for (final kind in ['model', 'topdown', 'calibration', 'conformity', 'precision', 'multivariate']) {
        final p = e.template(kind);
        expect(e.validate(p), isNull);
        final r = e.evaluateSync(p, const EvaluateOptions(lang: 'tr', trials: 10000));
        expect(
          (r['reported'] as List).first,
          contains(switch (kind) {
            'conformity' => 'p_c',
            'precision' => 's_R',
            'multivariate' => 'u(R)',
            _ => '±',
          }),
        );
      }
      // Localised warnings and the Kragten view come from the engine.
      final h1 = jsonDecode(File('../library/length/gum-h1-end-gauge.json').readAsStringSync()) as Map<String, dynamic>;
      final w = e.evaluateSync(h1, const EvaluateOptions(lang: 'tr', skipMcm: true));
      expect((w['warnings'] as List).single, contains('alpha_s, theta_bar, Delta için duyarlılık katsayısı'));
      final a1 = jsonDecode(File('../library/chemistry/quam-a1-cadmium-standard.json').readAsStringSync()) as Map<String, dynamic>;
      final k = e.evaluateSync(a1, const EvaluateOptions(skipMcm: true));
      expect((k['results']['gum']['kragten']['combined_standard_uncertainty'] as num).toDouble(), closeTo(0.8633, 0.00005));
      // The spreadsheet verification workbook (an .xlsx file is a ZIP archive: "PK").
      final x = e.evaluateSync(a1, const EvaluateOptions(lang: 'tr', skipMcm: true, xlsx: true));
      expect(x['xlsx_supported'], isTrue);
      expect(base64Decode(x['xlsx_base64'] as String).take(2), [0x50, 0x4b]);
      expect(hasVerificationWorkbook(a1), isTrue);
      expect(hasVerificationWorkbook(e.template('calibration')), isFalse);
      final bad = e.template('model')..remove('model');
      expect(e.validate(bad), contains('model'));
      expect(() => e.evaluateSync(bad, const EvaluateOptions()), throwsA(isA<EngineException>()));
    });
  });

  testWidgets('open a library example and calculate it', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1200, 900));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    expect(find.textContaining('Example library (286'), findsOneWidget);

    await searchLibrary(tester, 'quam-a1-cadmium-standard-volume');
    await tester.pumpAndSettle();
    await tester.ensureVisible(find.text('Cadmium standard with the volume sub-model'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Cadmium standard with the volume sub-model'));
    await tester.pumpAndSettle();
    expect(find.text('Input quantities'), findsNothing); // on the overview tab
    expect(find.textContaining('QUAM Example A1'), findsWidgets);

    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.textContaining('c_Cd = (1002.7 ± 1.7) mg/L'));
    expect(find.textContaining('c_Cd = (1002.7 ± 1.7) mg/L'), findsOneWidget);

    // The language switch re-labels the interface.
    await tester.tap(find.text('TR'));
    await tester.pumpAndSettle();
    expect(find.text('Sonuçlar'), findsOneWidget);
  });

  testWidgets('a Monte-Carlo-only project keeps the GUM switched off', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1200, 900));
    final state = AppState('en');
    await tester.pumpWidget(UncertaintyApp(state: state));
    await showLibrary(tester);
    await searchLibrary(tester, 'nist-tn1900-e11');
    await tester.pumpAndSettle();
    await tester.ensureVisible(find.text('Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11)'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Microwave step attenuator, U-shaped inputs (NIST TN 1900 E11)'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Settings'));
    await tester.pumpAndSettle();
    final gumSwitch = find.widgetWithText(SwitchListTile, 'Evaluate with the GUM framework');
    expect(tester.widget<SwitchListTile>(gumSwitch).value, isFalse);
    expect(find.text('Coverage factor k'), findsNothing);

    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.textContaining('30.04'), attempts: 600);
    expect(find.textContaining('30.04'), findsWidgets);
    expect(find.text('Monte Carlo method (JCGM 101)'), findsOneWidget);
    expect(find.text('GUM uncertainty framework (JCGM 100)'), findsNothing);
  });

  test('proficiency results are read from pasted lines', () {
    final (rows, bad) = parseResults('L04\t0,013\t0,003\t2\tAMA\nL17 <0.015\nL23;0.0135;0.00108;1.732\n\nL08 0.044');
    expect(bad, isNull);
    expect(rows!.length, 4);
    expect(rows[0], {'lab': 'L04', 'value': 0.013, 'expanded': 0.003, 'k': 2.0, 'method': 'AMA'});
    expect(rows[1], {'lab': 'L17', 'less_than': 0.015});
    expect(rows[2]['k'], 1.732);
    expect(rows[3], {'lab': 'L08', 'value': 0.044});
    expect(parseResults('L01 abc').$2, 1);
    expect(resultsToText(rows, 'en').split('\n').first, 'L04\t0.013\t0.003\t2\tAMA');
  });

  testWidgets('a proficiency test round shows the scores of ISO 13528 E.4', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'iso13528-e4');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('Mercury in animal feed'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pt-results')), findsOneWidget);
    expect(find.textContaining('L17\t<0.015'), findsOneWidget);

    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.text('Performance statistics'));
    expect(find.text('-4.70'), findsWidgets); // z of L04 and L05, Table E.7
    expect(find.text('-7.10'), findsOneWidget); // ζ of L04
    final list = find.ancestor(of: find.text('Performance statistics'), matching: find.byType(Scrollable)).first;
    await tester.scrollUntilVisible(find.text('z scores (±2 warning, ±3 action)'), 400, scrollable: list);
    expect(find.byType(ZScoreChart), findsOneWidget);
  });

  test('calibration points are read from pasted lines', () {
    final (rows, bad) = parseNamedRows('M1\t4,5\t0,045\t0,1969\t0,003938\n18.75 0.1875 0.7874 0.015748\n', const ['x', 'u_x', 'y', 'u_y'], 'M');
    expect(bad, isNull);
    expect(rows![0], {'name': 'M1', 'x': 4.5, 'u_x': 0.045, 'y': 0.1969, 'u_y': 0.003938});
    expect(rows[1]['name'], 'M2');
    expect(parseNamedRows('S1 0.2', const ['y', 'u_y'], 'S').$2, 1);
    expect(namedRowsToText(rows, const ['x', 'u_x', 'y', 'u_y'], 'tr').split('\n').first, 'M1\t4,5\t0,045\t0,1969\t0,003938');
  });

  testWidgets('a calibration in both variables reproduces ISO 6143 Example 1', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'iso6143-b1');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('NO in N₂ by UV absorption'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('cal-points')), findsOneWidget);
    expect(find.textContaining('M2\t18.75\t0.1875'), findsOneWidget);

    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.text('Contents of the mixtures'));
    expect(find.text('14.409 µmol/mol'), findsOneWidget); // Table B.6
    expect(find.text('0.35599 µmol/mol'), findsOneWidget);
    expect(find.textContaining('Γ ≤ 2'), findsOneWidget);
    final list = find.ancestor(of: find.text('Contents of the mixtures'), matching: find.byType(Scrollable)).first;
    await tester.scrollUntilVisible(find.text('Weighted deviations (x and y) with the limits ±2'), 400, scrollable: list);
    expect(find.byType(ZScoreChart), findsOneWidget);
  });

  test('measured values for a conformity assessment are read from pasted lines', () {
    final (rows, bad) = parseMeasured('car A 105\ncar B\t108,5\t2,2\n\nS3;1.5;0.1');
    expect(bad, isNull);
    expect(rows, [
      {'name': 'car A', 'value': 105.0},
      {'name': 'car B', 'value': 108.5, 'u': 2.2},
      {'name': 'S3', 'value': 1.5, 'u': 0.1},
    ]);
    expect(parseMeasured('only-a-name').$2, 1);
  });

  testWidgets('global risks of JCGM 106 9.5.3 in the results', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'jcgm106-9.5-precision');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('Precision resistors'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('cf-rule')), findsOneWidget);
    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.text('Global risks'));
    expect(find.text('0.99 %'), findsOneWidget); // R_C, JCGM 106: 1 %
    expect(find.text('6.9 %'), findsOneWidget); // R_P: 7 %
    expect(find.text('guarded acceptance'), findsOneWidget);
  });

  test('results and exclusions of a precision experiment are read from pasted lines', () {
    final (cells, bad) = parseCells('L1 1,02 1,05\nLab 2\t0.98\t1.00\n\nL3;1.07;-;1.04');
    expect(bad, isNull);
    expect(cells!.map((e) => e.key).toList(), ['L1', 'Lab 2', 'L3']);
    expect(cells[0].value, [1.02, 1.05]);
    expect(cells[2].value, [1.07, 1.04]);
    expect(parseCells('L1 1\nL1 2').$2, 2);
    expect(parseCells('L1 x').$2, 1);
    final (ex, bad2) = parseExclusions('1\n6; 5\n2; 3; 1; wrong sample; relabelled');
    expect(bad2, isNull);
    expect(ex, [
      {'laboratory': '1'},
      {'laboratory': '6', 'level': '5'},
      {'laboratory': '2', 'level': '3', 'result': 1, 'reason': 'wrong sample; relabelled'},
    ]);
    expect(parseExclusions('1; 2; zero').$2, 1);
    expect(exclusionsToText(ex!, 'en'), '1\n6; 5\n2; 3; 1; wrong sample; relabelled');
  });

  testWidgets('the creosote oil experiment of ISO 5725-2 C.3 flags laboratory 1', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'iso5725-2-c3-creosote-oil');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('outlying laboratory and precision relationships'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('pe-cells-0')), findsOneWidget);
    final editor = find.descendant(of: find.byType(PrecisionEditor), matching: find.byType(Scrollable)).first;
    await tester.scrollUntilVisible(find.byKey(const Key('pe-exclude')), 400, scrollable: editor);
    expect(find.byKey(const Key('pe-exclude')), findsOneWidget);
    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.text('Stragglers and outliers (all results)'));
    expect(find.text('outlier **'), findsNWidgets(2)); // Grubbs, laboratory 1 at levels 3 and 4
    expect(find.text('0.3368 %'), findsOneWidget); // s_r at level 4, Table C.18: 0.337
  });

  testWidgets('a model with several outputs (JCGM 102 9.4) shows its correlation matrix', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'jcgm102-9.4-resistance-reactance-alternative');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('t covariance'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Model'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('mv-implicit')), findsOneWidget);
    expect(find.textContaining('Indication values: V, I, phi'), findsOneWidget);
    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.text('GUM uncertainty framework (JCGM 102)'));
    expect(find.text('0.130 Ω'), findsWidgets); // u(R), Table 11, Alt. GUF
    expect(find.text('-0.588'), findsWidgets); // r(R, X)
  });

  testWidgets('the zinc control chart of Nordtest TR 569 Table 1', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'tr569-table1-zinc');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('Zinc in water: X-chart'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('qc-data')), findsOneWidget);
    expect(find.byKey(const Key('qc-basis')), findsOneWidget);
    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.byKey(const Key('control-chart')));
    expect(find.text('60 control values: 57 within the warning limits, 3 between warning and action limits (in control), 0 out of control.'), findsOneWidget);
    expect(find.text('65.474 µg/L'), findsWidgets); // upper warning limit
  });

  testWidgets('degrees of equivalence of the 60Co comparison by procedure', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'nicob-co60');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('in 19 laboratories'));
    await tester.pumpAndSettle();
    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.text('Consensus value (DerSimonian–Laird)'), attempts: 1200);
    final list = find.descendant(of: find.byKey(const Key('results-list')), matching: find.byType(Scrollable)).first;
    await tester.scrollUntilVisible(find.byKey(const Key('cs-bilateral-by-procedure')), 400, scrollable: list);
    final text = tester.widget<Text>(find.byKey(const Key('cs-bilateral-by-procedure'))).data!;
    expect(text, contains('DerSimonian–Laird 9/171'));
    expect(text, contains('Linear Pool 0/171'));
    expect(find.text('Degrees of equivalence by procedure'), findsOneWidget);
  });

  testWidgets('lead in soil by the duplicate method of Eurachem UfS Example A2', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'ufs-a2');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('Lead in contaminated top soil'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('sp-data')), findsOneWidget);
    expect(find.byKey(const Key('sp-method')), findsOneWidget);
    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.text('Uncertainty from sampling'));
    expect(find.text('136.6 mg/kg'), findsOneWidget); // s_meas, Figure A2.2
    expect(find.text('2.621'), findsOneWidget); // F_U, measurement
  });

  testWidgets('the error function of EURAMET cg-18 H4 with s_m = 0.05 mg', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'cg18-h4-400g-error-function-sm-0.05');
    await tester.pumpAndSettle();
    await tester.tap(find.textContaining('error approximation function with s_m = 0.05 mg'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    expect(find.byKey(const Key('ef-data')), findsOneWidget);
    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.byKey(const Key('ef-chi')));
    expect(find.text('0.00083839'), findsOneWidget); // a₁, H4.4
    expect(find.textContaining('χ² test passed (C2.2-2a): χ² = 7.32, ν = 8'), findsOneWidget);
  });

  testWidgets('top-down forms show and edit the raw data', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await showLibrary(tester);
    await searchLibrary(tester, 'nordtest-8-bod-pt');
    await tester.pumpAndSettle();
    await tester.tap(find.text('BOD in wastewater: control chart and proficiency tests'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('Data'));
    await tester.pumpAndSettle();
    await tester.scrollUntilVisible(find.text('s_R (%)').first, 300, scrollable: find.byType(Scrollable).last);
    expect(find.text('s_R (%)'), findsNWidgets(3)); // one per proficiency-test round
    expect(find.text('proficiency tests'), findsOneWidget);

    await tester.tap(find.byKey(const Key('run')));
    await pumpUntilFound(tester, find.textContaining('U = 10 % (k = 2.00)'));
    expect(find.textContaining('U = 10 % (k = 2.00)'), findsOneWidget);
  });

  group('guide', () {
    List<Map<String, dynamic>> library() => [
      for (final i in jsonDecode(File('assets/library.json').readAsStringSync()) as List) ((i as Map)['project'] as Map).cast<String, dynamic>(),
    ];
    String idOf(Map<String, dynamic> p) => (p['metadata'] as Map)['id'] as String;

    test('every library example has a lesson', () {
      final lessons = parseLessons(File('assets/guide/lessons.json').readAsStringSync());
      for (final p in library()) {
        expect(lessons.forId(idOf(p)), isNotNull, reason: idOf(p));
      }
      for (final l in lessons.items) {
        expect([1, 2, 3], contains(l.level), reason: l.prefix);
        expect(l.inLang('en'), isNotEmpty);
        expect(l.inLang('tr'), isNotEmpty);
      }
    });

    test('links of the guide point to existing chapters, anchors and examples', () {
      final ids = {for (final p in library()) idOf(p)};
      Set<String> anchors(String text) => {for (final m in RegExp(r'\{#([\w-]+)\}').allMatches(text)) m.group(1)!};
      for (final lang in ['en', 'tr']) {
        for (final ch in guideChapters) {
          final text = File('assets/guide/$lang/$ch.md').readAsStringSync();
          expect(text.startsWith('# '), isTrue, reason: '$lang/$ch');
          expect(anchors(text), anchors(File('assets/guide/${lang == 'en' ? 'tr' : 'en'}/$ch.md').readAsStringSync()), reason: 'anchors of $ch differ between languages');
          for (final m in RegExp(r'\]\(([^)\s]+)\)').allMatches(text)) {
            final target = m.group(1)!;
            if (target.startsWith('http')) continue;
            if (target.startsWith('example:')) {
              expect(ids, contains(target.substring(8)), reason: '$lang/$ch → $target');
              continue;
            }
            final parts = target.split('#');
            if (parts.first == examplesChapter) continue;
            expect(guideChapters, contains(parts.first), reason: '$lang/$ch → $target');
            if (parts.length > 1) {
              expect(anchors(File('assets/guide/$lang/${parts.first}.md').readAsStringSync()), contains(parts[1]), reason: '$lang/$ch → $target');
            }
          }
        }
      }
    });

    test('every project of the library can be explained in both languages', () {
      for (final p in library()) {
        for (final lang in ['en', 'tr']) {
          final md = explainProject(p, lang);
          expect(md, isNotEmpty, reason: idOf(p));
          expect(md.replaceAll('null detector', '').contains('null'), isFalse, reason: '${idOf(p)} $lang');
        }
      }
      // The certificate of the 10 kg weight: u = U/k.
      final weight = library().firstWhere((p) => idOf(p) == 'ea402-s2-weight-10kg');
      expect(explainProject(weight, 'en'), contains('u = U/k = 45/2 = 22.5 mg'));
      expect(explainProject(weight, 'tr'), contains('u = U/k = 45/2 = 22,5 mg'));
    });

    testWidgets('the guide opens from the home screen and moves between chapters', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1200, 900));
      await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
      await pumpUntilFound(tester, find.byKey(const Key('open-guide')));
      await tester.tap(find.byKey(const Key('open-guide')));
      await pumpUntilFound(tester, find.text('1. Start here'));
      expect(find.text('1. Start here'), findsWidgets);
      await tester.ensureVisible(find.byKey(const Key('guide-next')));
      await tester.pumpAndSettle();
      await tester.tap(find.byKey(const Key('guide-next')));
      await tester.pumpAndSettle();
      expect(find.text('2. What is measurement uncertainty?'), findsWidgets);
      await tester.enterText(find.byKey(const Key('guide-search')), 'Welch');
      await tester.pumpAndSettle();
      expect(find.widgetWithText(ListTile, '3. The key words in plain language'), findsOneWidget);
      expect(find.widgetWithText(ListTile, '1. Start here'), findsNothing);
    });

    testWidgets('a library example explains itself step by step', (tester) async {
      await tester.binding.setSurfaceSize(const Size(1200, 900));
      await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
      await showLibrary(tester);
      await searchLibrary(tester, 'ea402-s2-weight-10kg');
      await tester.pumpAndSettle();
      await tester.tap(find.text('Calibration of a 10 kg weight'));
      await pumpUntilFound(tester, find.byKey(const Key('learn-lesson')));
      expect(find.text('Calibrating a weight'), findsOneWidget);
      expect(find.textContaining('u = U/k = 45/2 = 22.5 mg'), findsOneWidget);
    });
  });
}
