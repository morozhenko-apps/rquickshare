"""Exhaustive local contracts for native launcher and Dolphin metadata."""
import configparser
import copy
import importlib.util
from pathlib import Path
import re
import tempfile
import unittest


spec = importlib.util.spec_from_file_location(
    "native_deb", Path(__file__).parents[1] / "native_deb.py"
)
package = importlib.util.module_from_spec(spec)
spec.loader.exec_module(package)

LOCALES = (
    "ru", "es", "pt_PT", "de", "fr", "it", "pl", "hi", "zh_TW", "zh_HK",
    "ja", "nl", "sv", "tr", "ko", "bn", "th", "id", "fil", "ar",
)
ASSET_PATHS = {
    "application": "share/applications/io.github.morozhenko_apps.RQuickShare.desktop",
    "service": "share/kio/servicemenus/io.github.morozhenko_apps.RQuickShare.Send.desktop",
}
# Independent expectations: never derive the oracle from the production checker.
SECTIONS = (
    ("application", "Desktop Entry", "Comment", {
        "Type": "Application",
        "Name": "rQuickShare Native",
        "Comment": "Share files and text with nearby devices",
        "Exec": "/usr/bin/rquickshare",
        "Icon": "rquickshare",
        "Terminal": "false",
        "Categories": "Network;FileTransfer;",
        "StartupNotify": "false",
    }),
    ("service", "Desktop Entry", "Name", {
        "Type": "Service",
        "Name": "Send with rQuickShare",
        "MimeType": "application/octet-stream;inode/directory;",
        "Actions": "Send;",
        "X-KDE-Protocols": "file",
        "X-KDE-MinNumberOfUrls": "1",
        "X-KDE-MaxNumberOfUrls": "256",
        "X-KDE-Priority": "TopLevel",
    }),
    ("service", "Desktop Action Send", "Name", {
        "Name": "Send with rQuickShare…",
        "Icon": "document-send",
        "Exec": "rquickshare send -- %U",
    }),
)


