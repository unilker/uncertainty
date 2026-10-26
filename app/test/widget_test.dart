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
        [9.01, 9.17]
      ]);
      expect(parsePairs('1 2 3'), isNull);
    });
  });

  group('engine', () {
    test('templates evaluate and errors are reported', () {
      const e = Engine();
      expect(e.version()['version'], isNotEmpty);
      for (final kind in ['model', 'topdown']) {
        final p = e.template(kind);
        expect(e.validate(p), isNull);
        final r = e.evaluateSync(p, const EvaluateOptions(lang: 'tr', trials: 10000));
        expect((r['reported'] as List).first, contains('±'));
      }
      // Localised warnings and the Kragten view come from the engine.
      final h1 = jsonDecode(File('../library/length/gum-h1-end-gauge.json').readAsStringSync()) as Map<String, dynamic>;
      final w = e.evaluateSync(h1, const EvaluateOptions(lang: 'tr', skipMcm: true));
      expect((w['warnings'] as List).single, contains('alpha_s, theta_bar, Delta için duyarlılık katsayısı'));
      final a1 = jsonDecode(File('../library/chemistry/quam-a1-cadmium-standard.json').readAsStringSync()) as Map<String, dynamic>;
      final k = e.evaluateSync(a1, const EvaluateOptions(skipMcm: true));
      expect((k['results']['gum']['kragten']['combined_standard_uncertainty'] as num).toDouble(), closeTo(0.8633, 0.00005));
      final bad = e.template('model')..remove('model');
      expect(e.validate(bad), contains('model'));
      expect(() => e.evaluateSync(bad, const EvaluateOptions()), throwsA(isA<EngineException>()));
    });
  });

  testWidgets('open a library example and calculate it', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1200, 900));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await pumpUntilFound(tester, find.textContaining('Example library (71'));
    expect(find.textContaining('Example library (71'), findsOneWidget);

    await tester.enterText(find.byKey(const Key('search')), 'quam-a1-cadmium-standard-volume');
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
    await pumpUntilFound(tester, find.textContaining('Example library (71'));
    await tester.enterText(find.byKey(const Key('search')), 'nist-tn1900-e11');
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

  testWidgets('top-down forms show and edit the raw data', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1400, 1000));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await pumpUntilFound(tester, find.textContaining('Example library (71'));
    await tester.enterText(find.byKey(const Key('search')), 'nordtest-8-bod-pt');
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
}
