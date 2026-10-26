// Binding to the Rust engine (crate `uncertainty-ffi`) through dart:ffi.
//
// Every call passes and returns JSON text; the project file is the document
// model and all computations happen in the validated Rust core.

import 'dart:convert';
import 'dart:ffi';
import 'dart:io';
import 'dart:isolate';

import 'package:ffi/ffi.dart';

typedef _NoArg = Pointer<Utf8> Function();
typedef _OneArg = Pointer<Utf8> Function(Pointer<Utf8>);
typedef _TwoArgs = Pointer<Utf8> Function(Pointer<Utf8>, Pointer<Utf8>);
typedef _FreeC = Void Function(Pointer<Utf8>);
typedef _FreeDart = void Function(Pointer<Utf8>);

/// Error reported by the engine (invalid project, unsupported option, …).
class EngineException implements Exception {
  EngineException(this.message);
  final String message;
  @override
  String toString() => message;
}

/// Location of the engine library, if not the platform default (tests set
/// it; the environment variable UNCERTAINTY_FFI does the same).
String? engineLibraryPath;

/// Opens the engine library for the current platform.
DynamicLibrary _openLibrary() {
  final override = engineLibraryPath ?? Platform.environment['UNCERTAINTY_FFI'];
  if (override != null && override.isNotEmpty) return DynamicLibrary.open(override);
  if (Platform.isLinux || Platform.isAndroid) return DynamicLibrary.open('libuncertainty_ffi.so');
  if (Platform.isWindows) return DynamicLibrary.open('uncertainty_ffi.dll');
  if (Platform.isMacOS) return DynamicLibrary.open('libuncertainty_ffi.dylib');
  // iOS links the static library into the executable.
  return DynamicLibrary.process();
}

class _Bindings {
  _Bindings(DynamicLibrary lib)
    : version = lib.lookupFunction<_NoArg, _NoArg>('uc_version'),
      template = lib.lookupFunction<_OneArg, _OneArg>('uc_template'),
      validate = lib.lookupFunction<_OneArg, _OneArg>('uc_validate'),
      evaluate = lib.lookupFunction<_TwoArgs, _TwoArgs>('uc_evaluate'),
      free = lib.lookupFunction<_FreeC, _FreeDart>('uc_free');

  final _NoArg version;
  final _OneArg template;
  final _OneArg validate;
  final _TwoArgs evaluate;
  final _FreeDart free;

  static _Bindings? _instance;
  static _Bindings get instance => _instance ??= _Bindings(_openLibrary());

  Map<String, dynamic> _take(Pointer<Utf8> p) {
    try {
      return jsonDecode(p.toDartString()) as Map<String, dynamic>;
    } finally {
      free(p);
    }
  }

  Map<String, dynamic> call0(_NoArg f) => _take(f());

  Map<String, dynamic> call1(_OneArg f, String a) {
    final pa = a.toNativeUtf8();
    try {
      return _take(f(pa));
    } finally {
      calloc.free(pa);
    }
  }

  Map<String, dynamic> call2(_TwoArgs f, String a, String b) {
    final pa = a.toNativeUtf8();
    final pb = b.toNativeUtf8();
    try {
      return _take(f(pa, pb));
    } finally {
      calloc.free(pa);
      calloc.free(pb);
    }
  }
}

Map<String, dynamic> _checked(Map<String, dynamic> r) {
  if (r['ok'] != true) throw EngineException((r['error'] ?? 'unknown error').toString());
  return r;
}

/// Options of an evaluation.
class EvaluateOptions {
  const EvaluateOptions({this.lang = 'en', this.skipMcm = false, this.trials, this.seed, this.markdown = false, this.html = false, this.generated});
  final String lang;
  final bool skipMcm;
  final int? trials;
  final int? seed;
  final bool markdown;
  final bool html;
  final String? generated;

  Map<String, dynamic> toJson() => {
    'lang': lang,
    'skip_mcm': skipMcm,
    if (trials != null) 'trials': trials,
    if (seed != null) 'seed': seed,
    'markdown': markdown,
    'html': html,
    if (generated != null) 'generated': generated,
  };
}

/// The engine's public functions.
class Engine {
  const Engine();

  /// Software name, version and citation.
  Map<String, dynamic> version() => _checked(_Bindings.instance.call0(_Bindings.instance.version));

  /// A starting project ("model" or "topdown") as a decoded JSON object.
  Map<String, dynamic> template(String kind) {
    final r = _checked(_Bindings.instance.call1(_Bindings.instance.template, kind));
    return jsonDecode(r['project'] as String) as Map<String, dynamic>;
  }

  /// Null if the project is valid, otherwise the engine's message.
  String? validate(Map<String, dynamic> project) {
    final r = _Bindings.instance.call1(_Bindings.instance.validate, jsonEncode(project));
    return r['ok'] == true ? null : (r['error'] ?? 'invalid project').toString();
  }

  /// Evaluates a project on the calling isolate.
  Map<String, dynamic> evaluateSync(Map<String, dynamic> project, EvaluateOptions options) {
    final b = _Bindings.instance;
    return _checked(b.call2(b.evaluate, jsonEncode(project), jsonEncode(options.toJson())));
  }

  /// Evaluates a project on a background isolate so that long Monte Carlo
  /// runs do not block the interface.
  Future<Map<String, dynamic>> evaluate(Map<String, dynamic> project, EvaluateOptions options) {
    final p = jsonEncode(project);
    final o = jsonEncode(options.toJson());
    final path = engineLibraryPath;
    return Isolate.run(() {
      engineLibraryPath = path;
      final b = _Bindings.instance;
      return _checked(b.call2(b.evaluate, p, o));
    });
  }
}
