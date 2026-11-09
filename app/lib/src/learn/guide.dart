// Content of the beginner's guide "Uncertainty from scratch" and the lessons
// of the library examples (assets/guide/).

import 'dart:convert';

import 'package:flutter/services.dart' show rootBundle;

/// The chapters of the guide, in order. Each is a Markdown file
/// `assets/guide/<lang>/<id>.md` whose first line is "# Title".
const guideChapters = [
  '01-start-here',
  '02-what-is-uncertainty',
  '03-key-words',
  '04-first-calculation',
  '05-where-numbers-come-from',
  '06-reading-results',
  '07-monte-carlo',
  '08-choosing-a-project-type',
  '09-laboratory-data',
  '10-special-evaluations',
  '11-reporting',
  '12-mistakes-and-questions',
];

/// Id of the chapter that is generated from the lessons of the examples.
const examplesChapter = '13-learning-with-examples';

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

/// What a group of library examples teaches, and how difficult it is.
class Lesson {
  Lesson(this.prefix, this.level, this.title, this.text);
  final String prefix;

  /// Name of the group of examples.
  final Map<String, String> title;

  /// 1 beginner, 2 intermediate, 3 advanced.
  final int level;
  final Map<String, String> text;

  String inLang(String lang) => text[lang] ?? text['en'] ?? '';
  String titleIn(String lang) => title[lang] ?? title['en'] ?? prefix;
}

class Lessons {
  Lessons(this.items);
  final List<Lesson> items;

  /// The lesson with the longest prefix of [id].
  Lesson? forId(String id) {
    Lesson? best;
    for (final l in items) {
      if ((id == l.prefix || id.startsWith('${l.prefix}-') || id.startsWith(l.prefix)) && (best == null || l.prefix.length > best.prefix.length)) best = l;
    }
    return best;
  }
}

Lessons parseLessons(String json) {
  final list = jsonDecode(json) as List;
  return Lessons([
    for (final e in list)
      Lesson(
        (e as Map)['prefix'] as String,
        (e['level'] as num).toInt(),
        {'en': (e['title'] as Map)['en'] as String, 'tr': (e['title'] as Map)['tr'] as String},
        {'en': e['en'] as String, 'tr': e['tr'] as String},
      ),
  ]);
}

Future<Lessons> loadLessons() async => parseLessons(await rootBundle.loadString('assets/guide/lessons.json', cache: false));

/// Names of the levels.
String levelName(int level, String lang) => switch (level) {
  1 => lang == 'tr' ? 'Başlangıç' : 'Beginner',
  2 => lang == 'tr' ? 'Orta' : 'Intermediate',
  _ => lang == 'tr' ? 'İleri' : 'Advanced',
};
