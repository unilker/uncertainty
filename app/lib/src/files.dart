// Saving files on desktop and mobile platforms.
//
// Desktop platforms have a save dialog. Android and iOS do not: projects are
// saved in the application's documents folder, and files meant to leave the
// device (reports, "Save as") are handed to the system share sheet, from
// which the user can store them in Files/Drive or send them.

import 'dart:io';

import 'package:file_selector/file_selector.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';
import 'package:share_plus/share_plus.dart';

bool get isMobile => Platform.isAndroid || Platform.isIOS;

/// Where to save a new file named [name]: a path chosen in the save dialog
/// (desktop; null if cancelled) or a path in the documents folder (mobile).
Future<String?> chooseSavePath(String name, XTypeGroup group) async {
  if (isMobile) {
    final dir = await getApplicationDocumentsDirectory();
    return p.join(dir.path, name);
  }
  final loc = await getSaveLocation(suggestedName: name, acceptedTypeGroups: [group]);
  return loc?.path;
}

/// Writes [text] to a temporary file named [name] and opens the share sheet
/// (mobile only).
Future<void> shareText(String name, String text) async {
  final dir = await getTemporaryDirectory();
  final file = File(p.join(dir.path, name));
  await file.writeAsString(text);
  await SharePlus.instance.share(ShareParams(files: [XFile(file.path)], title: name));
}

/// Writes [bytes] to a temporary file named [name] and opens the share sheet
/// (mobile only).
Future<void> shareBytes(String name, List<int> bytes) async {
  final dir = await getTemporaryDirectory();
  final file = File(p.join(dir.path, name));
  await file.writeAsBytes(bytes);
  await SharePlus.instance.share(ShareParams(files: [XFile(file.path)], title: name));
}
