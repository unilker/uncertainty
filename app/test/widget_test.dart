// Tests of the interface. The engine library must be built first:
//   cargo build --release -p uncertainty-ffi

import 'dart:io';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:uncertainty_app/main.dart';
import 'package:uncertainty_app/src/app_state.dart';
import 'package:uncertainty_app/src/engine.dart';
import 'package:uncertainty_app/src/format.dart';

void main() {
  setUpAll(() {
    engineLibraryPath = Platform.environment['UNCERTAINTY_FFI'] ?? '${Directory.current.path}/../target/release/libuncertainty_ffi.so';
  });

  group('format', () {
    test('numbers and percentages', () {
      expect(fmtNum(1002.69972, 6, 'en'), '1002.70');
      expect(fmtNum(1002.69972, 6, 'tr'), '1002,70');
      expect(fmtNum(9.999, 3, 'en'), '10.0');
      expect(fmtNum(2.5e-7, 2, 'en'), '2.5 × 10⁻⁷');
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
      final bad = e.template('model')..remove('model');
      expect(e.validate(bad), contains('model'));
      expect(() => e.evaluateSync(bad, const EvaluateOptions()), throwsA(isA<EngineException>()));
    });
  });

  testWidgets('open a library example and calculate it', (tester) async {
    await tester.binding.setSurfaceSize(const Size(1200, 900));
    await tester.pumpWidget(UncertaintyApp(state: AppState('en')));
    await tester.runAsync(() => Future<void>.delayed(const Duration(milliseconds: 200)));
    await tester.pumpAndSettle();
    expect(find.textContaining('Example library (60'), findsOneWidget);

    await tester.enterText(find.byKey(const Key('search')), 'quam-a1-cadmium-standard-volume');
    await tester.pumpAndSettle();
    await tester.tap(find.text('Cadmium standard with the volume sub-model'));
    await tester.pumpAndSettle();
    expect(find.text('Input quantities'), findsNothing); // on the overview tab
    expect(find.textContaining('QUAM Example A1'), findsWidgets);

    await tester.runAsync(() async {
      await tester.tap(find.byKey(const Key('run')));
      await Future<void>.delayed(const Duration(seconds: 3));
    });
    await tester.pumpAndSettle();
    expect(find.textContaining('c_Cd = (1002.7 ± 1.7) mg/L'), findsOneWidget);

    // The language switch re-labels the interface.
    await tester.tap(find.text('TR'));
    await tester.pumpAndSettle();
    expect(find.text('Sonuçlar'), findsOneWidget);
  });
}
