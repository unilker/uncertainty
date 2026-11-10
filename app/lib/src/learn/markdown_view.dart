// A small Markdown renderer for the guide and the explanations of projects.
//
// Supported: headings (#, ##, ###, ####) with an optional anchor `{#id}`,
// paragraphs, **bold**, *italic*, `code`, links [text](target), bullet and
// numbered lists, tables, block quotes as call-out boxes (the first bold word
// chooses the colour: Tip/İpucu, Note/Not, Warning/Dikkat, Example/Örnek,
// Try/Deneyin), fenced code blocks and horizontal rules.
//
// Link targets: `example:ID` opens a library example, `chapter-id` or
// `chapter-id#anchor` opens a guide chapter, `http...` an external page; the
// owner decides through [MarkdownView.onLink].

import 'package:flutter/gestures.dart';
import 'package:flutter/material.dart';

import '../theme.dart';

/// One block of a Markdown text.
sealed class MdBlock {
  const MdBlock();
}

class MdHeading extends MdBlock {
  const MdHeading(this.level, this.text, this.anchor);
  final int level;
  final String text;
  final String? anchor;
}

class MdParagraph extends MdBlock {
  const MdParagraph(this.text);
  final String text;
}

class MdList extends MdBlock {
  const MdList(this.items, {required this.ordered});
  final List<String> items;
  final bool ordered;
}

class MdTable extends MdBlock {
  const MdTable(this.header, this.rows);
  final List<String> header;
  final List<List<String>> rows;
}

class MdQuote extends MdBlock {
  const MdQuote(this.blocks);
  final List<MdBlock> blocks;
}

class MdCode extends MdBlock {
  const MdCode(this.text);
  final String text;
}

class MdRule extends MdBlock {
  const MdRule();
}

final _heading = RegExp(r'^(#{1,4})\s+(.*?)\s*(?:\{#([\w-]+)\})?\s*$');
final _bullet = RegExp(r'^\s*[-*]\s+(.*)$');
final _numbered = RegExp(r'^\s*\d+[.)]\s+(.*)$');

/// Splits a Markdown text into blocks.
List<MdBlock> parseMarkdown(String text) {
  final lines = text.replaceAll('\r\n', '\n').split('\n');
  final out = <MdBlock>[];
  var i = 0;
  bool isTableLine(String l) => l.trimLeft().startsWith('|');
  bool startsBlock(String l) =>
      l.trim().isEmpty ||
      _heading.hasMatch(l) ||
      _bullet.hasMatch(l) ||
      _numbered.hasMatch(l) ||
      isTableLine(l) ||
      l.startsWith('>') ||
      l.startsWith('```') ||
      l.trim() == '---';
  while (i < lines.length) {
    final line = lines[i];
    if (line.trim().isEmpty) {
      i++;
      continue;
    }
    final h = _heading.firstMatch(line);
    if (h != null) {
      out.add(MdHeading(h.group(1)!.length, h.group(2)!, h.group(3)));
      i++;
      continue;
    }
    if (line.trim() == '---') {
      out.add(const MdRule());
      i++;
      continue;
    }
    if (line.startsWith('```')) {
      final buf = <String>[];
      i++;
      while (i < lines.length && !lines[i].startsWith('```')) {
        buf.add(lines[i]);
        i++;
      }
      i++;
      out.add(MdCode(buf.join('\n')));
      continue;
    }
    if (line.startsWith('>')) {
      final buf = <String>[];
      while (i < lines.length && lines[i].startsWith('>')) {
        buf.add(lines[i].replaceFirst(RegExp(r'^>\s?'), ''));
        i++;
      }
      out.add(MdQuote(parseMarkdown(buf.join('\n'))));
      continue;
    }
    if (isTableLine(line)) {
      final rows = <List<String>>[];
      while (i < lines.length && isTableLine(lines[i])) {
        final l = lines[i].trim();
        i++;
        if (RegExp(r'^\|[\s:|-]+\|$').hasMatch(l)) continue;
        var cells = l.substring(1, l.endsWith('|') ? l.length - 1 : l.length).split('|').map((c) => c.trim()).toList();
        rows.add(cells);
      }
      if (rows.isNotEmpty) out.add(MdTable(rows.first, rows.sublist(1)));
      continue;
    }
    final b = _bullet.firstMatch(line);
    final n = _numbered.firstMatch(line);
    if (b != null || n != null) {
      final ordered = b == null;
      final items = <String>[];
      while (i < lines.length) {
        final l = lines[i];
        final m = ordered ? _numbered.firstMatch(l) : _bullet.firstMatch(l);
        if (m != null) {
          items.add(m.group(1)!);
          i++;
        } else if (l.trim().isNotEmpty && l.startsWith('  ') && items.isNotEmpty) {
          items[items.length - 1] = '${items.last} ${l.trim()}';
          i++;
        } else {
          break;
        }
      }
      out.add(MdList(items, ordered: ordered));
      continue;
    }
    final buf = <String>[line.trim()];
    i++;
    while (i < lines.length && !startsBlock(lines[i])) {
      buf.add(lines[i].trim());
      i++;
    }
    out.add(MdParagraph(buf.join(' ')));
  }
  return out;
}

