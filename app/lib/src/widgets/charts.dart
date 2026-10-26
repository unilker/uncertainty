import 'dart:math' as math;

import 'package:flutter/material.dart';

import '../format.dart';

/// Horizontal bars of variance shares (fractions).
class SharesChart extends StatelessWidget {
  const SharesChart({super.key, required this.rows, required this.lang});
  final List<MapEntry<String, double>> rows;
  final String lang;

  @override
  Widget build(BuildContext context) {
    final sorted = rows.where((r) => r.value > 0).toList()..sort((a, b) => b.value.compareTo(a.value));
    if (sorted.isEmpty) return const SizedBox.shrink();
    final max = sorted.first.value;
    final c = Theme.of(context).colorScheme;
    return Column(
      children: [
        for (final r in sorted)
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 3),
            child: Row(children: [
              SizedBox(width: 130, child: Text(r.key, textAlign: TextAlign.end, overflow: TextOverflow.ellipsis)),
              const SizedBox(width: 8),
              Expanded(
                child: Align(
                  alignment: Alignment.centerLeft,
                  child: FractionallySizedBox(
                    widthFactor: math.max(0.002, r.value / max),
                    child: Container(
                      height: 16,
                      decoration: BoxDecoration(color: c.primary, borderRadius: BorderRadius.circular(3)),
                    ),
                  ),
                ),
              ),
              const SizedBox(width: 6),
              SizedBox(width: 64, child: Text(fmtPct(r.value, 3, lang), style: Theme.of(context).textTheme.bodySmall)),
            ]),
          ),
      ],
    );
  }
}

/// Monte Carlo histogram with the GUM density and the coverage intervals.
class HistogramChart extends StatelessWidget {
  const HistogramChart({
    super.key,
    required this.lower,
    required this.binWidth,
    required this.counts,
    required this.trials,
    this.gumPdf,
    this.mcmInterval,
    this.gumInterval,
    required this.lang,
  });
  final double lower;
  final double binWidth;
  final List<int> counts;
  final int trials;
  final List<List<double>>? gumPdf;
  final List<double>? mcmInterval;
  final List<double>? gumInterval;
  final String lang;

  @override
  Widget build(BuildContext context) {
    final c = Theme.of(context).colorScheme;
    return Column(children: [
      SizedBox(
        height: 260,
        width: double.infinity,
        child: CustomPaint(
          painter: _HistogramPainter(this, c, Theme.of(context).textTheme.bodySmall!),
        ),
      ),
      const SizedBox(height: 4),
      Wrap(spacing: 16, children: [
        _Legend(color: c.primaryContainer, label: 'MCM', box: true),
        if (gumPdf != null) _Legend(color: c.tertiary, label: 'GUM'),
      ]),
    ]);
  }
}

class _Legend extends StatelessWidget {
  const _Legend({required this.color, required this.label, this.box = false});
  final Color color;
  final String label;
  final bool box;

  @override
  Widget build(BuildContext context) {
    return Row(mainAxisSize: MainAxisSize.min, children: [
      Container(width: box ? 12 : 18, height: box ? 12 : 3, color: color),
      const SizedBox(width: 6),
      Text(label, style: Theme.of(context).textTheme.bodySmall),
    ]);
  }
}

class _HistogramPainter extends CustomPainter {
  _HistogramPainter(this.h, this.colors, this.textStyle);
  final HistogramChart h;
  final ColorScheme colors;
  final TextStyle textStyle;

  @override
  void paint(Canvas canvas, Size size) {
    if (h.counts.isEmpty || h.binWidth <= 0 || h.trials <= 0) return;
    const pad = 22.0;
    final x0 = h.lower;
    final x1 = h.lower + h.binWidth * h.counts.length;
    final dens = [for (final n in h.counts) n / (h.trials * h.binWidth)];
    var ymax = dens.reduce(math.max);
    for (final p in h.gumPdf ?? const <List<double>>[]) {
      ymax = math.max(ymax, p[1]);
    }
    if (ymax <= 0) return;
    double sx(double x) => (x - x0) / (x1 - x0) * size.width;
    double sy(double y) => size.height - pad - y / ymax * (size.height - pad - 4);
    final bar = Paint()..color = colors.primaryContainer;
    for (var i = 0; i < dens.length; i++) {
      final a = sx(x0 + i * h.binWidth);
      final b = sx(x0 + (i + 1) * h.binWidth);
      canvas.drawRect(Rect.fromLTRB(a, sy(dens[i]), math.max(b - 0.5, a + 0.5), sy(0)), bar);
    }
    final pdf = h.gumPdf;
    if (pdf != null && pdf.isNotEmpty) {
      final path = Path()..moveTo(sx(pdf.first[0]), sy(pdf.first[1]));
      for (final p in pdf.skip(1)) {
        path.lineTo(sx(p[0]), sy(p[1]));
      }
      canvas.drawPath(
          path,
          Paint()
            ..color = colors.tertiary
            ..style = PaintingStyle.stroke
            ..strokeWidth = 2);
    }
    void vline(double x, Color color, double dash) {
      if (x < x0 || x > x1) return;
      final p = Paint()
        ..color = color
        ..strokeWidth = 1.5;
      for (var y = 4.0; y < size.height - pad; y += dash * 2) {
        canvas.drawLine(Offset(sx(x), y), Offset(sx(x), math.min(y + dash, size.height - pad)), p);
      }
    }

    for (final x in h.mcmInterval ?? const <double>[]) {
      vline(x, colors.primary, 5);
    }
    for (final x in h.gumInterval ?? const <double>[]) {
      vline(x, colors.tertiary, 2);
    }
    canvas.drawLine(Offset(0, sy(0)), Offset(size.width, sy(0)), Paint()..color = colors.outline);
    void label(String t, double x, TextAlign align) {
      final tp = TextPainter(text: TextSpan(text: t, style: textStyle), textDirection: TextDirection.ltr)..layout();
      final dx = align == TextAlign.end ? x - tp.width : x;
      tp.paint(canvas, Offset(dx, size.height - pad + 4));
    }

    label(fmtNum(x0, 5, h.lang), 0, TextAlign.start);
    label(fmtNum(x1, 5, h.lang), size.width, TextAlign.end);
  }

  @override
  bool shouldRepaint(covariant _HistogramPainter old) => old.h != h || old.colors != colors;
}
