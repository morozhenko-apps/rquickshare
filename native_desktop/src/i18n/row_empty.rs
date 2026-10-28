//! Presentation-only empty-state messages and exact saved-file tally.
//! No transfer completion or device discovery state is inferred here.

pub(super) const KEYS: [&str; 3] = [
    "No nearby devices. Make the receiving device visible.",
    "No incoming requests",
    "No transfers",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 3] = match locale {
        "ru" => &[
            "Устройств поблизости нет. Включите видимость на принимающем устройстве.",
            "Нет входящих запросов",
            "Нет передач",
        ],
        "es" => &[
            "No hay dispositivos cercanos. Haz visible el dispositivo receptor.",
            "No hay solicitudes entrantes",
            "No hay transferencias",
        ],
        "pt" => &[
            "Não há dispositivos próximos. Ative a visibilidade no dispositivo recetor.",
            "Não há pedidos recebidos",
            "Não há transferências",
        ],
        "de" => &[
            "Keine Geräte in der Nähe. Machen Sie das empfangende Gerät sichtbar.",
            "Keine eingehenden Anfragen",
            "Keine Übertragungen",
        ],
        "fr" => &[
            "Aucun appareil à proximité. Rendez l’appareil de réception visible.",
            "Aucune demande entrante",
            "Aucun transfert",
        ],
        "it" => &[
            "Nessun dispositivo nelle vicinanze. Rendi visibile il dispositivo ricevente.",
            "Nessuna richiesta in arrivo",
            "Nessun trasferimento",
        ],
        "pl" => &[
            "Brak urządzeń w pobliżu. Włącz widoczność urządzenia odbierającego.",
            "Brak przychodzących żądań",
            "Brak transferów",
        ],
        "hi" => &[
            "आस-पास कोई डिवाइस नहीं है। प्राप्त करने वाले डिवाइस को दृश्यमान बनाएं।",
            "कोई आने वाला अनुरोध नहीं",
            "कोई स्थानांतरण नहीं",
        ],
        "zh-Hant" => &[
            "附近沒有裝置。請開啟接收裝置的可見性。",
            "沒有傳入要求",
            "沒有傳輸",
        ],
        "ja" => &[
            "近くにデバイスがありません。受信側のデバイスを検出可能にしてください。",
            "受信リクエストはありません",
            "転送はありません",
        ],
        "nl" => &[
            "Geen apparaten in de buurt. Maak het ontvangende apparaat zichtbaar.",
            "Geen binnenkomende verzoeken",
            "Geen overdrachten",
        ],
        "sv" => &[
            "Inga enheter i närheten. Gör den mottagande enheten synlig.",
            "Inga inkommande förfrågningar",
            "Inga överföringar",
        ],
        "tr" => &[
            "Yakında cihaz yok. Alıcı cihazı görünür yapın.",
            "Gelen istek yok",
            "Aktarım yok",
        ],
        "ko" => &[
            "주변에 기기가 없습니다. 수신 기기를 검색 가능하게 설정하세요.",
            "받은 요청 없음",
            "전송 없음",
        ],
        "bn" => &[
            "কাছাকাছি কোনো ডিভাইস নেই। গ্রহণকারী ডিভাইসটি দৃশ্যমান করুন।",
            "কোনো আসা অনুরোধ নেই",
            "কোনো স্থানান্তর নেই",
        ],
        "th" => &[
            "ไม่พบอุปกรณ์ใกล้เคียง โปรดเปิดการมองเห็นบนอุปกรณ์รับ",
            "ไม่มีคำขอขาเข้า",
            "ไม่มีการโอนย้าย",
        ],
        "id" => &[
            "Tidak ada perangkat di dekat Anda. Aktifkan visibilitas perangkat penerima.",
            "Tidak ada permintaan masuk",
            "Tidak ada transfer",
        ],
        "fil" => &[
            "Walang kalapit na device. Gawing nakikita ang tatanggap na device.",
            "Walang papasok na kahilingan",
            "Walang paglilipat",
        ],
        "ar" => &[
            "لا توجد أجهزة قريبة. اجعل جهاز الاستقبال مرئيًا.",
            "لا توجد طلبات واردة",
            "لا توجد عمليات نقل",
        ],
        _ => &KEYS,
    };
    Some(row[index])
}

