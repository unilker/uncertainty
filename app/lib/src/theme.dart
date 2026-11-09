// Colours and theme of the interface.
//
// The TÜBİTAK design (navy and red) is used on Windows first; the other
// platforms keep the earlier blue theme until the design is approved. Build
// with --dart-define=UC_TUBITAK_DESIGN=true to see it on any platform.

import 'dart:io';

import 'package:flutter/material.dart';

/// Whether the TÜBİTAK design is used.
final bool tubitakDesign = Platform.isWindows || const bool.fromEnvironment('UC_TUBITAK_DESIGN');

/// Brand colours. Red follows CMYK 0/100/100/0 of the UME logo guide; the
/// navy is the dark blue of the institute's presentations. Replace both with
/// the values of the corporate identity guide if they differ.
abstract final class Brand {
  static const navy = Color(0xFF1B2F5E);
  static const navyDark = Color(0xFF0F1D40);
  static const navyLight = Color(0xFF2E4A8C);
  static const red = Color(0xFFD0101C);
  static const redBright = Color(0xFFE30613);
  static const page = Color(0xFFF4F6FA);
  static const line = Color(0xFFDDE2EC);
  static const muted = Color(0xFF5A6478);
}

ThemeData lightTheme() {
  if (!tubitakDesign) return ThemeData(colorSchemeSeed: const Color(0xFF2B6CB0), useMaterial3: true);
  final scheme = ColorScheme.fromSeed(seedColor: Brand.navy).copyWith(
    primary: Brand.navy,
    onPrimary: Colors.white,
    primaryContainer: const Color(0xFFDDE5F5),
    onPrimaryContainer: Brand.navyDark,
    secondary: Brand.red,
    onSecondary: Colors.white,
    secondaryContainer: const Color(0xFFFCE4E5),
    onSecondaryContainer: const Color(0xFF5C0509),
    tertiary: Brand.navyLight,
    onTertiary: Colors.white,
    tertiaryContainer: const Color(0xFFE6ECF8),
    onTertiaryContainer: Brand.navyDark,
    surface: Colors.white,
    onSurface: const Color(0xFF14192A),
    onSurfaceVariant: Brand.muted,
    surfaceContainerLowest: Colors.white,
    surfaceContainerLow: Brand.page,
    surfaceContainer: const Color(0xFFEEF1F7),
    surfaceContainerHigh: const Color(0xFFE8ECF4),
    surfaceContainerHighest: const Color(0xFFE2E7F0),
    outline: const Color(0xFFB5BDCC),
    outlineVariant: Brand.line,
  );
  return _common(scheme, Brand.page);
}

ThemeData darkTheme() {
  if (!tubitakDesign) {
    return ThemeData(colorSchemeSeed: const Color(0xFF2B6CB0), brightness: Brightness.dark, useMaterial3: true);
  }
  final scheme = ColorScheme.fromSeed(seedColor: Brand.navy, brightness: Brightness.dark).copyWith(
    primary: const Color(0xFFAFC2EE),
    onPrimary: Brand.navyDark,
    primaryContainer: Brand.navyLight,
    onPrimaryContainer: Colors.white,
    secondary: const Color(0xFFFF8A80),
    onSecondary: const Color(0xFF5C0509),
    secondaryContainer: const Color(0xFF7A0A10),
    onSecondaryContainer: const Color(0xFFFFDAD7),
    surface: const Color(0xFF111827),
    surfaceContainerLow: const Color(0xFF0B1220),
    outlineVariant: const Color(0xFF2A3550),
  );
  return _common(scheme, const Color(0xFF0B1220));
}

ThemeData _common(ColorScheme c, Color background) {
  final dark = c.brightness == Brightness.dark;
  final base = ThemeData(colorScheme: c, useMaterial3: true, scaffoldBackgroundColor: background);
  final text = base.textTheme;
  return base.copyWith(
    appBarTheme: AppBarTheme(
      backgroundColor: dark ? const Color(0xFF0F1D40) : Brand.navy,
      foregroundColor: Colors.white,
      elevation: 0,
      scrolledUnderElevation: 0,
      centerTitle: false,
      titleTextStyle: text.titleLarge?.copyWith(color: Colors.white, fontWeight: FontWeight.w600, letterSpacing: 0.2),
      iconTheme: const IconThemeData(color: Colors.white),
      actionsIconTheme: const IconThemeData(color: Colors.white),
      shape: const Border(bottom: BorderSide(color: Brand.redBright, width: 3)),
    ),
    cardTheme: CardThemeData(
      color: c.surface,
      elevation: 0,
      margin: EdgeInsets.zero,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: c.outlineVariant),
      ),
    ),
    textTheme: text.copyWith(
      titleLarge: text.titleLarge?.copyWith(color: c.primary, fontWeight: FontWeight.w600),
      titleMedium: text.titleMedium?.copyWith(fontWeight: FontWeight.w600),
    ),
    filledButtonTheme: FilledButtonThemeData(
      style: FilledButton.styleFrom(backgroundColor: c.primary, foregroundColor: c.onPrimary),
    ),
    tabBarTheme: TabBarThemeData(
      labelColor: dark ? c.primary : Colors.white,
      unselectedLabelColor: dark ? c.onSurfaceVariant : const Color(0xFFC9D3EA),
      // Above the red line of the app bar, which is drawn over the bottom 3 px.
      indicator: UnderlineTabIndicator(
        borderSide: BorderSide(color: dark ? Brand.redBright : Colors.white, width: 3),
        insets: const EdgeInsets.only(bottom: 3),
      ),
      indicatorSize: TabBarIndicatorSize.tab,
      tabAlignment: TabAlignment.start,
      dividerColor: Colors.transparent,
    ),
    expansionTileTheme: ExpansionTileThemeData(
      backgroundColor: c.surface,
      collapsedBackgroundColor: c.surface,
      textColor: c.primary,
      collapsedTextColor: c.onSurface,
      iconColor: c.secondary,
      collapsedIconColor: c.primary,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
        side: BorderSide(color: c.outlineVariant),
      ),
      collapsedShape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(10),
        side: BorderSide(color: c.outlineVariant),
      ),
    ),
    inputDecorationTheme: InputDecorationTheme(
      filled: true,
      fillColor: c.surface,
      border: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: BorderSide(color: c.outlineVariant),
      ),
      enabledBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: BorderSide(color: c.outlineVariant),
      ),
      focusedBorder: OutlineInputBorder(
        borderRadius: BorderRadius.circular(10),
        borderSide: BorderSide(color: c.primary, width: 2),
      ),
    ),
    dividerTheme: DividerThemeData(color: c.outlineVariant),
    floatingActionButtonTheme: FloatingActionButtonThemeData(backgroundColor: dark ? c.secondaryContainer : Brand.red, foregroundColor: Colors.white),
    snackBarTheme: SnackBarThemeData(backgroundColor: dark ? c.surfaceContainerHighest : Brand.navyDark),
  );
}
