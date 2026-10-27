import 'dart:convert';

import 'package:flutter/material.dart';

import '../app_state.dart';
import '../engine.dart';

/// The whole project file as text; applied only when the engine accepts it.
class JsonEditor extends StatefulWidget {
  const JsonEditor({super.key, required this.project, required this.onApply});
  final Map<String, dynamic> project;
  final ValueChanged<Map<String, dynamic>> onApply;

  @override
  State<JsonEditor> createState() => _JsonEditorState();
}

class _JsonEditorState extends State<JsonEditor> {
  late final TextEditingController _text = TextEditingController(text: const JsonEncoder.withIndent('  ').convert(widget.project));
  String? _error;

  @override
  void dispose() {
    _text.dispose();
    super.dispose();
  }

  void _apply() {
    try {
      final v = jsonDecode(_text.text);
      if (v is! Map<String, dynamic>) throw const FormatException('a JSON object is expected');
      final err = const Engine().validate(v);
      if (err != null) {
        setState(() => _error = err);
        return;
      }
      setState(() => _error = null);
      widget.onApply(v);
    } on FormatException catch (e) {
      setState(() => _error = e.message);
    }
  }

  @override
  Widget build(BuildContext context) {
    final s = AppScope.of(context).s;
    return Padding(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              Expanded(child: Text(s['json_help'])),
              FilledButton(onPressed: _apply, child: Text(s['apply'])),
            ],
          ),
          if (_error != null)
            Padding(
              padding: const EdgeInsets.only(top: 8),
              child: Text(_error!, style: TextStyle(color: Theme.of(context).colorScheme.error)),
            ),
          const SizedBox(height: 8),
          Expanded(
            child: TextField(
              controller: _text,
              expands: true,
              maxLines: null,
              style: const TextStyle(fontFamily: 'monospace', fontSize: 13),
              decoration: const InputDecoration(border: OutlineInputBorder()),
            ),
          ),
        ],
      ),
    );
  }
}
