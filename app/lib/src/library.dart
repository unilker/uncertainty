import 'dart:convert';

import 'package:flutter/services.dart' show rootBundle;

/// One validated example of the library bundled with the application.
class LibraryEntry {
  LibraryEntry(this.path, this.project);
  final String path;
  final Map<String, dynamic> project;

  Map<String, dynamic> get metadata => (project['metadata'] as Map).cast<String, dynamic>();
  String get id => (metadata['id'] ?? path).toString();
  String? get field => metadata['field'] as String?;
  bool get isTopDown => project.containsKey('topdown');
  bool get isProficiency => project.containsKey('proficiency');
  bool get isCalibration => project.containsKey('calibration');
  bool get isConformity => project.containsKey('conformity') && !project.containsKey('model');
  bool get isPrecision => project.containsKey('precision_experiment');
  bool get isMultivariate => project.containsKey('multivariate');
  bool get isControlChart => project.containsKey('control_chart');
  bool get isSampling => project.containsKey('sampling');
  bool get isErrorFunction => project.containsKey('error_function');
  bool get isConsensus => project.containsKey('consensus');
  bool get isMulticomponent => project.containsKey('multicomponent_conformity');

  /// Short description of the method for the library list.
  String get methodLabel => isTopDown
      ? 'top-down'
      : isProficiency
      ? 'ISO 13528'
      : isCalibration
      ? 'ISO 6143'
      : isConformity
      ? 'JCGM 106'
      : isPrecision
      ? 'ISO 5725-2'
      : isMultivariate
      ? 'JCGM 102'
      : isControlChart
      ? 'TR 569'
      : isSampling
      ? 'UfS'
      : isErrorFunction
      ? 'cg-18 C'
      : isConsensus
      ? 'NICOB'
      : isMulticomponent
      ? 'JCGM 106'
      : 'GUM + MCM';

  /// Title, description and id in both languages, for searching.
  String get searchText =>
      [id, jsonEncode(metadata['title']), jsonEncode(metadata['description']), (metadata['tags'] as List?)?.join(' ') ?? ''].join(' ').toLowerCase();
}

/// Loads assets/library.json (written by tool/bundle_library.dart).
Future<List<LibraryEntry>> loadLibrary() async {
  // Read once per screen; not cached by the bundle (a cached future can
  // outlive the zone that created it, e.g. between widget tests).
  final text = await rootBundle.loadString('assets/library.json', cache: false);
  final items = jsonDecode(text) as List;
  return [for (final i in items) LibraryEntry((i as Map)['path'] as String, (i['project'] as Map).cast<String, dynamic>())];
}

/// A deep copy of a JSON value, so that editing a project never changes the
/// library entry it came from.
T deepCopy<T>(T value) => jsonDecode(jsonEncode(value)) as T;