/// Exact source counts; the LRI/PDI guards the fraction in RTL UI.
pub(super) fn saved_files_for(locale: &str, saved: usize, total: usize) -> String {
    format!(
        "{}: \u{2066}{saved}/{total}\u{2069}",
        super::translated(locale, "Saved files")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn three_empty_states_preserve_distinct_english_sources_and_twenty_locale_rows() {
        assert_eq!(KEYS.len(), 3);
        for (i, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..i].contains(key));
            for locale in LOCALES {
                let translated = translated(locale, key).expect("known empty-state key");
                assert!(!translated.trim().is_empty(), "{locale}: {key}");
                assert!(!translated.contains("<"), "{locale}: {key}");
                assert!(!translated.contains("{"), "{locale}: {key}");
            }
            assert_eq!(translated("en", key), Some(*key));
            assert_eq!(translated("pt-BR", key), Some(*key));
            assert_eq!(translated("zh-Hans", key), Some(*key));
        }
        for locale in LOCALES {
            assert_ne!(translated(locale, KEYS[0]), translated(locale, KEYS[1]));
            assert_ne!(translated(locale, KEYS[1]), translated(locale, KEYS[2]));
        }
        assert_eq!(translated("pt", KEYS[1]), Some("Não há pedidos recebidos"));
        assert_eq!(translated("ru", KEYS[2]), Some("Нет передач"));
        assert_eq!(translated("ar", KEYS[1]), Some("لا توجد طلبات واردة"));
        assert_eq!(translated("unknown", "not cataloged"), None);
    }

    #[test]
    fn saved_file_tally_preserves_exact_counts_and_ltr_isolation_without_success_inference() {
        for locale in LOCALES {
            for (saved, total) in [
                (0usize, 0usize),
                (0, 2),
                (1, 2),
                (2, 2),
                (3, 2),
                (u16::MAX as usize, u16::MAX as usize),
                (usize::MAX, usize::MAX),
            ] {
                let caption = saved_files_for(locale, saved, total);
                assert!(caption.ends_with(&format!("\u{2066}{saved}/{total}\u{2069}")));
                assert!(caption.starts_with(super::super::translated(locale, "Saved files")));
            }
        }
        assert_eq!(
            saved_files_for("en", 1, 2),
            "Saved files: \u{2066}1/2\u{2069}"
        );
        assert_eq!(
            saved_files_for("pt-BR", 0, 2),
            "Saved files: \u{2066}0/2\u{2069}"
        );
        assert_eq!(
            saved_files_for("ar", 1, 2),
            "ملفات محفوظة: \u{2066}1/2\u{2069}"
        );
    }

    #[test]
    fn given_partial_and_unexpected_counts_when_saved_then_no_count_is_invented() {
        let lri = char::from_u32(0x2066).unwrap();
        let pdi = char::from_u32(0x2069).unwrap();
        let cases = [
            ("en", 0usize, 2usize, "Saved files"),
            ("pt", 1, 2, "Ficheiros guardados"),
            ("ru", 3, 2, "Сохранённые файлы"),
            ("ar", 2, 2, "ملفات محفوظة"),
            ("pt-BR", 1, 2, "Saved files"),
            ("zh-Hans", 1, 2, "Saved files"),
        ];
        for (locale, saved, total, expected_label) in cases {
            let actual = saved_files_for(locale, saved, total);
            let expected = format!("{expected_label}: {lri}{saved}/{total}{pdi}");
            assert_eq!(actual, expected, "{locale}: {saved}/{total}");
            assert_eq!(actual.matches(lri).count(), 1);
            assert_eq!(actual.matches(pdi).count(), 1);
        }
        assert_eq!(
            saved_files_for("ar", usize::MAX, 0),
            format!("ملفات محفوظة: {lri}{}/0{pdi}", usize::MAX)
        );
    }
}
