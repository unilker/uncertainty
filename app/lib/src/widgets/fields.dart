import 'package:flutter/material.dart';

import '../app_state.dart';
import '../format.dart';

/// A text field bound to a key of a JSON object: numbers are parsed (decimal
/// comma accepted), an empty field removes the key.
class NumberField extends StatelessWidget {
  const NumberField({
    super.key,
    required this.map,
    required this.field,
    required this.label,
    required this.onChanged,
    this.integer = false,
    this.width = 180,
  });
  final Map<String, dynamic> map;
  final String field;
  final String label;
  final VoidCallback onChanged;
  final bool integer;
  final double width;

  @override
  Widget build(BuildContext context) {
    final v = map[field];
    final lang = AppScope.of(context).lang;
    return SizedBox(
      width: width,
      child: TextFormField(
        initialValue: v is num ? fmtGivenIn(v, lang) : '',
        decoration: InputDecoration(labelText: label, isDense: true),
        keyboardType: const TextInputType.numberWithOptions(decimal: true, signed: true),
        onChanged: (t) {
          final x = parseNum(t);
          if (t.trim().isEmpty || x == null) {
            map.remove(field);
          } else {
            map[field] = integer ? x.round() : x;
          }
          onChanged();
        },
      ),
    );
  }
}

/// A text field bound to a string key of a JSON object.
class StringField extends StatelessWidget {
  const StringField({
    super.key,
    required this.map,
    required this.field,
    required this.label,
    required this.onChanged,
    this.width,
    this.maxLines = 1,
    this.keepEmpty = false,
    this.monospace = false,
  });
  final Map<String, dynamic> map;
  final String field;
  final String label;
  final VoidCallback onChanged;
  final double? width;
  final int maxLines;
  final bool keepEmpty;
  final bool monospace;

  @override
  Widget build(BuildContext context) {
    final f = TextFormField(
      initialValue: (map[field] ?? '').toString(),
      maxLines: maxLines,
      style: monospace ? const TextStyle(fontFamily: 'monospace') : null,
      decoration: InputDecoration(labelText: label, isDense: true),
      onChanged: (t) {
        if (t.isEmpty && !keepEmpty) {
          map.remove(field);
        } else {
          map[field] = t;
        }
        onChanged();
      },
    );
    return width == null ? f : SizedBox(width: width, child: f);
  }
}

/// A multi-line field holding a list of numbers (one per line).
class ListField extends StatelessWidget {
  const ListField(
      {super.key, required this.map, required this.field, required this.label, required this.onChanged});
  final Map<String, dynamic> map;
  final String field;
  final String label;
  final VoidCallback onChanged;

  @override
  Widget build(BuildContext context) {
    final lang = AppScope.of(context).lang;
    final v = (map[field] as List?)?.map((e) => fmtGivenIn(e as num, lang)).join('\n') ?? '';
    return TextFormField(
      initialValue: v,
      minLines: 3,
      maxLines: 10,
      style: const TextStyle(fontFamily: 'monospace'),
      decoration: InputDecoration(labelText: label, alignLabelWithHint: true),
      onChanged: (t) {
        final list = parseList(t);
        if (list == null || list.isEmpty) {
          map.remove(field);
        } else {
          map[field] = list;
        }
        onChanged();
      },
    );
  }
}

/// A multi-line field holding rows of two numbers.
class PairsField extends StatelessWidget {
  const PairsField(
      {super.key, required this.map, required this.field, required this.label, required this.onChanged});
  final Map<String, dynamic> map;
  final String field;
  final String label;
  final VoidCallback onChanged;

  @override
  Widget build(BuildContext context) {
    final lang = AppScope.of(context).lang;
    final v = (map[field] as List?)
            ?.map((p) => (p as List).map((e) => fmtGivenIn(e as num, lang)).join('\t'))
            .join('\n') ??
        '';
    return TextFormField(
      initialValue: v,
      minLines: 3,
      maxLines: 12,
      style: const TextStyle(fontFamily: 'monospace'),
      decoration: InputDecoration(labelText: label, alignLabelWithHint: true),
      onChanged: (t) {
        final pairs = parsePairs(t);
        if (pairs == null || pairs.isEmpty) {
          map.remove(field);
        } else {
          map[field] = pairs;
        }
        onChanged();
      },
    );
  }
}

/// A section heading.
class Section extends StatelessWidget {
  const Section(this.title, {super.key, this.trailing});
  final String title;
  final Widget? trailing;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.only(top: 16, bottom: 8),
      child: Row(children: [
        Expanded(child: Text(title, style: Theme.of(context).textTheme.titleMedium)),
        ?trailing,
      ]),
    );
  }
}