/// The text of a Markdown string without its markup, for searching.
String plainText(String md) => md.replaceAll(RegExp(r'[*`#>|]'), ' ').replaceAllMapped(RegExp(r'\[([^\]]*)\]\([^)]*\)'), (m) => m.group(1)!);

/// Renders Markdown. Headings with an anchor get the key from [anchors] (if
/// given), so that the owner can scroll to them.
class MarkdownView extends StatefulWidget {
  const MarkdownView(this.text, {super.key, this.onLink, this.anchors, this.compact = false});
  final String text;
  final void Function(String target)? onLink;
  final Map<String, GlobalKey>? anchors;

  /// Smaller headings, for use inside a card.
  final bool compact;

  @override
  State<MarkdownView> createState() => _MarkdownViewState();
}

class _MarkdownViewState extends State<MarkdownView> {
  final _recognizers = <TapGestureRecognizer>[];
  late List<MdBlock> _blocks = parseMarkdown(widget.text);

  @override
  void didUpdateWidget(MarkdownView old) {
    super.didUpdateWidget(old);
    if (old.text != widget.text) _blocks = parseMarkdown(widget.text);
  }

  @override
  void dispose() {
    for (final r in _recognizers) {
      r.dispose();
    }
    super.dispose();
  }

  TextSpan _inline(String text, TextStyle? base) {
    final theme = Theme.of(context);
    final spans = <InlineSpan>[];
    final re = RegExp(r'\*\*(.+?)\*\*|(?<![\w*])\*(?!\s)(.+?)(?<!\s)\*(?![\w*])|`([^`]+)`|\[([^\]]+)\]\(([^)\s]+)\)');
    var pos = 0;
    for (final m in re.allMatches(text)) {
      if (m.start > pos) spans.add(TextSpan(text: text.substring(pos, m.start)));
      if (m.group(1) != null) {
        spans.add(_inline(m.group(1)!, base?.copyWith(fontWeight: FontWeight.w700)));
      } else if (m.group(2) != null) {
        spans.add(_inline(m.group(2)!, base?.copyWith(fontStyle: FontStyle.italic)));
      } else if (m.group(3) != null) {
        spans.add(
          TextSpan(
            text: m.group(3),
            style: base?.copyWith(fontFamily: 'monospace', backgroundColor: theme.colorScheme.surfaceContainerHigh, fontSize: (base.fontSize ?? 14) * 0.95),
          ),
        );
      } else {
        final target = m.group(5)!;
        final r = TapGestureRecognizer()..onTap = () => widget.onLink?.call(target);
        _recognizers.add(r);
        final link = base?.copyWith(color: theme.colorScheme.secondary, decoration: TextDecoration.underline, decorationColor: theme.colorScheme.secondary);
        spans.add(
          TextSpan(
            children: [
              if (target.startsWith('example:'))
                WidgetSpan(
                  alignment: PlaceholderAlignment.middle,
                  child: Padding(
                    padding: const EdgeInsets.only(right: 3),
                    child: Icon(Icons.play_circle_outline, size: (base?.fontSize ?? 14) + 2, color: theme.colorScheme.secondary),
                  ),
                ),
              TextSpan(text: m.group(4), style: link, recognizer: r, mouseCursor: SystemMouseCursors.click),
            ],
          ),
        );
      }
      pos = m.end;
    }
    if (pos < text.length) spans.add(TextSpan(text: text.substring(pos)));
    return TextSpan(style: base, children: spans);
  }

