// Writes the beginner's guide as Markdown files for reading outside the
// application:
//
//   dart run tool/write_learning_docs.dart      (from the app directory)
//
// docs/guide/beginner/<lang>/ receives the chapters of assets/guide/<lang>/,
// the generated chapter "Learning with the examples" and examples.md, which
// explains every library example step by step with its lesson. Links between
// chapters and to examples are rewritten to links between the files.

import 'dart:convert';
import 'dart:io';

import 'package:uncertainty_app/src/learn/explain.dart';
import 'package:uncertainty_app/src/learn/lessons.dart';

String rewrite(String md) => md
    .replaceAllMapped(RegExp(r'\]\(example:([\w.\-]+)\)'), (m) => '](examples.md#${m.group(1)})')
    .replaceAllMapped(RegExp(r'\]\((\d\d-[\w-]+)(#[\w-]+)?\)'), (m) => '](${m.group(1)}.md${m.group(2) ?? ''})')
    // Heading anchors {#id} become HTML anchors, which every Markdown viewer understands.
    .replaceAllMapped(RegExp(r'^(#{1,4}) (.*?) \{#([\w-]+)\}$', multiLine: true), (m) => '<a id="${m.group(3)}"></a>\n\n${m.group(1)} ${m.group(2)}');

void main() {
  final lessons = parseLessons(File('assets/guide/lessons.json').readAsStringSync());
  final library = [for (final i in jsonDecode(File('assets/library.json').readAsStringSync()) as List) ((i as Map)['project'] as Map).cast<String, dynamic>()];
  String id(Map<String, dynamic> p) => (p['metadata'] as Map)['id'] as String;
  String text(Object? v, String lang) => v is Map ? (v[lang] ?? v['en']).toString() : '${v ?? ''}';

  for (final lang in ['en', 'tr']) {
    final tr = lang == 'tr';
    final out = Directory('../docs/guide/beginner/$lang')..createSync(recursive: true);
    final index = StringBuffer()
      ..writeln(tr ? '# Sıfırdan belirsizlik: başlangıç kılavuzu' : '# Uncertainty from scratch: the beginner\'s guide')
      ..writeln()
      ..writeln(
        tr ? 'Bu dosyalar `tool/write_learning_docs.dart` ile uygulamanın içindeki kılavuzdan (`app/assets/guide/`) üretilir; elle düzenlemeyin.' : 'These files are written by `tool/write_learning_docs.dart` from the guide inside the application (`app/assets/guide/`); do not edit them by hand.',
      )
      ..writeln();
    for (final ch in guideChapters) {
      final md = File('assets/guide/$lang/$ch.md').readAsStringSync();
      File('${out.path}/$ch.md').writeAsStringSync(rewrite(md));
      index.writeln('- [${md.split('\n').first.substring(2)}]($ch.md)');
    }
    final examples = examplesGuide(lessons, library, lang);
    File('${out.path}/$examplesChapter.md').writeAsStringSync(rewrite(examples));
    index.writeln('- [${examples.split('\n').first.substring(2)}]($examplesChapter.md)');
    index.writeln('- [${tr ? 'Bütün örnekler adım adım' : 'All examples step by step'}](examples.md)');
    File('${out.path}/README.md').writeAsStringSync(index.toString());

    // Every example, grouped by lesson in the order of the levels.
    final b = StringBuffer()
      ..writeln(tr ? '# Bütün örnekler adım adım' : '# All examples step by step')
      ..writeln()
      ..writeln(
        tr
            ? 'Kütüphanedeki ${library.length} örneğin her biri, uygulamanın **Genel** sekmesinde gösterilen açıklamayla. Örnekler derslerine ve düzeylerine göre gruplanmıştır.'
            : 'Each of the ${library.length} examples of the library, with the explanation shown on the **Overview** tab of the application. The examples are grouped by lesson and level.',
      );
    for (final level in [1, 2, 3]) {
      for (final l in lessons.items.where((l) => l.level == level)) {
        final group = library.where((p) => lessons.forId(id(p)) == l).toList();
        if (group.isEmpty) continue;
        b
          ..writeln()
          ..writeln('## ${l.titleIn(lang)} (${levelName(level, lang)})')
          ..writeln()
          ..writeln(l.inLang(lang));
        for (final p in group) {
          final meta = p['metadata'] as Map;
          final explanation = explainProject(p, lang).replaceAllMapped(RegExp(r'^### ', multiLine: true), (_) => '#### ');
          b
            ..writeln()
            ..writeln('<a id="${id(p)}"></a>')
            ..writeln()
            ..writeln('### ${text(meta['title'], lang)}')
            ..writeln()
            ..writeln('`${id(p)}`')
            ..writeln()
            ..writeln(rewrite(explanation));
        }
      }
    }
    File('${out.path}/examples.md').writeAsStringSync(b.toString());
    stdout.writeln('wrote ${out.path}');
  }
}
