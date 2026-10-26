// Uncertainty Calculations (Belirsizlik Hesaplamaları) — graphical interface.
//
// Copyright (C) 2026 İlker Ün, TÜBİTAK UME. GPL-3.0-or-later with additional
// attribution terms (ADDITIONAL-TERMS.md).

import 'dart:convert';
import 'dart:io';
import 'dart:ui' show PlatformDispatcher;

import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

import 'src/app_state.dart';
import 'src/screens/home_screen.dart';
import 'src/screens/project_screen.dart';

/// Command line: `uncertainty_app [project.json] [--run] [--lang en|tr]`.
void main(List<String> args) {
  final system = PlatformDispatcher.instance.locale.languageCode;
  var lang = system == 'tr' ? 'tr' : 'en';
  String? file;
  var run = false;
  for (var i = 0; i < args.length; i++) {
    switch (args[i]) {
      case '--run':
        run = true;
      case '--lang' when i + 1 < args.length:
        lang = args[++i] == 'tr' ? 'tr' : 'en';
      default:
        if (!args[i].startsWith('-')) file = args[i];
    }
  }
  Map<String, dynamic>? project;
  if (file != null) {
    try {
      project = jsonDecode(File(file).readAsStringSync()) as Map<String, dynamic>;
    } on Object {
      project = null;
    }
  }
  runApp(UncertaintyApp(state: AppState(lang), project: project, projectPath: file, runOnOpen: run));
}

class UncertaintyApp extends StatelessWidget {
  const UncertaintyApp({super.key, required this.state, this.project, this.projectPath, this.runOnOpen = false});
  final AppState state;

  /// A project to open at start, e.g. given on the command line.
  final Map<String, dynamic>? project;
  final String? projectPath;
  final bool runOnOpen;

  @override
  Widget build(BuildContext context) {
    return AppScope(
      state: state,
      child: ListenableBuilder(
        listenable: state,
        builder: (context, _) {
          const seed = Color(0xFF2B6CB0);
          return MaterialApp(
            title: state.s['app'],
            debugShowCheckedModeBanner: false,
            locale: Locale(state.lang),
            supportedLocales: const [Locale('en'), Locale('tr')],
            localizationsDelegates: const [GlobalMaterialLocalizations.delegate, GlobalWidgetsLocalizations.delegate, GlobalCupertinoLocalizations.delegate],
            theme: ThemeData(colorSchemeSeed: seed, useMaterial3: true),
            darkTheme: ThemeData(colorSchemeSeed: seed, brightness: Brightness.dark, useMaterial3: true),
            home: project == null ? const HomeScreen() : ProjectScreen(project: project!, filePath: projectPath, runOnOpen: runOnOpen),
          );
        },
      ),
    );
  }
}