class NativeDesktopAssetsTests(unittest.TestCase):
    def setUp(self):
        self.original = {
            name: package.read_desktop_entry(package.ASSETS / relative)
            for name, relative in ASSET_PATHS.items()
        }

    def fresh(self):
        return copy.deepcopy(self.original)

    def validate(self, entries):
        package.validate_localized_desktop_entries(
            entries["application"], entries["service"]
        )

    def assert_rejected(self, entries, message):
        with self.assertRaisesRegex(ValueError, "^" + re.escape(message) + "$"):
            self.validate(entries)

    def test_given_fixed_fields_when_any_is_missing_changed_or_added_then_reject(self):
        # Arrange: independently enumerate every fixed field in every section.
        for asset, group, _, fields in SECTIONS:
            for key, expected in fields.items():
                self.assertEqual(self.original[asset][group][key], expected)
                with self.subTest(asset=asset, group=group, key=key, mutation="missing"):
                    entries = self.fresh()
                    del entries[asset][group][key]
                    # Act / Assert: reject the missing field, not another defect.
                    self.assert_rejected(entries, f"Unexpected {group} desktop fields or locales")
                for value in ("", "unexpected", expected + " "):
                    with self.subTest(asset=asset, group=group, key=key, value=value):
                        entries = self.fresh()
                        entries[asset][group][key] = value
                        self.assert_rejected(entries, f"Modified {group} launch contract")
            for key in ("Hidden", "NoDisplay", "TryExec", "X-KDE-AuthorizeAction"):
                with self.subTest(asset=asset, group=group, extra=key):
                    entries = self.fresh()
                    entries[asset][group][key] = "unexpected"
                    self.assert_rejected(entries, f"Unexpected {group} desktop fields or locales")

        for key, values in (
            ("X-KDE-MinNumberOfUrls", ("0", "2", "-1", "01", "1.0", "2147483648", " 1", "1 ")),
            ("X-KDE-MaxNumberOfUrls", ("-1", "0", "1", "255", "257", "0256", "256.0", "2147483648", " 256", "256 ")),
        ):
            for value in values:
                with self.subTest(key=key, value=value):
                    entries = self.fresh()
                    entries["service"]["Desktop Entry"][key] = value
                    self.assert_rejected(entries, "Modified Desktop Entry launch contract")

    def test_given_desktop_groups_when_missing_renamed_extra_or_reordered_then_reject(self):
        # Arrange: exhaust the two ordered group contracts, independently of fields.
        layouts = (
            ("application", ()),
            ("application", ("Renamed",)),
            ("application", ("Desktop Entry", "Extra")),
            ("application", ("Desktop Entry", "Desktop Action Send")),
            ("service", ()),
            ("service", ("Desktop Entry",)),
            ("service", ("Desktop Action Send",)),
            ("service", ("Renamed", "Desktop Action Send")),
            ("service", ("Desktop Entry", "Renamed")),
            ("service", ("Desktop Entry", "Desktop Action Send", "Extra")),
            ("service", ("Desktop Action Send", "Desktop Entry")),
        )
        for asset, groups in layouts:
            with self.subTest(asset=asset, groups=groups):
                entries = self.fresh()
                for group in entries[asset].sections():
                    entries[asset].remove_section(group)
                for group in groups:
                    entries[asset][group] = dict(self.original[asset][group]) if group in self.original[asset] else {}
                # Act / Assert.
                kind = "application" if asset == "application" else "Dolphin"
                self.assert_rejected(entries, f"Invalid {kind} desktop groups")

    def test_given_locale_fields_when_any_is_missing_or_an_alias_is_added_then_reject(self):
        # Arrange / Act / Assert: every locale is required in each translated slot.
        unsupported = ("pt", "pt_BR", "zh", "zh_CN", "zh_Hans", "en", "en_US", "PT_pt", "ru_RU", "zh-Hant")
        for asset, group, key, _ in SECTIONS:
            for locale in LOCALES:
                with self.subTest(asset=asset, group=group, missing=locale):
                    entries = self.fresh()
                    del entries[asset][group][f"{key}[{locale}]"]
                    self.assert_rejected(entries, f"Unexpected {group} desktop fields or locales")
            for locale in unsupported:
                with self.subTest(asset=asset, group=group, extra=locale):
                    entries = self.fresh()
                    entries[asset][group][f"{key}[{locale}]"] = "Extra rQuickShare caption"
                    self.assert_rejected(entries, f"Unexpected {group} desktop fields or locales")

    def test_given_valid_unicode_when_each_locale_is_checked_then_preserve_text_and_commands(self):
        # Arrange: literal expectations include printable whitespace boundaries.
        self.assertEqual(package.DESKTOP_LOCALES, frozenset(LOCALES))
        self.validate(self.original)
        for locale in LOCALES:
            for text in ("ASCII ~ text", "café العربية 中文 क\u200dष", "a\u00a0b", "a\u202fb"):
                with self.subTest(locale=locale, text=text):
                    entries = self.fresh()
                    header = text + " rQuickShare"
                    entries["application"]["Desktop Entry"][f"Comment[{locale}]"] = text
                    entries["service"]["Desktop Entry"][f"Name[{locale}]"] = header
                    entries["service"]["Desktop Action Send"][f"Name[{locale}]"] = header + "…"
                    # Act / Assert: validation must neither alter text nor commands.
                    self.validate(entries)
                    self.assertEqual(entries["application"]["Desktop Entry"][f"Comment[{locale}]"], text)
                    self.assertEqual(entries["service"]["Desktop Entry"][f"Name[{locale}]"], header)
                    self.assertEqual(entries["service"]["Desktop Action Send"][f"Name[{locale}]"], header + "…")
                    self.assertEqual(entries["application"]["Desktop Entry"]["Exec"], "/usr/bin/rquickshare")
                    self.assertEqual(entries["service"]["Desktop Action Send"]["Exec"], "rquickshare send -- %U")

    def test_given_blank_or_padded_translations_when_each_locale_is_checked_then_reject(self):
        for asset, group, key, _ in SECTIONS:
            for locale in LOCALES:
                # Arrange: each mutation starts from a fresh, valid pair of files.
                original = self.original[asset][group][f"{key}[{locale}]"]
                variants = ("", " ", "\t", "\r", "\n", "\u00a0", "\u202f", " " + original, original + " ", "\u00a0" + original, original + "\u00a0", " \t" + original, original + "\n")
                for value in variants:
                    with self.subTest(asset=asset, group=group, locale=locale, value=value):
                        entries = self.fresh()
                        entries[asset][group][f"{key}[{locale}]"] = value
                        # Act / Assert.
                        self.assert_rejected(entries, f"Empty or padded desktop translation: {locale}")

    def test_given_control_or_bidi_characters_when_each_translation_is_checked_then_reject(self):
        # Arrange: include both endpoints of every prohibited interval, not a sample.
        forbidden = (*range(0x20), 0x7F, *range(0x202A, 0x202F), *range(0x2066, 0x206A))
        for asset, group, key, _ in SECTIONS:
            for locale in LOCALES:
                for codepoint in forbidden:
                    with self.subTest(asset=asset, group=group, locale=locale, codepoint=hex(codepoint)):
                        entries = self.fresh()
                        # Keep controls internal so trim validation cannot mask them.
                        entries[asset][group][f"{key}[{locale}]"] = "Text" + chr(codepoint) + " rQuickShare"
                        # Act / Assert: require the safety rejection specifically.
                        self.assert_rejected(entries, f"Unsafe desktop translation: {locale}")

    def test_given_field_codes_or_escapes_when_any_translation_contains_them_then_reject(self):
        # Arrange / Act / Assert: prefix, infix and suffix in every localized field.
        for asset, group, key, _ in SECTIONS:
            for locale in LOCALES:
                for marker in ("%", "\\"):
                    for value in (marker + "rQuickShare", "rQuick" + marker + "Share", "rQuickShare" + marker):
                        with self.subTest(asset=asset, group=group, locale=locale, value=value):
                            entries = self.fresh()
                            entries[asset][group][f"{key}[{locale}]"] = value
                            self.assert_rejected(entries, f"Escaped desktop translation: {locale}")

    def test_given_dolphin_captions_when_attribution_or_action_relationship_changes_then_reject(self):
        for locale in LOCALES:
            # Arrange: preserve valid translation fields to reach the caption checks.
            original = self.original["service"]["Desktop Entry"][f"Name[{locale}]"]
            for header in ("Send with another product", "Send with rquickshare", original + "…"):
                with self.subTest(locale=locale, header=header):
                    entries = self.fresh()
                    entries["service"]["Desktop Entry"][f"Name[{locale}]"] = header
                    entries["service"]["Desktop Action Send"][f"Name[{locale}]"] = header + "…"
                    # Act / Assert.
                    self.assert_rejected(entries, f"Incorrect Dolphin caption: {locale}")
            for action in (original, original + "...", original + "……", "Different rQuickShare…", "…" + original):
                with self.subTest(locale=locale, action=action):
                    entries = self.fresh()
                    entries["service"]["Desktop Action Send"][f"Name[{locale}]"] = action
                    self.assert_rejected(entries, f"Incorrect Dolphin actionable caption: {locale}")

    def test_given_duplicate_keys_or_groups_when_parsed_then_reject_every_shipped_slot(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "duplicate.desktop"
            for asset, group, _, _ in SECTIONS:
                for key in self.original[asset][group]:
                    with self.subTest(asset=asset, group=group, duplicate=key):
                        # Arrange: duplicate the chosen key inside its actual group.
                        lines = []
                        for section in self.original[asset].sections():
                            lines.append(f"[{section}]")
                            for name, value in self.original[asset][section].items():
                                lines.append(f"{name}={value}")
                                if section == group and name == key:
                                    lines.append(f"{name}=Duplicate")
                        path.write_text("\n".join(lines) + "\n", encoding="utf-8")
                        # Act / Assert: actual parser, no mocked config reader.
                        with self.assertRaises(configparser.DuplicateOptionError):
                            package.read_desktop_entry(path)
                with self.subTest(asset=asset, duplicate_group=group):
                    source = (package.ASSETS / ASSET_PATHS[asset]).read_text(encoding="utf-8")
                    path.write_text(source + f"\n[{group}]\nUnexpected=value\n", encoding="utf-8")
                    with self.assertRaises(configparser.DuplicateSectionError):
                        package.read_desktop_entry(path)

    def test_given_case_sensitive_literal_or_malformed_files_when_parsed_then_preserve_or_fail_explicitly(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "fixture.desktop"
            # Arrange: case and interpolation are security-relevant parser inputs.
            path.write_text("[Desktop Entry]\nName=rQuickShare\nname=Other\nComment=literal %U \\s\n", encoding="utf-8")
            # Act / Assert: both names survive; field codes are not interpolated.
            parsed = package.read_desktop_entry(path)
            self.assertEqual(dict(parsed["Desktop Entry"]), {
                "Name": "rQuickShare", "name": "Other", "Comment": "literal %U \\s",
            })
            for content, error in (
                ("Name=rQuickShare\n", configparser.MissingSectionHeaderError),
                ("[Desktop Entry\nName=rQuickShare\n", configparser.MissingSectionHeaderError),
                ("[Desktop Entry]\nName\n", configparser.ParsingError),
            ):
                with self.subTest(content=content):
                    path.write_text(content, encoding="utf-8")
                    with self.assertRaises(error):
                        package.read_desktop_entry(path)
            path.write_bytes(b"[Desktop Entry]\nName=\xff\n")
            with self.subTest(case="invalid UTF-8"), self.assertRaises(UnicodeDecodeError):
                package.read_desktop_entry(path)
            with self.subTest(case="missing file"), self.assertRaises(FileNotFoundError):
                package.read_desktop_entry(Path(temporary) / "missing.desktop")


if __name__ == "__main__":
    unittest.main()
