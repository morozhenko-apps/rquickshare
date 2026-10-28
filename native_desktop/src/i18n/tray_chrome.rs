//! StatusNotifierItem and DBusMenu presentation strings only.
//! Action IDs, D-Bus properties and consent/policy decisions remain independent.

pub(super) const KEYS: [&str; 10] = [
    "Open card",
    "Send text…",
    "Send image…",
    "Retry visibility change",
    "Send a folder…",
    "Incoming requests",
    "Compact view",
    "Full view",
    "Reconnect service",
    "Send and receive nearby files",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TrayCountKind {
    Incoming,
    History,
}

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 10] = match locale {
        "ru" => &[
            "Открыть карточку",
            "Отправить текст…",
            "Отправить изображение…",
            "Повторить изменение видимости",
            "Отправить папку…",
            "Входящие запросы",
            "Компактный вид",
            "Полный вид",
            "Переподключиться к службе",
            "Отправка и получение файлов с ближайших устройств",
        ],
        "es" => &[
            "Abrir tarjeta",
            "Enviar texto…",
            "Enviar imagen…",
            "Reintentar el cambio de visibilidad",
            "Enviar una carpeta…",
            "Solicitudes entrantes",
            "Vista compacta",
            "Vista completa",
            "Reconectar el servicio",
            "Enviar y recibir archivos con dispositivos cercanos",
        ],
        "pt" => &[
            "Abrir cartão",
            "Enviar texto…",
            "Enviar imagem…",
            "Repetir a alteração de visibilidade",
            "Enviar uma pasta…",
            "Pedidos recebidos",
            "Vista compacta",
            "Vista completa",
            "Voltar a ligar ao serviço",
            "Enviar e receber ficheiros com dispositivos próximos",
        ],
        "de" => &[
            "Karte öffnen",
            "Text senden…",
            "Bild senden…",
            "Sichtbarkeitsänderung erneut versuchen",
            "Ordner senden…",
            "Eingehende Anfragen",
            "Kompakte Ansicht",
            "Vollständige Ansicht",
            "Dienst erneut verbinden",
            "Dateien mit Geräten in der Nähe austauschen",
        ],
        "fr" => &[
            "Ouvrir la carte",
            "Envoyer du texte…",
            "Envoyer une image…",
            "Réessayer de modifier la visibilité",
            "Envoyer un dossier…",
            "Demandes entrantes",
            "Vue compacte",
            "Vue complète",
            "Reconnecter le service",
            "Envoyer et recevoir des fichiers avec les appareils à proximité",
        ],
        "it" => &[
            "Apri scheda",
            "Invia testo…",
            "Invia immagine…",
            "Riprova a cambiare la visibilità",
            "Invia una cartella…",
            "Richieste in arrivo",
            "Vista compatta",
            "Vista completa",
            "Riconnetti il servizio",
            "Invia e ricevi file da dispositivi vicini",
        ],
        "pl" => &[
            "Otwórz kartę",
            "Wyślij tekst…",
            "Wyślij obraz…",
            "Ponów zmianę widoczności",
            "Wyślij folder…",
            "Żądania przychodzące",
            "Widok kompaktowy",
            "Widok pełny",
            "Połącz ponownie z usługą",
            "Wysyłaj i odbieraj pliki z pobliskich urządzeń",
        ],
        "hi" => &[
            "कार्ड खोलें",
            "टेक्स्ट भेजें…",
            "छवि भेजें…",
            "दृश्यता बदलने का फिर प्रयास करें",
            "फ़ोल्डर भेजें…",
            "आने वाले अनुरोध",
            "संक्षिप्त दृश्य",
            "पूरा दृश्य",
            "सेवा से फिर कनेक्ट करें",
            "आस-पास के उपकरणों के साथ फ़ाइलें भेजें और प्राप्त करें",
        ],
        "zh-Hant" => &[
            "開啟卡片",
            "傳送文字…",
            "傳送圖片…",
            "重試變更可見性",
            "傳送資料夾…",
            "傳入要求",
            "精簡檢視",
            "完整檢視",
            "重新連接服務",
            "與附近裝置傳送及接收檔案",
        ],
        "ja" => &[
            "カードを開く",
            "テキストを送信…",
            "画像を送信…",
            "公開設定の変更を再試行",
            "フォルダーを送信…",
            "受信リクエスト",
            "コンパクト表示",
            "全体表示",
            "サービスに再接続",
            "近くのデバイスとファイルを送受信",
        ],
        "nl" => &[
            "Kaart openen",
            "Tekst verzenden…",
            "Afbeelding verzenden…",
            "Zichtbaarheidswijziging opnieuw proberen",
            "Map verzenden…",
            "Binnenkomende verzoeken",
            "Compacte weergave",
            "Volledige weergave",
            "Opnieuw verbinden met service",
            "Bestanden verzenden naar en ontvangen van apparaten in de buurt",
        ],
        "sv" => &[
            "Öppna kort",
            "Skicka text…",
            "Skicka bild…",
            "Försök ändra synlighet igen",
            "Skicka mapp…",
            "Inkommande förfrågningar",
            "Kompakt vy",
            "Fullständig vy",
            "Återanslut till tjänsten",
            "Skicka och ta emot filer med enheter i närheten",
        ],
        "tr" => &[
            "Kartı aç",
            "Metin gönder…",
            "Resim gönder…",
            "Görünürlük değişikliğini yeniden dene",
            "Klasör gönder…",
            "Gelen istekler",
            "Kompakt görünüm",
            "Tam görünüm",
            "Hizmete yeniden bağlan",
            "Yakındaki cihazlarla dosya gönder ve al",
        ],
        "ko" => &[
            "카드 열기",
            "텍스트 보내기…",
            "이미지 보내기…",
            "공개 상태 변경 다시 시도",
            "폴더 보내기…",
            "받은 요청",
            "간단 보기",
            "전체 보기",
            "서비스 다시 연결",
            "주변 기기와 파일 주고받기",
        ],
        "bn" => &[
            "কার্ড খুলুন",
            "লেখা পাঠান…",
            "ছবি পাঠান…",
            "দৃশ্যমানতা পরিবর্তন আবার চেষ্টা করুন",
            "ফোল্ডার পাঠান…",
            "আসা অনুরোধ",
            "সংক্ষিপ্ত দৃশ্য",
            "সম্পূর্ণ দৃশ্য",
            "সেবায় আবার সংযোগ করুন",
            "কাছের ডিভাইসে ফাইল পাঠান ও গ্রহণ করুন",
        ],
        "th" => &[
            "เปิดการ์ด",
            "ส่งข้อความ…",
            "ส่งรูปภาพ…",
            "ลองเปลี่ยนการมองเห็นอีกครั้ง",
            "ส่งโฟลเดอร์…",
            "คำขอขาเข้า",
            "มุมมองย่อ",
            "มุมมองเต็ม",
            "เชื่อมต่อบริการอีกครั้ง",
            "ส่งและรับไฟล์กับอุปกรณ์ใกล้เคียง",
        ],
        "id" => &[
            "Buka kartu",
            "Kirim teks…",
            "Kirim gambar…",
            "Coba lagi ubah visibilitas",
            "Kirim folder…",
            "Permintaan masuk",
            "Tampilan ringkas",
            "Tampilan penuh",
            "Sambungkan ulang layanan",
            "Kirim dan terima berkas dengan perangkat terdekat",
        ],
        "fil" => &[
            "Buksan ang card",
            "Magpadala ng text…",
            "Magpadala ng larawan…",
            "Subukang baguhin muli ang visibility",
            "Magpadala ng folder…",
            "Mga papasok na kahilingan",
            "Compact na view",
            "Buong view",
            "Ikonekta muli ang serbisyo",
            "Magpadala at tumanggap ng mga file sa kalapit na device",
        ],
        "ar" => &[
            "فتح البطاقة",
            "إرسال نص…",
            "إرسال صورة…",
            "إعادة محاولة تغيير مستوى الظهور",
            "إرسال مجلد…",
            "الطلبات الواردة",
            "عرض مدمج",
            "عرض كامل",
            "إعادة الاتصال بالخدمة",
            "إرسال الملفات واستلامها من الأجهزة القريبة",
        ],
        _ => &KEYS,
    };
    Some(row[index])
}

