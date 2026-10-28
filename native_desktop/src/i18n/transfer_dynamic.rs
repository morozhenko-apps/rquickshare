//! Presentation-only transfer recipient, byte progress and phase labels.
//! Never infer verified recipient identity or successful publication from display text.

use super::dynamic::isolate_untrusted;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProgressKind {
    Written,
    Received,
    Sent,
}

const ENGLISH_PHASES: [&str; 11] = [
    "Queued",
    "Connecting",
    "Waiting for consent",
    "Receiving",
    "Sending",
    "Cancellation requested",
    "Completed",
    "Declined",
    "Cancelled",
    "Failed",
    "Disconnected",
];

pub(super) fn recipient_for(locale: &str, name: &str) -> String {
    let caption = match locale {
        "ru" => "Получатель",
        "es" => "Destinatario",
        "pt" => "Destinatário",
        "de" => "Empfänger",
        "fr" => "Destinataire",
        "it" => "Destinatario",
        "pl" => "Odbiorca",
        "hi" => "प्राप्तकर्ता",
        "zh-Hant" => "收件者",
        "ja" => "送信先",
        "nl" => "Ontvanger",
        "sv" => "Mottagare",
        "tr" => "Alıcı",
        "ko" => "수신자",
        "bn" => "প্রাপক",
        "th" => "ผู้รับ",
        "id" => "Penerima",
        "fil" => "Tatanggap",
        "ar" => "المستلم",
        _ => "Recipient",
    };
    format!("{caption}: {}", isolate_untrusted(name))
}

/// Numbers stay exact and ASCII, including zero and u64::MAX.
/// LRI/PDI isolates the numeric fraction from RTL captions.
pub(super) fn progress_for(
    locale: &str,
    transferred: u64,
    total: u64,
    kind: ProgressKind,
) -> String {
    let descriptions: &[&str; 3] = match locale {
        "ru" => &[
            "байт записано локально",
            "байт получено локально",
            "байт отправлено локально",
        ],
        "es" => &[
            "bytes escritos localmente",
            "bytes recibidos localmente",
            "bytes enviados localmente",
        ],
        "pt" => &[
            "bytes escritos localmente",
            "bytes recebidos localmente",
            "bytes enviados localmente",
        ],
        "de" => &[
            "Bytes lokal geschrieben",
            "Bytes lokal empfangen",
            "Bytes lokal gesendet",
        ],
        "fr" => &[
            "octets écrits localement",
            "octets reçus localement",
            "octets envoyés localement",
        ],
        "it" => &[
            "byte scritti localmente",
            "byte ricevuti localmente",
            "byte inviati localmente",
        ],
        "pl" => &[
            "bajtów zapisanych lokalnie",
            "bajtów odebranych lokalnie",
            "bajtów wysłanych lokalnie",
        ],
        "hi" => &[
            "बाइट स्थानीय रूप से लिखे गए",
            "बाइट स्थानीय रूप से प्राप्त हुए",
            "बाइट स्थानीय रूप से भेजे गए",
        ],
        "zh-Hant" => &[
            "位元組已寫入本機",
            "位元組已在本機接收",
            "位元組已在本機送出",
        ],
        "ja" => &[
            "バイトをローカルに書き込み",
            "バイトをローカルで受信",
            "バイトをローカルから送信",
        ],
        "nl" => &[
            "bytes lokaal geschreven",
            "bytes lokaal ontvangen",
            "bytes lokaal verzonden",
        ],
        "sv" => &[
            "byte skrivna lokalt",
            "byte mottagna lokalt",
            "byte skickade lokalt",
        ],
        "tr" => &[
            "bayt yerel olarak yazıldı",
            "bayt yerel olarak alındı",
            "bayt yerel olarak gönderildi",
        ],
        "ko" => &["바이트 로컬 기록", "바이트 로컬 수신", "바이트 로컬 전송"],
        "bn" => &[
            "বাইট স্থানীয়ভাবে লেখা হয়েছে",
            "বাইট স্থানীয়ভাবে গৃহীত হয়েছে",
            "বাইট স্থানীয়ভাবে পাঠানো হয়েছে",
        ],
        "th" => &["ไบต์เขียนในเครื่อง", "ไบต์รับในเครื่อง", "ไบต์ส่งจากเครื่อง"],
        "id" => &[
            "byte ditulis secara lokal",
            "byte diterima secara lokal",
            "byte dikirim secara lokal",
        ],
        "fil" => &[
            "byte na isinulat nang lokal",
            "byte na natanggap nang lokal",
            "byte na ipinadala nang lokal",
        ],
        "ar" => &["بايت كُتبت محليًا", "بايت استُلمت محليًا", "بايت أُرسلت محليًا"],
        _ => &[
            "bytes locally written",
            "bytes locally received",
            "bytes locally sent",
        ],
    };
    let description = descriptions[match kind {
        ProgressKind::Written => 0,
        ProgressKind::Received => 1,
        ProgressKind::Sent => 2,
    }];
    format!("\u{2066}{transferred} / {total}\u{2069} {description}")
}

