// Writes assets/library.json from the example library of the repository
// (../library), so that the application ships every validated example.
//
//   dart run tool/bundle_library.dart
//
// The Rust test suite checks that the bundle matches the library.

import 'dart:convert';
import 'dart:io';

void main() {
  final root = Directory('../library');
  final files = root
      .listSync(recursive: true)
      .whereType<File>()
      .where((f) => f.path.endsWith('.json'))
      .toList()
    ..sort((a, b) => a.path.compareTo(b.path));
  final items = [
    for (final f in files)
      {
        'path': 'library/${f.path.substring(root.path.length + 1).replaceAll('\\', '/')}',
        'project': jsonDecode(f.readAsStringSync()),
      }
  ];
  File('assets/library.json').writeAsStringSync('${const JsonEncoder.withIndent(' ').convert(items)}\n');
  stdout.writeln('assets/library.json: ${items.length} projects');
}
