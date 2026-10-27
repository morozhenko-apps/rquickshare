//! Locale-aware, presentation-only dynamic text.
//! Original session metadata and authentication remain unchanged.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cardinal {
    Zero,
    One,
    Two,
    Few,
    Many,
    Other,
}

fn cardinal(locale: &str, count: usize) -> Cardinal {
    if locale == "ar" {
        return match count {
            0 => Cardinal::Zero,
            1 => Cardinal::One,
            2 => Cardinal::Two,
            _ => match count % 100 {
                3..=10 => Cardinal::Few,
                11..=99 => Cardinal::Many,
                _ => Cardinal::Other,
            },
        };
    }
    if locale == "ru" || locale == "pl" {
        let last = count % 10;
        let last_two = count % 100;
        if last == 1 && last_two != 11 && (locale == "ru" || count == 1) {
            return Cardinal::One;
        }
        if (2..=4).contains(&last) && !(12..=14).contains(&last_two) {
            return Cardinal::Few;
        }
        return Cardinal::Many;
    }
    if matches!(locale, "fr" | "hi" | "bn") && count <= 1 {
        return Cardinal::One;
    }
    if count == 1 {
        Cardinal::One
    } else {
        Cardinal::Other
    }
}

/// Count of received files. Integer cardinal categories are intentionally
/// scoped to the values the UI renders (not decimal or ordinal CLDR data).
pub(super) fn file_count_for(locale: &str, count: usize) -> String {
    use Cardinal::{Few, Many, One, Two, Zero};
    let category = cardinal(locale, count);
    let noun = match locale {
        "ru" => match category {
            One => "файл",
            Few => "файла",
            _ => "файлов",
        },
        "es" => {
            if category == One {
                "archivo"
            } else {
                "archivos"
            }
        }
        "pt" => {
            if category == One {
                "ficheiro"
            } else {
                "ficheiros"
            }
        }
        "de" => {
            if category == One {
                "Datei"
            } else {
                "Dateien"
            }
        }
        "fr" => {
            if category == One {
                "fichier"
            } else {
                "fichiers"
            }
        }
        "it" => "file",
        "pl" => match category {
            One => "plik",
            Few => "pliki",
            _ => "plików",
        },
        "hi" => {
            if category == One {
                "फ़ाइल"
            } else {
                "फ़ाइलें"
            }
        }
        "zh-Hant" => "個檔案",
        "ja" => "件のファイル",
        "nl" => {
            if category == One {
                "bestand"
            } else {
                "bestanden"
            }
        }
        "sv" => {
            if category == One {
                "fil"
            } else {
                "filer"
            }
        }
        "tr" => "dosya",
        "ko" => "개 파일",
        "bn" => "ফাইল",
        "th" => "ไฟล์",
        "id" => "berkas",
        "fil" => "file",
        "ar" => match category {
            Zero | Few => "ملفات",
            One => "ملف",
            Two => "ملفان",
            Many => "ملفًا",
            _ => "ملف",
        },
        _ => {
            if category == One {
                "file"
            } else {
                "files"
            }
        }
    };
    format!("{count} {noun}")
}

/// Exact byte count. B is the internationally recognized byte unit symbol.
pub(super) fn bytes_for(bytes: u64) -> String {
    format!("{bytes} B")
}

fn pin_caption(locale: &str) -> &'static str {
    match locale {
        "ru" => "Код",
        "es" | "pt" => "Código",
        "de" | "fr" | "nl" | "fil" => "Code",
        "it" => "Codice",
        "pl" | "sv" | "tr" => "Kod",
        "hi" => "कोड",
        "zh-Hant" => "代碼",
        "ja" => "コード",
        "ko" => "코드",
        "bn" => "কোড",
        "th" => "รหัส",
        "id" => "Kode",
        "ar" => "الرمز",
        _ => "Code",
    }
}

/// Only trusted, validated four ASCII digit display tokens are shown.
/// LRI/PDI preserves digit order within Arabic and mixed-direction captions.
pub(super) fn code_for(locale: &str, pin: &str) -> String {
    if pin.len() != 4 || !pin.bytes().all(|b| b.is_ascii_digit()) {
        return String::new();
    }
    format!("{}: \u{2066}{pin}\u{2069}", pin_caption(locale))
}

fn is_bidi_control(ch: char) -> bool {
    matches!(
        ch,
        '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'
    )
}

/// Display-only projection; it does not change source metadata or paths.
/// Remove embedded direction controls before enclosing remote text in FSI/PDI.
pub(super) fn isolate_untrusted(value: &str) -> String {
    let mut visible = String::with_capacity(value.len() + 6);
    visible.push('\u{2068}');
    visible.extend(value.chars().filter(|ch| !is_bidi_control(*ch)));
    visible.push('\u{2069}');
    visible
}

