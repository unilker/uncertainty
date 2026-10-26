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
            child: Row(
              children: [
                SizedBox(
                  width: 130,
                  child: Text(r.key, textAlign: TextAlign.end, overflow: TextOverflow.ellipsis),
                ),
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
              ],
            ),
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
    return Column(
      children: [
        SizedBox(
          height: 260,
          width: double.infinity,
          child: CustomPaint(painter: _HistogramPainter(this, c, Theme.of(context).textTheme.bodySmall!)),
        ),
        const SizedBox(height: 4),
        Wrap(
          spacing: 16,
          children: [
            _Legend(color: c.primaryContainer, label: 'MCM', box: true),
            if (gumPdf != null) _Legend(color: c.tertiary, label: 'GUM'),
          ],
        ),
      ],
    );
  }
}

class _Legend extends StatelessWidget {
  const _Legend({required this.color, required this.label, this.box = false});
  final Color color;
  final String label;
  final bool box;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Container(width: box ? 12 : 18, height: box ? 12 : 3, color: color),
        const SizedBox(width: 6),
        Text(label, style: Theme.of(context).textTheme.bodySmall),
      ],
    );
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
          ..strokeWidth = 2,
      );
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
      final tp = TextPainter(
        text: TextSpan(text: t, style: textStyle),
        textDirection: TextDirection.ltr,
      )..layout();
      final dx = align == TextAlign.end ? x - tp.width : x;
      tp.paint(canvas, Offset(dx, size.height - pad + 4));
    }

    label(fmtNum(x0, 5, h.lang), 0, TextAlign.start);
    label(fmtNum(x1, 5, h.lang), size.width, TextAlign.end);
  }

  @override
  bool shouldRepaint(covariant _HistogramPainter old) => old.h != h || old.colors != colors;
}

/// z scores of a proficiency test round as bars around zero, with the
/// warning (±2) and action (±3) limits; with a single limit (weighted
/// deviations of a calibration, ±2) the bars beyond it are marked.
class ZScoreChart extends StatelessWidget {
  const ZScoreChart({super.key, required this.rows, required this.lang, this.limits = const [2.0, 3.0]});
  final List<MapEntry<String, double>> rows;
  final String lang;
  final List<double> limits;

  @override
  Widget build(BuildContext context) {
    if (rows.isEmpty) return const SizedBox.shrink();
    final zmax = math.min(10.0, rows.fold<double>(limits.last + 0.5, (m, r) => math.max(m, r.value.abs())));
    final c = Theme.of(context).colorScheme;
    final ticks = [for (final l in limits.reversed) -l, 0.0, ...limits];
    Color bar(double v) {
      final a = v.abs();
      if (limits.length == 1) return a > limits[0] ? c.error : c.primary;
      return a >= limits[1] ? c.error : (a > limits[0] ? c.tertiary : c.primary);
    }

    return LayoutBuilder(
      builder: (context, box) {
        const label = 80.0;
        final w = math.max(100.0, box.maxWidth - label - 8);
        double x(double z) => w * (z.clamp(-zmax, zmax) + zmax) / (2 * zmax);
        return Column(
          children: [
            for (final r in rows)
              SizedBox(
                height: 20,
                child: Row(
                  children: [
                    SizedBox(
                      width: label,
                      child: Text(r.key, textAlign: TextAlign.end, overflow: TextOverflow.ellipsis),
                    ),
                    const SizedBox(width: 8),
                    SizedBox(
                      width: w,
                      child: Stack(
                        children: [
                          for (final z in ticks)
                            Positioned(
                              left: x(z),
                              top: 0,
                              bottom: 0,
                              child: Container(width: 1, color: z == 0 ? c.outline : (limits.length > 1 && z.abs() == limits[0] ? c.tertiary : c.error)),
                            ),
                          Positioned(
                            left: math.min(x(0), x(r.value)),
                            top: 3,
                            bottom: 3,
                            child: Container(width: math.max(1.0, (x(r.value) - x(0)).abs()), color: bar(r.value)),
                          ),
                        ],
                      ),
                    ),
                  ],
                ),
              ),
            Row(
              children: [
                const SizedBox(width: label + 8),
                SizedBox(
                  width: w,
                  height: 16,
                  child: Stack(
                    children: [
                      for (final z in ticks)
                        Positioned(
                          left: math.max(0, x(z) - 8),
                          child: Text(fmtNum(z, 1, lang), style: Theme.of(context).textTheme.bodySmall),
                        ),
                    ],
                  ),
                ),
              ],
            ),
          ],
        );
      },
    );
  }
}
