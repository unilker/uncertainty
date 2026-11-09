// Content of the beginner's guide "Uncertainty from scratch" and the lessons
// of the library examples (assets/guide/).

import 'package:flutter/services.dart' show rootBundle;

import 'lessons.dart';

export 'lessons.dart';

/// The chapter (and anchor) that explains a project type, for the help
/// button of a project.
String helpChapterFor(Map<String, dynamic> project) {
  if (project.containsKey('topdown')) return '09-laboratory-data#topdown';
  if (project.containsKey('proficiency')) return '09-laboratory-data#proficiency';
  if (project.containsKey('precision_experiment')) return '09-laboratory-data#precision';
  if (project.containsKey('control_chart')) return '09-laboratory-data#control-chart';
  if (project.containsKey('sampling')) return '09-laboratory-data#sampling';
  if (project.containsKey('calibration')) return '10-special-evaluations#calibration';
  if (project.containsKey('multivariate')) return '10-special-evaluations#multivariate';
  if (project.containsKey('error_function')) return '10-special-evaluations#weighing';
  if (project.containsKey('consensus')) return '10-special-evaluations#consensus';
  if (project.containsKey('conformity') || project.containsKey('multicomponent_conformity')) return '10-special-evaluations#conformity';
  return '04-first-calculation';
}

class GuideChapter {
  GuideChapter(this.id, this.text);
  final String id;
  final String text;

  String get title {
    final first = text.split('\n').firstWhere((l) => l.startsWith('# '), orElse: () => id);
    return first.replaceFirst('# ', '').trim();
  }
}

/// Loads the chapters in [lang] ('en' or 'tr').
Future<List<GuideChapter>> loadGuide(String lang) async {
  final out = <GuideChapter>[];
  for (final id in guideChapters) {
    out.add(GuideChapter(id, await rootBundle.loadString('assets/guide/$lang/$id.md', cache: false)));
  }
  return out;
}

Future<Lessons> loadLessons() async => parseLessons(await rootBundle.loadString('assets/guide/lessons.json', cache: false));