/// Phase translation is intentionally separate from the generic i18n key space,
/// where identical words in Settings/History have different UI contexts.
pub(super) fn phase_for(locale: &str, phase: &'static str) -> &'static str {
    let Some(index) = ENGLISH_PHASES.iter().position(|key| *key == phase) else {
        return phase;
    };
    let translated: &[&str; 11] = match locale {
        "ru" => &[
            "В очереди",
            "Подключение",
            "Ожидание согласия",
            "Получение",
            "Отправка",
            "Отмена запрошена",
            "Завершено",
            "Отклонено",
            "Отменено",
            "Ошибка",
            "Соединение разорвано",
        ],
        "es" => &[
            "En cola",
            "Conectando",
            "Esperando consentimiento",
            "Recibiendo",
            "Enviando",
            "Cancelación solicitada",
            "Completado",
            "Rechazado",
            "Cancelado",
            "Fallido",
            "Desconectado",
        ],
        "pt" => &[
            "Em fila",
            "A ligar",
            "À espera de consentimento",
            "A receber",
            "A enviar",
            "Cancelamento solicitado",
            "Concluído",
            "Recusado",
            "Cancelado",
            "Falhou",
            "Desligado",
        ],
        "de" => &[
            "In Warteschlange",
            "Verbindung wird aufgebaut",
            "Warten auf Zustimmung",
            "Empfangen",
            "Senden",
            "Abbruch angefordert",
            "Abgeschlossen",
            "Abgelehnt",
            "Abgebrochen",
            "Fehlgeschlagen",
            "Verbindung getrennt",
        ],
        "fr" => &[
            "En attente",
            "Connexion en cours",
            "En attente du consentement",
            "Réception en cours",
            "Envoi en cours",
            "Annulation demandée",
            "Terminé",
            "Refusé",
            "Annulé",
            "Échec",
            "Déconnecté",
        ],
        "it" => &[
            "In coda",
            "Connessione in corso",
            "In attesa del consenso",
            "Ricezione",
            "Invio",
            "Annullamento richiesto",
            "Completato",
            "Rifiutato",
            "Annullato",
            "Non riuscito",
            "Disconnesso",
        ],
        "pl" => &[
            "W kolejce",
            "Łączenie",
            "Oczekiwanie na zgodę",
            "Odbieranie",
            "Wysyłanie",
            "Zażądano anulowania",
            "Ukończono",
            "Odrzucono",
            "Anulowano",
            "Błąd",
            "Rozłączono",
        ],
        "hi" => &[
            "कतार में",
            "कनेक्ट हो रहा है",
            "सहमति की प्रतीक्षा",
            "प्राप्त हो रहा है",
            "भेजा जा रहा है",
            "रद्द करने का अनुरोध",
            "पूरा हुआ",
            "अस्वीकार किया गया",
            "रद्द किया गया",
            "विफल",
            "कनेक्शन टूटा",
        ],
        "zh-Hant" => &[
            "佇列中",
            "連線中",
            "等待同意",
            "接收中",
            "傳送中",
            "已要求取消",
            "已完成",
            "已拒絕",
            "已取消",
            "失敗",
            "已中斷連線",
        ],
        "ja" => &[
            "キュー内",
            "接続中",
            "同意待ち",
            "受信中",
            "送信中",
            "キャンセルを要求済み",
            "完了",
            "拒否",
            "キャンセル済み",
            "失敗",
            "接続解除",
        ],
        "nl" => &[
            "In wachtrij",
            "Verbinden",
            "Wachten op toestemming",
            "Ontvangen",
            "Verzenden",
            "Annulering aangevraagd",
            "Voltooid",
            "Geweigerd",
            "Geannuleerd",
            "Mislukt",
            "Verbinding verbroken",
        ],
        "sv" => &[
            "I kö",
            "Ansluter",
            "Väntar på samtycke",
            "Tar emot",
            "Skickar",
            "Avbrott begärt",
            "Slutförd",
            "Avböjd",
            "Avbruten",
            "Misslyckades",
            "Frånkopplad",
        ],
        "tr" => &[
            "Kuyrukta",
            "Bağlanıyor",
            "Onay bekleniyor",
            "Alınıyor",
            "Gönderiliyor",
            "İptal istendi",
            "Tamamlandı",
            "Reddedildi",
            "İptal edildi",
            "Başarısız",
            "Bağlantı kesildi",
        ],
        "ko" => &[
            "대기열",
            "연결 중",
            "동의 대기 중",
            "수신 중",
            "전송 중",
            "취소 요청됨",
            "완료",
            "거부됨",
            "취소됨",
            "실패",
            "연결 끊김",
        ],
        "bn" => &[
            "সারিতে",
            "সংযোগ হচ্ছে",
            "সম্মতির অপেক্ষা",
            "গ্রহণ চলছে",
            "পাঠানো হচ্ছে",
            "বাতিলের অনুরোধ করা হয়েছে",
            "সম্পন্ন",
            "প্রত্যাখ্যাত",
            "বাতিল হয়েছে",
            "ব্যর্থ",
            "সংযোগ বিচ্ছিন্ন",
        ],
        "th" => &[
            "อยู่ในคิว",
            "กำลังเชื่อมต่อ",
            "รอการยินยอม",
            "กำลังรับ",
            "กำลังส่ง",
            "ขอยกเลิกแล้ว",
            "เสร็จสมบูรณ์",
            "ถูกปฏิเสธ",
            "ยกเลิกแล้ว",
            "ล้มเหลว",
            "ขาดการเชื่อมต่อ",
        ],
        "id" => &[
            "Dalam antrean",
            "Menghubungkan",
            "Menunggu persetujuan",
            "Menerima",
            "Mengirim",
            "Pembatalan diminta",
            "Selesai",
            "Ditolak",
            "Dibatalkan",
            "Gagal",
            "Terputus",
        ],
        "fil" => &[
            "Nasa pila",
            "Kumokonekta",
            "Naghihintay ng pahintulot",
            "Tumatanggap",
            "Nagpapadala",
            "Hiniling ang pagkansela",
            "Nakumpleto",
            "Tinanggihan",
            "Kinansela",
            "Nabigo",
            "Nadiskonekta",
        ],
        "ar" => &[
            "في قائمة الانتظار",
            "جارٍ الاتصال",
            "في انتظار الموافقة",
            "جارٍ الاستلام",
            "جارٍ الإرسال",
            "طُلب الإلغاء",
            "مكتمل",
            "مرفوض",
            "ملغى",
            "فشل",
            "انقطع الاتصال",
        ],
        _ => &ENGLISH_PHASES,
    };
    translated[index]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn transfer_phases_are_total_unambiguous_and_do_not_alias_cancel_request_with_cancelled() {
        for locale in LOCALES {
            let values: Vec<_> = ENGLISH_PHASES
                .iter()
                .map(|phase| phase_for(locale, phase))
                .collect();
            assert_eq!(values.len(), 11);
            assert!(
                values.iter().all(|value| !value.trim().is_empty()),
                "{locale}"
            );
            for (index, value) in values.iter().enumerate() {
                assert!(!values[..index].contains(value), "{locale}: {index}");
            }
            assert_ne!(values[5], values[8], "{locale}");
            assert_ne!(values[6], values[9], "{locale}");
            assert_ne!(values[7], values[10], "{locale}");
        }
        assert_eq!(
            phase_for("en", "Cancellation requested"),
            "Cancellation requested"
        );
        assert_eq!(
            phase_for("ru", "Cancellation requested"),
            "Отмена запрошена"
        );
        assert_eq!(phase_for("pt", "Cancelled"), "Cancelado");
        assert_eq!(phase_for("ar", "Disconnected"), "انقطع الاتصال");
        assert_eq!(phase_for("pt-BR", "Completed"), "Completed");
        assert_eq!(phase_for("zh-Hans", "Receiving"), "Receiving");
        assert_eq!(phase_for("en", "Unknown phase"), "Unknown phase");
    }

    #[test]
    fn exact_progress_never_rounds_or_reports_finished_content() {
        for locale in LOCALES {
            for (done, total) in [(0, 0), (0, 1), (1, 1), (2048, 12345), (u64::MAX, u64::MAX)] {
                for kind in [
                    ProgressKind::Written,
                    ProgressKind::Received,
                    ProgressKind::Sent,
                ] {
                    let value = progress_for(locale, done, total, kind);
                    assert!(
                        value.starts_with(&format!("\u{2066}{done} / {total}\u{2069} ")),
                        "{locale}"
                    );
                    assert!(!value.trim().is_empty(), "{locale}");
                }
            }
            for kind in [
                ProgressKind::Written,
                ProgressKind::Received,
                ProgressKind::Sent,
            ] {
                let value = progress_for(locale, 1, 2, kind);
                assert!(!value.contains("Completed"), "{locale}");
                assert!(!value.contains('<'), "{locale}");
            }
        }
        assert_eq!(
            progress_for("en", 2048, 12345, ProgressKind::Sent),
            "\u{2066}2048 / 12345\u{2069} bytes locally sent"
        );
        assert_eq!(
            progress_for("pt", 2, 4, ProgressKind::Written),
            "\u{2066}2 / 4\u{2069} bytes escritos localmente"
        );
        assert_eq!(
            progress_for("ar", 0, 1, ProgressKind::Received),
            "\u{2066}0 / 1\u{2069} بايت استُلمت محليًا"
        );
    }

    #[test]
    fn untrusted_recipient_is_isolated_without_overwriting_raw_identity() {
        let hostile = "PC\u{2069}\u{202e}<i>fake</i>\u{202c}.png";
        for locale in LOCALES {
            let value = recipient_for(locale, hostile);
            assert!(
                value.contains("\u{2068}PC<i>fake</i>.png\u{2069}"),
                "{locale}"
            );
            assert!(!value.contains('\u{202e}'), "{locale}");
            assert!(!value.contains('\u{202c}'), "{locale}");
            assert!(
                recipient_for(locale, "").contains("\u{2068}\u{2069}"),
                "{locale}"
            );
        }
        assert!(hostile.contains('\u{202e}'));
        assert_eq!(recipient_for("ru", "PC"), "Получатель: \u{2068}PC\u{2069}");
        assert_eq!(
            recipient_for("pt", "PC"),
            "Destinatário: \u{2068}PC\u{2069}"
        );
        assert_eq!(recipient_for("ar", "PC"), "المستلم: \u{2068}PC\u{2069}");
        assert_eq!(
            recipient_for("pt-BR", "PC"),
            "Recipient: \u{2068}PC\u{2069}"
        );
    }
}
