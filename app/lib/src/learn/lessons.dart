// Lessons of the library examples (assets/guide/lessons.json, written by
// tools/guide/lessons.py) and the generated chapter that lists the examples by
// level. Pure Dart, so that tool/write_learning_docs.dart can use it too.

import 'dart:convert';

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

/// Names of the levels.
String levelName(int level, String lang) => switch (level) {
  1 => lang == 'tr' ? 'Başlangıç' : 'Beginner',
  2 => lang == 'tr' ? 'Orta' : 'Intermediate',
  _ => lang == 'tr' ? 'İleri' : 'Advanced',
};

/// The generated chapter that lists the examples of the library by level,
/// with the lesson of each group.
String examplesGuide(Lessons lessons, List<Map<String, dynamic>> library, String lang) {
  final tr = lang == 'tr';
  final b = StringBuffer();
  b.writeln(tr ? '# 13. Örneklerle öğrenmek' : '# 13. Learning with the examples');
  b.writeln();
  b.writeln(
    tr
        ? 'Kütüphanedeki ${library.length} örneğin her biri yayımlanmış bir kılavuzdan, standarttan ya da makaleden alınmıştır ve program her değişiklikte onları yeniden hesaplar. Aşağıda örnekler zorluk düzeyine göre gruplanmıştır. Her grubun başında o grubun ne öğrettiği yazar; bir örneğin adına tıklayınca örnek açılır. Açılan örneğin **Genel** sekmesinde, örneğin adım adım açıklaması vardır.'
        : 'Each of the ${library.length} examples of the library comes from a published guide, standard or paper, and the program recomputes them at every change. Below, the examples are grouped by level. Each group starts with what it teaches; click the name of an example to open it. The **Overview** tab of the opened example explains it step by step.',
  );
  b.writeln();
  b.writeln(
    tr
        ? '> **İpucu:** Yeni başlıyorsanız Başlangıç düzeyindeki ilk gruplarla başlayın, her örnekte önce **Hesapla** düğmesine basın, sonra sonuçları açıklamayla karşılaştırın.'
        : '> **Tip:** If you are new, start with the first groups of the Beginner level; in each example press **Calculate** first, then compare the results with the explanation.',
  );
  String id(Map<String, dynamic> p) => ((p['metadata'] as Map?)?['id'] ?? '').toString();
  final byLesson = <Lesson, List<Map<String, dynamic>>>{};
  final other = <Map<String, dynamic>>[];
  for (final e in library) {
    final l = lessons.forId(id(e));
    if (l == null) {
      other.add(e);
    } else {
      byLesson.putIfAbsent(l, () => []).add(e);
    }
  }
  String title(Map<String, dynamic> e) {
    final t = (e['metadata'] as Map?)?['title'];
    return (t is Map ? (t[lang] ?? t['en']) : t).toString();
  }

  for (final level in [1, 2, 3]) {
    final group = lessons.items.where((l) => l.level == level && byLesson.containsKey(l)).toList();
    if (group.isEmpty) continue;
    b.writeln();
    b.writeln('## ${tr ? 'Düzey' : 'Level'} $level: ${levelName(level, lang)} {#level-$level}');
    for (final l in group) {
      b.writeln();
      b.writeln('### ${l.titleIn(lang)}');
      b.writeln();
      b.writeln(l.inLang(lang));
      b.writeln();
      for (final e in byLesson[l]!) {
        b.writeln('- [${title(e)}](example:${id(e)})');
      }
    }
  }
  if (other.isNotEmpty) {
    b.writeln();
    b.writeln(tr ? '## Diğer örnekler' : '## Other examples');
    b.writeln();
    for (final e in other) {
      b.writeln('- [${title(e)}](example:${id(e)})');
    }
  }
  return b.toString();
}
