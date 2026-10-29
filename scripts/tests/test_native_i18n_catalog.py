#!/usr/bin/env python3
"""Source-wide native GTK translation inventory; no network or native-speaker signoff."""
from pathlib import Path
import re
import unittest

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / "native_desktop" / "src"
MODULES = (
    "about", "client_errors", "history", "image", "menu_extra", "operation_notices",
    "panel_cards", "panel_details", "panel_status", "reveal_notices",
    "row_empty", "settings", "settings_errors", "settings_residual",
    "settings_state", "settings_warning", "source_preparation",
    "transfer_rows", "tray_chrome", "visibility",
)
LOCALES = (
    "ru", "es", "pt", "de", "fr", "it", "pl", "hi", "zh-Hant", "ja",
    "nl", "sv", "tr", "ko", "bn", "th", "id", "fil", "ar",
)
LITERAL = re.compile(r'"((?:\\.|[^"\\])*)"')
TABLE = re.compile(r'const KEYS: \[&str; (\d+)\] = \[([\s\S]*?)\];')
UI_CALL = re.compile(r'(?:crate::)?i18n::label\(\s*"((?:\\.|[^"\\])*)"\s*,?\s*\)')


def catalog():
    source_keys = set()
    for module in MODULES:
        source = (SRC / "i18n" / f"{module}.rs").read_text(encoding="utf-8")
        match = TABLE.search(source)
        if match is None:
            raise AssertionError(f"No key table in {module}")
        keys = [item.group(1) for item in LITERAL.finditer(match.group(2))]
        if len(keys) != int(match.group(1)):
            raise AssertionError(f"Invalid key count in {module}")
        yield module, source, keys
        source_keys.update(keys)


def rows(source, locale):
    pattern = re.compile(
        re.escape('"' + locale + '"') + r'\s*=>\s*&\[([\s\S]*?)\],'
    )
    match = pattern.search(source)
    if match is None:
        raise AssertionError(f"Missing translation row {locale}")
    return [item.group(1) for item in LITERAL.finditer(match.group(1))]


class NativeI18nCatalogTests(unittest.TestCase):
    def test_given_all_static_catalogs_when_reviewed_then_every_locale_has_complete_nonempty_rows(self):
        unique = set()
        for module, source, keys in catalog():
            with self.subTest(module=module):
                unique.update(keys)
                for locale in LOCALES:
                    with self.subTest(locale=locale):
                        translated = rows(source, locale)
                        self.assertEqual(len(translated), len(keys))
                        for source_key, value in zip(keys, translated):
                            self.assertTrue(value.strip(), f"{module}/{locale}/{source_key!r}")
                            self.assertNotIn("\x00", value)
                            # Bidi overrides can impersonate labels even in a fixed catalog.
                            self.assertFalse(any(c in value for c in ("\u202a", "\u202b", "\u202d", "\u202e", "\u202c")))
        self.assertGreaterEqual(len(unique), 200)

    def test_given_about_theme_when_metadata_is_drawn_then_contrast_uses_full_foreground(self):
        # A themed solid foreground remains readable in both light and dark KDE.
        css = (ROOT / "native_desktop" / "ui" / "panel.css").read_text(encoding="utf-8")
        block = re.search(r"\\.rqs-about-muted\\s*\\{([^}]+)\\}", css)
        self.assertIsNotNone(block, "About metadata CSS selector is required")
        rules = dict(
            item.strip().split(":", 1)
            for item in block.group(1).split(";")
            if ":" in item
        )
        self.assertEqual(rules.get("color", "").strip(), "@theme_fg_color")
        self.assertNotIn("opacity", rules)
        self.assertNotIn("alpha(", block.group(1))

    def test_given_static_ui_label_calls_when_a_source_key_is_used_then_it_has_a_catalog_entry(self):
        keys = set()
        for _, _, entries in catalog():
            keys.update(entries)
        core = (SRC / "i18n.rs").read_text(encoding="utf-8")
        for group in ("MENU_KEYS", "REVIEW_KEYS", "COMPOSE_KEYS"):
            match = re.search(
                rf'const {group}: \[&str; \d+\] = \[([\s\S]*?)\];', core
            )
            self.assertIsNotNone(match, group)
            keys.update(item.group(1) for item in LITERAL.finditer(match.group(1)))
        keys.update(("Language", "System default"))
        paths = [SRC / "panel.rs", SRC / "main.rs", SRC / "tray.rs"]
        paths.extend((SRC / "panel").rglob("*.rs"))
        missing = []
        for path in paths:
            source = path.read_text(encoding="utf-8").split("#[cfg(test)]")[0]
            for match in UI_CALL.finditer(source):
                if match.group(1) not in keys:
                    missing.append(f"{path.relative_to(SRC)}: {match.group(1)}")
        self.assertEqual(missing, [], "UI literals must not silently fall back to English")


if __name__ == "__main__":
    unittest.main()
