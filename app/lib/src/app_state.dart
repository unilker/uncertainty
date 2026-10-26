import 'package:flutter/widgets.dart';

import 'strings.dart';

/// Interface language, shared by all screens.
class AppState extends ChangeNotifier {
  AppState([this._lang = 'en']);

  String _lang;
  String get lang => _lang;
  S get s => S(_lang);

  void toggleLanguage() {
    _lang = _lang == 'en' ? 'tr' : 'en';
    notifyListeners();
  }

  void setLanguage(String lang) {
    if (lang != _lang) {
      _lang = lang;
      notifyListeners();
    }
  }
}

/// Provides the [AppState] to the widget tree.
class AppScope extends InheritedNotifier<AppState> {
  const AppScope({super.key, required AppState state, required super.child}) : super(notifier: state);

  static AppState of(BuildContext context) => context.dependOnInheritedWidgetOfExactType<AppScope>()!.notifier!;
}
