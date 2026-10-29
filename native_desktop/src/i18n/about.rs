//! Locale-specific About credit stays on the primary page, not a Credits tab.
//! The author and upstream provenance are part of the product attribution.

pub(super) fn description(locale: &str) -> &'static str {
    match locale {
        "ru" => "Разработано Станиславом Мосейко на основе форка Martichou/rquickshare.",
        "es" => "Desarrollado por Stanislav Moseyko a partir del fork Martichou/rquickshare.",
        "pt" => "Desenvolvido por Stanislav Moseyko com base no fork Martichou/rquickshare.",
        "de" => "Entwickelt von Stanislav Moseyko auf Basis des Forks Martichou/rquickshare.",
        "fr" => "Développé par Stanislav Moseyko à partir du fork Martichou/rquickshare.",
        "it" => "Sviluppato da Stanislav Moseyko sulla base del fork Martichou/rquickshare.",
        "pl" => "Opracowane przez Stanislav Moseyko na podstawie forka Martichou/rquickshare.",
        "hi" => "Stanislav Moseyko द्वारा विकसित, Martichou/rquickshare फ़ोर्क पर आधारित।",
        "zh-Hant" => "由 Stanislav Moseyko 開發，以 Martichou/rquickshare 分支版本為基礎。",
        "ja" => "Stanislav Moseyko が開発。Martichou/rquickshare のフォークをベースにしています。",
        "nl" => "Ontwikkeld door Stanislav Moseyko op basis van de fork Martichou/rquickshare.",
        "sv" => "Utvecklat av Stanislav Moseyko baserat på förgreningen Martichou/rquickshare.",
        "tr" => "Stanislav Moseyko tarafından, Martichou/rquickshare çatalı temel alınarak geliştirildi.",
        "ko" => "Stanislav Moseyko 개발. Martichou/rquickshare 포크를 기반으로 합니다.",
        "bn" => "Stanislav Moseyko দ্বারা তৈরি, Martichou/rquickshare ফর্কের ভিত্তিতে।",
        "th" => "พัฒนาโดย Stanislav Moseyko โดยอ้างอิงจากฟอร์ก Martichou/rquickshare",
        "id" => "Dikembangkan oleh Stanislav Moseyko berdasarkan fork Martichou/rquickshare.",
        "fil" => "Binuo ni Stanislav Moseyko batay sa fork ng Martichou/rquickshare.",
        "ar" => "طوّره Stanislav Moseyko استنادًا إلى تفريع Martichou/rquickshare.",
        _ => "Developed by Stanislav Moseyko based on the Martichou/rquickshare fork.",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_each_canonical_locale_when_about_is_shown_then_author_and_fork_are_attributed() {
        for locale in super::super::LOCALES {
            let text = description(locale);
            assert!(text.contains("Martichou/rquickshare"), "{locale}");
            assert!(
                text.contains("Stanislav Moseyko") || text.contains("Станиславом Мосейко"),
                "{locale}"
            );
            assert!(!text.contains("Linux"), "{locale}");
            assert!(!text.contains('<') && !text.contains('{'), "{locale}");
        }
        assert_eq!(description("unsupported"), description("en"));
        assert_eq!(description("pt-BR"), description("en"));
        assert_eq!(description("zh-Hans"), description("en"));
    }
}
