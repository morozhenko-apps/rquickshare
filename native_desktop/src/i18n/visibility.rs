//! Presentation-only Receive visibility captions. The daemon owns visibility expiry.

pub(super) const KEYS: [&str; 2] = ["Visible to nearby devices", "Updating visibility…"];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 2] = match locale {
        "ru" => &["Виден устройствам поблизости", "Обновление видимости…"],
        "es" => &[
            "Visible para dispositivos cercanos",
            "Actualizando visibilidad…",
        ],
        "pt" => &[
            "Visível para dispositivos nas proximidades",
            "A atualizar a visibilidade…",
        ],
        "de" => &[
            "Für Geräte in der Nähe sichtbar",
            "Sichtbarkeit wird aktualisiert…",
        ],
        "fr" => &[
            "Visible pour les appareils à proximité",
            "Actualisation de la visibilité…",
        ],
        "it" => &[
            "Visibile ai dispositivi vicini",
            "Aggiornamento della visibilità…",
        ],
        "pl" => &[
            "Widoczny dla urządzeń w pobliżu",
            "Aktualizowanie widoczności…",
        ],
        "hi" => &[
            "आस-पास के डिवाइस को दिखाई दे रहा है",
            "दृश्यता अपडेट की जा रही है…",
        ],
        "zh-Hant" => &["附近裝置可見", "正在更新可見性…"],
        "ja" => &["近くのデバイスに表示中", "公開設定を更新中…"],
        "nl" => &[
            "Zichtbaar voor apparaten in de buurt",
            "Zichtbaarheid bijwerken…",
        ],
        "sv" => &["Synlig för enheter i närheten", "Uppdaterar synlighet…"],
        "tr" => &["Yakındaki cihazlara görünür", "Görünürlük güncelleniyor…"],
        "ko" => &["주변 기기에 표시 중", "공개 상태 업데이트 중…"],
        "bn" => &["কাছাকাছি ডিভাইসগুলির কাছে দৃশ্যমান", "দৃশ্যমানতা আপডেট করা হচ্ছে…"],
        "th" => &["อุปกรณ์ใกล้เคียงมองเห็นได้", "กำลังอัปเดตการมองเห็น…"],
        "id" => &[
            "Terlihat oleh perangkat di sekitar",
            "Memperbarui visibilitas…",
        ],
        "fil" => &[
            "Nakikita ng mga device sa malapit",
            "Ina-update ang visibility…",
        ],
        "ar" => &["مرئي للأجهزة القريبة", "جارٍ تحديث إمكانية الظهور…"],
        _ => &KEYS,
    };
    Some(row[index])
}

/// Exact whole seconds already checked against the daemon's remaining lease.
/// LRI/PDI keeps ASCII M:SS intact in Arabic and other mixed-direction captions.
pub(super) fn countdown_for(locale: &str, remaining_seconds: u64) -> String {
    let caption = translated(locale, KEYS[0]).expect("static visibility caption");
    let minutes = remaining_seconds / 60;
    let seconds = remaining_seconds % 60;
    format!("{caption} · \u{2066}{minutes}:{seconds:02}\u{2069}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn given_twenty_locales_when_resolving_visibility_then_two_distinct_complete_captions_remain() {
        assert_eq!(KEYS.len(), 2);
        assert_ne!(KEYS[0], KEYS[1]);
        for locale in LOCALES {
            let visible = translated(locale, KEYS[0]).expect("visible caption");
            let updating = translated(locale, KEYS[1]).expect("updating caption");
            assert!(!visible.trim().is_empty(), "{locale}: visible");
            assert!(!updating.trim().is_empty(), "{locale}: updating");
            assert_ne!(visible, updating, "{locale}");
            assert!(!visible.contains('{'), "{locale}");
            assert!(!updating.contains('{'), "{locale}");
        }
        assert_eq!(
            translated("ru", KEYS[0]),
            Some("Виден устройствам поблизости")
        );
        assert_eq!(
            translated("pt", KEYS[1]),
            Some("A atualizar a visibilidade…")
        );
        assert_eq!(translated("ar", KEYS[1]), Some("جارٍ تحديث إمكانية الظهور…"));
        assert_eq!(translated("en", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("pt-BR", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("zh-Hans", KEYS[1]), Some(KEYS[1]));
        assert_eq!(translated("unknown", "Untrusted peer"), None);
    }

    #[test]
    fn given_exact_seconds_when_formatting_countdown_then_time_is_not_rounded_or_reordered() {
        for locale in LOCALES {
            for remaining in [0, 1, 9, 10, 59, 60, 61, 3599, 3600, u64::MAX] {
                let shown = countdown_for(locale, remaining);
                let minute = remaining / 60;
                let second = remaining % 60;
                let expected_time = format!("\u{2066}{minute}:{second:02}\u{2069}");
                assert!(shown.ends_with(&expected_time), "{locale}: {remaining}");
                assert_eq!(shown.matches('\u{2066}').count(), 1, "{locale}");
                assert_eq!(shown.matches('\u{2069}').count(), 1, "{locale}");
                assert!(
                    shown.starts_with(translated(locale, KEYS[0]).unwrap()),
                    "{locale}"
                );
            }
        }
        assert_eq!(
            countdown_for("en", 60),
            "Visible to nearby devices · \u{2066}1:00\u{2069}"
        );
        assert_eq!(
            countdown_for("pt", 1),
            "Visível para dispositivos nas proximidades · \u{2066}0:01\u{2069}"
        );
        assert_eq!(
            countdown_for("ar", 59),
            "مرئي للأجهزة القريبة · \u{2066}0:59\u{2069}"
        );
    }
}
