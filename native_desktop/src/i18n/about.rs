//! Compact, locally trusted About text modeled after RepoTicker's single-page layout.
//! Preserve the original fork provenance and GPL without presenting a Credits notebook.

pub(super) const KEYS: [&str; 5] = [
    "Version",
    "Author:",
    "Based on the fork",
    "Open source · GNU GPLv3",
    "About rQuickShare Native",
];

pub(super) fn translated(locale: &str, source: &'static str) -> &'static str {
    let Some(index) = KEYS.iter().position(|key| *key == source) else {
        return source;
    };
    let row: &[&str; 5] = match locale {
        "ru" => &[
            "Версия",
            "Автор:",
            "На основе форка",
            "Открытый исходный код · GNU GPLv3",
            "О rQuickShare Native",
        ],
        "es" => &[
            "Versión",
            "Autor:",
            "Basado en el fork",
            "Código abierto · GNU GPLv3",
            "Acerca de rQuickShare Native",
        ],
        "pt" => &[
            "Versão",
            "Autor:",
            "Baseado no fork",
            "Código aberto · GNU GPLv3",
            "Sobre o rQuickShare Native",
        ],
        "de" => &[
            "Version",
            "Autor:",
            "Basierend auf dem Fork",
            "Open Source · GNU GPLv3",
            "Über rQuickShare Native",
        ],
        "fr" => &[
            "Version",
            "Auteur :",
            "Basé sur le fork",
            "Code source ouvert · GNU GPLv3",
            "À propos de rQuickShare Native",
        ],
        "it" => &[
            "Versione",
            "Autore:",
            "Basato sul fork",
            "Codice aperto · GNU GPLv3",
            "Informazioni su rQuickShare Native",
        ],
        "pl" => &[
            "Wersja",
            "Autor:",
            "Na podstawie forka",
            "Otwarte źródła · GNU GPLv3",
            "O rQuickShare Native",
        ],
        "hi" => &[
            "संस्करण",
            "लेखक:",
            "इस फ़ोर्क पर आधारित",
            "ओपन सोर्स · GNU GPLv3",
            "rQuickShare Native के बारे में",
        ],
        "zh-Hant" => &[
            "版本",
            "作者：",
            "基於此分支版本",
            "開放原始碼 · GNU GPLv3",
            "關於 rQuickShare Native",
        ],
        "ja" => &[
            "バージョン",
            "作者：",
            "次のフォークを使用",
            "オープンソース · GNU GPLv3",
            "rQuickShare Native について",
        ],
        "nl" => &[
            "Versie",
            "Auteur:",
            "Gebaseerd op de fork",
            "Open source · GNU GPLv3",
            "Over rQuickShare Native",
        ],
        "sv" => &[
            "Version",
            "Skapad av:",
            "Baserat på förgreningen",
            "Öppen källkod · GNU GPLv3",
            "Om rQuickShare Native",
        ],
        "tr" => &[
            "Sürüm",
            "Yazar:",
            "Temel alınan çatal",
            "Açık kaynak · GNU GPLv3",
            "rQuickShare Native Hakkında",
        ],
        "ko" => &[
            "버전",
            "개발자:",
            "기반 포크",
            "오픈 소스 · GNU GPLv3",
            "rQuickShare Native 정보",
        ],
        "bn" => &[
            "সংস্করণ",
            "লেখক:",
            "এই ফর্কের ভিত্তিতে",
            "উন্মুক্ত উৎস · GNU GPLv3",
            "rQuickShare Native সম্পর্কে",
        ],
        "th" => &[
            "เวอร์ชัน",
            "ผู้พัฒนา:",
            "พัฒนาจากฟอร์ก",
            "โอเพนซอร์ส · GNU GPLv3",
            "เกี่ยวกับ rQuickShare Native",
        ],
        "id" => &[
            "Versi",
            "Pengembang:",
            "Berdasarkan fork",
            "Sumber terbuka · GNU GPLv3",
            "Tentang rQuickShare Native",
        ],
        "fil" => &[
            "Bersyon",
            "May-akda:",
            "Batay sa fork",
            "Bukas na source · GNU GPLv3",
            "Tungkol sa rQuickShare Native",
        ],
        "ar" => &[
            "الإصدار",
            "المطوّر:",
            "مبني على التفريع",
            "مفتوح المصدر · GNU GPLv3",
            "حول rQuickShare Native",
        ],
        _ => &KEYS,
    };
    row[index]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn given_about_keys_when_localized_then_all_twenty_languages_keep_license_and_attribution_context(
    ) {
        for locale in LOCALES {
            for key in KEYS {
                let value = translated(locale, key);
                assert!(!value.trim().is_empty(), "{locale}: {key}");
                assert!(
                    !value.contains('<') && !value.contains('{'),
                    "{locale}: {key}"
                );
            }
            assert!(translated(locale, KEYS[3]).contains("GNU GPLv3"));
            assert!(translated(locale, KEYS[4]).contains("rQuickShare Native"));
            assert_ne!(translated(locale, KEYS[0]), translated(locale, KEYS[1]));
        }
        for unsupported in ["unknown", "pt-BR", "zh-Hans"] {
            for key in KEYS {
                assert_eq!(translated(unsupported, key), key);
            }
        }
    }
}