/// A display-only projection; the original resolved device name remains unmodified.
/// This is not a peer-authentication or verified-route identity statement.
pub(super) fn resolved_name_for(locale: &str, name: &str) -> String {
    if name.is_empty() {
        return String::new();
    }
    let caption = match locale {
        "ru" => "Имя для новых подключений",
        "es" => "Nombre para conexiones nuevas",
        "pt" => "Nome para novas ligações",
        "de" => "Name für neue Verbindungen",
        "fr" => "Nom pour les nouvelles connexions",
        "it" => "Nome per le nuove connessioni",
        "pl" => "Nazwa dla nowych połączeń",
        "hi" => "नए कनेक्शन के लिए नाम",
        "zh-Hant" => "新連線使用的名稱",
        "ja" => "新しい接続で使用する名前",
        "nl" => "Naam voor nieuwe verbindingen",
        "sv" => "Namn för nya anslutningar",
        "tr" => "Yeni bağlantılar için ad",
        "ko" => "새 연결에 사용할 이름",
        "bn" => "নতুন সংযোগের নাম",
        "th" => "ชื่อสำหรับการเชื่อมต่อใหม่",
        "id" => "Nama untuk koneksi baru",
        "fil" => "Pangalan para sa mga bagong koneksyon",
        "ar" => "الاسم المستخدم للاتصالات الجديدة",
        _ => "Name for new connections",
    };
    format!("{caption}: {}", isolate_untrusted(name))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn resolved_name_is_fsi_isolated_without_mutating_the_source() {
        let hostile = "PC\u{2069}\u{202e}<unsafe>\u{202c}.txt";
        for locale in LOCALES {
            let actual = resolved_name_for(locale, hostile);
            assert!(
                actual.contains("\u{2068}PC<unsafe>.txt\u{2069}"),
                "{locale}"
            );
            assert!(!actual.contains('\u{202e}'), "{locale}");
            assert!(!actual.contains('\u{2069}') || actual.ends_with('\u{2069}'));
            assert!(!actual.starts_with(':'));
            assert!(resolved_name_for(locale, "").is_empty(), "{locale}");
        }
        assert!(hostile.contains('\u{202e}'));
        assert_eq!(
            resolved_name_for("en", "Desktop"),
            "Name for new connections: \u{2068}Desktop\u{2069}"
        );
        assert_eq!(
            resolved_name_for("pt", "Desktop"),
            "Nome para novas ligações: \u{2068}Desktop\u{2069}"
        );
        assert_eq!(
            resolved_name_for("ru", "Desktop"),
            "Имя для новых подключений: \u{2068}Desktop\u{2069}"
        );
        assert_eq!(
            resolved_name_for("ar", "Desktop"),
            "الاسم المستخدم للاتصالات الجديدة: \u{2068}Desktop\u{2069}"
        );
        assert_eq!(
            resolved_name_for("pt-BR", "Desktop"),
            resolved_name_for("en", "Desktop")
        );
    }

    #[test]
    fn cardinal_integer_boundaries_keep_russian_polish_and_arabic_distinct() {
        for (count, ru, pl) in [
            (0, "0 файлов", "0 plików"),
            (1, "1 файл", "1 plik"),
            (2, "2 файла", "2 pliki"),
            (5, "5 файлов", "5 plików"),
            (11, "11 файлов", "11 plików"),
            (12, "12 файлов", "12 plików"),
            (21, "21 файл", "21 plików"),
            (22, "22 файла", "22 pliki"),
            (101, "101 файл", "101 plików"),
        ] {
            assert_eq!(file_count_for("ru", count), ru);
            assert_eq!(file_count_for("pl", count), pl);
        }
        for (count, category) in [
            (0, Cardinal::Zero),
            (1, Cardinal::One),
            (2, Cardinal::Two),
            (3, Cardinal::Few),
            (10, Cardinal::Few),
            (11, Cardinal::Many),
            (99, Cardinal::Many),
            (100, Cardinal::Other),
            (103, Cardinal::Few),
        ] {
            assert_eq!(cardinal("ar", count), category);
        }
    }

    #[test]
    fn file_cardinality_is_total_and_carries_exact_numbers() {
        for locale in LOCALES {
            for count in [0, 1, 2, 3, 4, 5, 11, 21, 101, 1000, usize::MAX] {
                let rendered = file_count_for(locale, count);
                assert!(
                    rendered.starts_with(&count.to_string()),
                    "{locale}: {count}"
                );
                assert!(!rendered.trim().is_empty(), "{locale}: {count}");
                assert!(!rendered.contains('<'), "{locale}: {count}");
            }
        }
        assert_eq!(file_count_for("en", 1), "1 file");
        assert_eq!(file_count_for("en", 2), "2 files");
        assert_eq!(file_count_for("fr", 0), "0 fichier");
        assert_eq!(file_count_for("pt", 2), "2 ficheiros");
        assert_eq!(file_count_for("unknown", 2), "2 files");
    }

    #[test]
    fn bytes_keep_precision_and_no_implicit_unit_conversion() {
        for bytes in [0, 1, 1023, 1024, 1025, 1_000_000, u64::MAX] {
            assert_eq!(bytes_for(bytes), format!("{bytes} B"));
        }
    }

    #[test]
    fn pin_is_explicitly_validated_and_ltr_isolated() {
        for locale in LOCALES {
            let shown = code_for(locale, "0123");
            assert!(shown.ends_with(": \u{2066}0123\u{2069}"), "{locale}");
            assert!(!shown.contains('<'), "{locale}");
        }
        assert_eq!(code_for("en", "0123"), "Code: \u{2066}0123\u{2069}");
        assert_eq!(code_for("ar", "0123"), "الرمز: \u{2066}0123\u{2069}");
        for invalid in [
            "",
            "1",
            "123",
            "12345",
            "12 3",
            "١٢٣٤",
            "ab12",
            "1\u{2069}23",
        ] {
            assert!(code_for("en", invalid).is_empty(), "{invalid:?}");
        }
    }

    #[test]
    fn untrusted_bidi_markup_cannot_escape_isolation() {
        let attacker = "report\u{2069}\u{202e}<b>pay</b>\u{202c}\u{2066}.png";
        let actual = isolate_untrusted(attacker);
        assert_eq!(actual, "\u{2068}report<b>pay</b>.png\u{2069}");
        assert!(!actual.contains('\u{202e}'));
        assert_eq!(isolate_untrusted("ملف.txt"), "\u{2068}ملف.txt\u{2069}");
    }
}