/// Counts describe the current tray projection, not unread or saved history.
/// Decimal ASCII counts remain exact; never infer transfer completion.
pub(super) fn count_for(locale: &str, kind: TrayCountKind, count: u16) -> String {
    let source = match kind {
        TrayCountKind::Incoming => "Incoming requests",
        TrayCountKind::History => "History",
    };
    format!("{} ({count})", super::translated(locale, source))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn tray_key_matrix_has_twenty_total_locale_rows_and_safe_fallbacks() {
        assert_eq!(KEYS.len(), 10);
        for (index, source) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(source));
            for locale in LOCALES {
                let value = translated(locale, source).expect("known tray key");
                assert!(!value.trim().is_empty(), "{locale}: {source}");
                assert!(!value.contains('{'), "{locale}: {source}");
                assert!(!value.contains('<'), "{locale}: {source}");
            }
            assert_eq!(translated("en", source), Some(*source));
            assert_eq!(translated("pt-BR", source), Some(*source));
            assert_eq!(translated("zh-Hans", source), Some(*source));
        }
        assert_eq!(translated("ru", KEYS[0]), Some("Открыть карточку"));
        assert_eq!(translated("pt", KEYS[5]), Some("Pedidos recebidos"));
        assert_eq!(translated("ar", KEYS[9]), Some("إرسال الملفات واستلامها من الأجهزة القريبة"));
        assert_eq!(translated("fr", "not a key"), None);
    }

    #[test]
    fn tray_counts_preserve_source_projection_and_locale_context() {
        for locale in LOCALES {
            for count in [0, 1, 2, 15, 256, u16::MAX] {
                let incoming = count_for(locale, TrayCountKind::Incoming, count);
                let history = count_for(locale, TrayCountKind::History, count);
                assert!(incoming.ends_with(&format!(" ({count})")), "{locale}");
                assert!(history.ends_with(&format!(" ({count})")), "{locale}");
                assert_ne!(incoming, history, "{locale}");
                assert!(!incoming.contains('{'), "{locale}");
            }
        }
        assert_eq!(
            count_for("en", TrayCountKind::Incoming, 0),
            "Incoming requests (0)"
        );
        assert_eq!(
            count_for("ru", TrayCountKind::Incoming, 2),
            "Входящие запросы (2)"
        );
        let portuguese_history = super::super::translated("pt", "History");
        assert_eq!(
            count_for("pt", TrayCountKind::History, 3),
            format!("{portuguese_history} (3)")
        );
        assert_eq!(
            count_for("ar", TrayCountKind::Incoming, u16::MAX),
            "الطلبات الواردة (65535)"
        );
        assert_eq!(
            count_for("pt-BR", TrayCountKind::Incoming, 1),
            "Incoming requests (1)"
        );
    }
}
