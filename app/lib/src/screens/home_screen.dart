import 'dart:convert';
import 'dart:io';

import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';

import '../app_state.dart';
import '../engine.dart';
import '../library.dart';
import 'project_screen.dart';

class HomeScreen extends StatefulWidget {
  const HomeScreen({super.key});

  @override
  State<HomeScreen> createState() => _HomeScreenState();
}

class _HomeScreenState extends State<HomeScreen> {
  late final Future<List<LibraryEntry>> _library = loadLibrary();
  String _query = '';

  void _openProject(Map<String, dynamic> project, {String? path, bool fromLibrary = false}) {
    Navigator.of(context).push(
      MaterialPageRoute(
        builder: (_) => ProjectScreen(project: deepCopy(project), filePath: path, fromLibrary: fromLibrary),
      ),
    );
  }

  Future<void> _openFile() async {
    final s = AppScope.of(context).s;
    const group = XTypeGroup(label: 'JSON', extensions: ['json']);
    final file = await openFile(acceptedTypeGroups: [group]);
    if (file == null || !mounted) return;
    try {
      final project = jsonDecode(await File(file.path).readAsString()) as Map<String, dynamic>;
      _openProject(project, path: file.path);
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(content: Text('${s['file_error']}: $e')));
    }
  }

  void _about() {
    final s = AppScope.of(context).s;
    Map<String, dynamic>? v;
    String? error;
    try {
      v = const Engine().version();
    } on Object catch (e) {
      error = e.toString();
    }
    showDialog<void>(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(s['app']),
        content: SelectableText(
          error ??
              '${v!['name']} ${v['version']}\n\nİlker Ün, TÜBİTAK UME\nORCID 0000-0001-5089-4602\n\n'
                  'GPL-3.0-or-later\n\n${s['cite']}:\n${v['citation']}',
        ),
        actions: [TextButton(onPressed: () => Navigator.pop(context), child: Text(s['close']))],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final app = AppScope.of(context);
    final s = app.s;
    return Scaffold(
      appBar: AppBar(
        title: Text(s['app']),
        actions: [
          TextButton(key: const Key('language'), onPressed: app.toggleLanguage, child: Text(app.lang == 'en' ? 'TR' : 'EN')),
          IconButton(tooltip: s['about'], onPressed: _about, icon: const Icon(Icons.info_outline)),
        ],
      ),
      body: FutureBuilder<List<LibraryEntry>>(
        future: _library,
        builder: (context, snap) {
          final entries = snap.data ?? const <LibraryEntry>[];
          final q = _query.trim().toLowerCase();
          final shown = q.isEmpty ? entries : entries.where((e) => e.searchText.contains(q)).toList();
          final fields = <String, List<LibraryEntry>>{};
          for (final e in shown) {
            fields.putIfAbsent(e.field ?? '', () => []).add(e);
          }
          return ListView(
            padding: const EdgeInsets.all(16),
            children: [
              Wrap(
                spacing: 12,
                runSpacing: 12,
                children: [
                  _ActionCard(
                    icon: Icons.functions,
                    title: s['new_model'],
                    subtitle: s['new_model_sub'],
                    onTap: () => _openProject(const Engine().template('model')),
                  ),
                  _ActionCard(
                    icon: Icons.science_outlined,
                    title: s['new_topdown'],
                    subtitle: s['new_topdown_sub'],
                    onTap: () => _openProject(const Engine().template('topdown')),
                  ),
                  _ActionCard(
                    icon: Icons.groups_outlined,
                    title: s['new_pt'],
                    subtitle: s['new_pt_sub'],
                    onTap: () => _openProject(const Engine().template('proficiency')),
                  ),
                  _ActionCard(icon: Icons.folder_open, title: s['open'], subtitle: s['open_sub'], onTap: _openFile),
                ],
              ),
              const SizedBox(height: 24),
              Text('${s['library']} (${entries.length} ${s['examples']})', style: Theme.of(context).textTheme.titleLarge),
              const SizedBox(height: 8),
              TextField(
                key: const Key('search'),
                decoration: InputDecoration(prefixIcon: const Icon(Icons.search), hintText: s['search']),
                onChanged: (v) => setState(() => _query = v),
              ),
              const SizedBox(height: 8),
              if (snap.hasError) Text('${snap.error}'),
              for (final f in fields.entries)
                ExpansionTile(
                  key: PageStorageKey('field-${f.key}-${q.isNotEmpty}'),
                  initiallyExpanded: q.isNotEmpty,
                  title: Text('${s.field(f.key)} (${f.value.length})'),
                  children: [
                    for (final e in f.value)
                      ListTile(
                        title: Text(s.text(e.metadata['title'])),
                        subtitle: Text('${e.id} · ${e.methodLabel}'),
                        trailing: const Icon(Icons.chevron_right),
                        onTap: () => _openProject(e.project, fromLibrary: true),
                      ),
                  ],
                ),
            ],
          );
        },
      ),
    );
  }
}

class _ActionCard extends StatelessWidget {
  const _ActionCard({required this.icon, required this.title, required this.subtitle, required this.onTap});
  final IconData icon;
  final String title;
  final String subtitle;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      width: 300,
      child: Card(
        child: InkWell(
          onTap: onTap,
          child: Padding(
            padding: const EdgeInsets.all(16),
            child: Row(
              children: [
                Icon(icon, size: 32, color: Theme.of(context).colorScheme.primary),
                const SizedBox(width: 16),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(title, style: Theme.of(context).textTheme.titleMedium),
                      Text(subtitle, style: Theme.of(context).textTheme.bodySmall),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