  Widget _block(MdBlock b, TextStyle body) {
    final theme = Theme.of(context);
    final c = theme.colorScheme;
    switch (b) {
      case MdHeading(:final level, :final text, :final anchor):
        final t = theme.textTheme;
        final style = switch (widget.compact ? level + 1 : level) {
          1 => t.headlineSmall?.copyWith(color: c.primary, fontWeight: FontWeight.w700),
          2 => t.titleLarge?.copyWith(color: c.primary, fontWeight: FontWeight.w700),
          3 => t.titleMedium?.copyWith(color: c.primary, fontWeight: FontWeight.w700),
          _ => t.titleSmall?.copyWith(color: c.primary, fontWeight: FontWeight.w700),
        };
        return Padding(
          key: anchor == null ? null : widget.anchors?.putIfAbsent(anchor, GlobalKey.new),
          padding: EdgeInsets.only(top: level <= 2 ? 18 : 12, bottom: 6),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              if (level == 2)
                Container(
                  width: 4,
                  height: (style?.fontSize ?? 20) * 1.2,
                  margin: const EdgeInsets.only(right: 10),
                  decoration: BoxDecoration(color: Brand.redBright, borderRadius: BorderRadius.circular(2)),
                ),
              Expanded(child: Text.rich(_inline(text, style))),
            ],
          ),
        );
      case MdParagraph(:final text):
        return Padding(padding: const EdgeInsets.only(bottom: 10), child: Text.rich(_inline(text, body)));
      case MdList(:final items, :final ordered):
        return Padding(
          padding: const EdgeInsets.only(bottom: 10, left: 4),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              for (var k = 0; k < items.length; k++)
                Padding(
                  padding: const EdgeInsets.only(bottom: 4),
                  child: Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      SizedBox(
                        width: ordered ? 26 : 18,
                        child: Text(
                          ordered ? '${k + 1}.' : '•',
                          style: body.copyWith(color: c.secondary, fontWeight: FontWeight.w700),
                        ),
                      ),
                      Expanded(child: Text.rich(_inline(items[k], body))),
                    ],
                  ),
                ),
            ],
          ),
        );
      case MdTable(:final header, :final rows):
        final small = body.copyWith(fontSize: (body.fontSize ?? 14) * 0.93);
        return Padding(
          padding: const EdgeInsets.only(bottom: 12),
          child: SingleChildScrollView(
            scrollDirection: Axis.horizontal,
            child: ConstrainedBox(
              constraints: const BoxConstraints(maxWidth: 1000),
              child: Table(
                defaultColumnWidth: const IntrinsicColumnWidth(flex: 1),
                border: TableBorder(
                  horizontalInside: BorderSide(color: c.outlineVariant),
                  top: BorderSide(color: c.primary, width: 1.5),
                  bottom: BorderSide(color: c.primary, width: 1.5),
                ),
                children: [
                  TableRow(
                    decoration: BoxDecoration(color: c.primaryContainer),
                    children: [
                      for (final h in header)
                        Padding(
                          padding: const EdgeInsets.all(8),
                          child: Text.rich(_inline(h, small.copyWith(fontWeight: FontWeight.w700, color: c.onPrimaryContainer))),
                        ),
                    ],
                  ),
                  for (final r in rows)
                    TableRow(
                      children: [
                        for (var k = 0; k < header.length; k++)
                          Padding(padding: const EdgeInsets.all(8), child: Text.rich(_inline(k < r.length ? r[k] : '', small))),
                      ],
                    ),
                ],
              ),
            ),
          ),
        );
      case MdQuote(:final blocks):
        final first = blocks.isNotEmpty && blocks.first is MdParagraph ? (blocks.first as MdParagraph).text.toLowerCase() : '';
        final (Color accent, IconData icon) = switch (first) {
          _ when first.startsWith('**dikkat') || first.startsWith('**warning') || first.startsWith('**caution') => (c.error, Icons.warning_amber_rounded),
          _ when first.startsWith('**deneyin') || first.startsWith('**try') => (c.secondary, Icons.play_circle_outline),
          _ when first.startsWith('**örnek') || first.startsWith('**example') => (c.tertiary, Icons.science_outlined),
          _ => (c.primary, Icons.lightbulb_outline),
        };
        return Container(
          margin: const EdgeInsets.only(bottom: 12),
          padding: const EdgeInsets.fromLTRB(12, 10, 14, 2),
          decoration: BoxDecoration(
            color: accent.withValues(alpha: 0.06),
            borderRadius: BorderRadius.circular(10),
            border: Border(left: BorderSide(color: accent, width: 4)),
          ),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              Padding(
                padding: const EdgeInsets.only(right: 10, top: 1),
                child: Icon(icon, color: accent, size: 22),
              ),
              Expanded(
                child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [for (final x in blocks) _block(x, body)]),
              ),
            ],
          ),
        );
      case MdCode(:final text):
        return Container(
          width: double.infinity,
          margin: const EdgeInsets.only(bottom: 12),
          padding: const EdgeInsets.all(12),
          decoration: BoxDecoration(color: c.surfaceContainerHigh, borderRadius: BorderRadius.circular(8)),
          child: SelectableText(
            text,
            style: body.copyWith(fontFamily: 'monospace', fontSize: (body.fontSize ?? 14) * 0.92),
          ),
        );
      case MdRule():
        return Divider(height: 24, color: c.outlineVariant);
    }
  }

  @override
  Widget build(BuildContext context) {
    for (final r in _recognizers) {
      r.dispose();
    }
    _recognizers.clear();
    final body = (Theme.of(context).textTheme.bodyLarge ?? const TextStyle()).copyWith(height: 1.5);
    return Column(crossAxisAlignment: CrossAxisAlignment.start, children: [for (final b in _blocks) _block(b, body)]);
  }
}
