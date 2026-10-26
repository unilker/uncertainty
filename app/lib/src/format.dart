// Number formatting for the interface: significant digits, ×10ⁿ notation
// for very small or large magnitudes, decimal comma in Turkish (as in the
// reports produced by the engine).

import 'dart:math' as math;

const _sup = {'-': '⁻', '0': '⁰', '1': '¹', '2': '²', '3': '³', '4': '⁴', '5': '⁵', '6': '⁶', '7': '⁷', '8': '⁸', '9': '⁹'};

String _superscript(int e) => e.toString().split('').map((c) => _sup[c] ?? c).join();

/// [x] with [sig] significant digits.
String fmtNum(num? x, int sig, String lang) {
  if (x == null) return '—';
  final v = x.toDouble();
  String s;
  if (v.isNaN) {
    s = '—';
  } else if (v.isInfinite) {
    s = '∞';
  } else if (v == 0) {
    s = '0';
  } else {
    var e = (math.log(v.abs()) / math.ln10).floor();
    final scale = math.pow(10, sig - 1 - e).toDouble();
    if ((v.abs() * scale).round() >= math.pow(10, sig)) e += 1;
    if (e >= -4 && e < 7) {
      final decimals = math.max(0, sig - 1 - e);
      s = v.toStringAsFixed(decimals);
    } else {
      final m = v / math.pow(10, e);
      s = '${m.toStringAsFixed(math.max(0, sig - 1))} × 10${_superscript(e)}';
    }
  }
  return lang == 'tr' ? s.replaceAll('.', ',') : s;
}

/// A fraction as a percentage: "95.45 %" in English, "%95,45" in Turkish.
String fmtPct(num? fraction, int sig, String lang) {
  if (fraction == null) return '—';
  final v = fmtNum(fraction * 100, sig, lang);
  return lang == 'tr' ? '%$v' : '$v %';
}

/// A number as typed by the user (shortest representation).
String fmtGiven(num? x) {
  if (x == null) return '';
  final d = x.toDouble();
  if (d == d.roundToDouble() && d.abs() < 1e15) return d.toInt().toString();
  return d.toString();
}

/// [fmtGiven] with the decimal comma in Turkish.
String fmtGivenIn(num? x, String lang) => lang == 'tr' ? fmtGiven(x).replaceAll('.', ',') : fmtGiven(x);

/// Parses a number typed by the user; accepts a decimal comma.
double? parseNum(String s) {
  final t = s.trim().replaceAll(',', '.');
  if (t.isEmpty) return null;
  return double.tryParse(t);
}

/// Parses whitespace-, semicolon- or newline-separated numbers. Commas are
/// decimal separators (Turkish) unless followed by a space.
List<double>? parseList(String s) {
  final tokens = s.split(RegExp(r'[\s;]+|,\s+')).where((t) => t.isNotEmpty);
  final out = <double>[];
  for (final t in tokens) {
    final v = parseNum(t);
    if (v == null) return null;
    out.add(v);
  }
  return out;
}

/// Parses rows of two numbers (one row per line).
List<List<double>>? parsePairs(String s) {
  final out = <List<double>>[];
  for (final line in s.split('\n')) {
    if (line.trim().isEmpty) continue;
    final v = parseList(line);
    if (v == null || v.length != 2) return null;
    out.add(v);
  }
  return out;
}
