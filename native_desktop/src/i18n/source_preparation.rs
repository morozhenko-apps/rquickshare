//! Localized prepared-source and main-card accessibility presentation.
//! Source ownership, discard action and discovery/visibility state are unchanged.

pub(super) const KEYS: [&str; 8] = [
    "Retry connection",
    "Choose target devices",
    "Clear prepared files",
    "prepared · choose a device",
    "Hide devices and transfers",
    "Show devices and transfers",
    "Visible for one minute — press again to stop receiving requests",
    "Review an incoming request or become visible for one minute",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 8] = match locale {
        "ru" => &[
            "Повторить подключение",
            "Выберите устройства-получатели",
            "Удалить подготовленные файлы",
            "подготовлено · выберите устройство",
            "Скрыть устройства и передачи",
            "Показать устройства и передачи",
            "Видимость на одну минуту — нажмите снова, чтобы прекратить приём запросов",
            "Просмотреть входящий запрос или включить видимость на одну минуту",
        ],
        "es" => &[
            "Reintentar conexión",
            "Elige los dispositivos de destino",
            "Descartar los archivos preparados",
            "preparado · elige un dispositivo",
            "Ocultar dispositivos y transferencias",
            "Mostrar dispositivos y transferencias",
            "Visible durante un minuto: pulsa de nuevo para dejar de recibir solicitudes",
            "Revisa una solicitud entrante o hazte visible durante un minuto",
        ],
        "pt" => &[
            "Tentar novamente a ligação",
            "Escolha os dispositivos de destino",
            "Descartar ficheiros preparados",
            "preparado · escolha um dispositivo",
            "Ocultar dispositivos e transferências",
            "Mostrar dispositivos e transferências",
            "Visível durante um minuto — prima novamente para parar de receber pedidos",
            "Reveja um pedido recebido ou fique visível durante um minuto",
        ],
        "de" => &[
            "Verbindung erneut versuchen",
            "Zielgeräte auswählen",
            "Vorbereitete Dateien verwerfen",
            "vorbereitet · Gerät auswählen",
            "Geräte und Übertragungen ausblenden",
            "Geräte und Übertragungen anzeigen",
            "Eine Minute sichtbar – erneut drücken, um Anfragen nicht mehr zu empfangen",
            "Eingehende Anfrage prüfen oder für eine Minute sichtbar werden",
        ],
        "fr" => &[
            "Réessayer la connexion",
            "Choisissez les appareils destinataires",
            "Supprimer les fichiers préparés",
            "préparé · choisissez un appareil",
            "Masquer les appareils et transferts",
            "Afficher les appareils et transferts",
            "Visible pendant une minute — appuyez de nouveau pour cesser de recevoir des demandes",
            "Examinez une demande entrante ou devenez visible pendant une minute",
        ],
        "it" => &[
            "Riprova la connessione",
            "Scegli i dispositivi destinatari",
            "Elimina i file preparati",
            "preparato · scegli un dispositivo",
            "Nascondi dispositivi e trasferimenti",
            "Mostra dispositivi e trasferimenti",
            "Visibile per un minuto — premi di nuovo per interrompere le richieste",
            "Controlla una richiesta in arrivo o renditi visibile per un minuto",
        ],
        "pl" => &[
            "Ponów połączenie",
            "Wybierz urządzenia docelowe",
            "Odrzuć przygotowane pliki",
            "przygotowano · wybierz urządzenie",
            "Ukryj urządzenia i transfery",
            "Pokaż urządzenia i transfery",
            "Widoczność przez minutę — naciśnij ponownie, aby zatrzymać odbieranie próśb",
            "Sprawdź przychodzącą prośbę lub włącz widoczność na minutę",
        ],
        "hi" => &[
            "कनेक्शन दोबारा आज़माएं",
            "लक्ष्य डिवाइस चुनें",
            "तैयार की गई फ़ाइलें हटाएं",
            "तैयार है · डिवाइस चुनें",
            "डिवाइस और स्थानांतरण छिपाएं",
            "डिवाइस और स्थानांतरण दिखाएं",
            "एक मिनट तक दिखाई देगा — अनुरोध लेना बंद करने के लिए फिर दबाएं",
            "आने वाले अनुरोध की समीक्षा करें या एक मिनट के लिए दिखाई दें",
        ],
        "zh-Hant" => &[
            "重試連線",
            "選擇接收裝置",
            "捨棄已準備的檔案",
            "已準備 · 選擇裝置",
            "隱藏裝置和傳輸",
            "顯示裝置和傳輸",
            "顯示一分鐘 — 再按一次即可停止接收要求",
            "檢查傳入要求，或顯示一分鐘",
        ],
        "ja" => &[
            "接続を再試行",
            "送信先デバイスを選択",
            "準備したファイルを破棄",
            "準備完了 · デバイスを選択",
            "デバイスと転送を隠す",
            "デバイスと転送を表示",
            "1分間検出可能 — もう一度押すとリクエストの受信を停止",
            "受信リクエストを確認するか、1分間検出可能にする",
        ],
        "nl" => &[
            "Verbinding opnieuw proberen",
            "Kies doelapparaten",
            "Voorbereide bestanden verwijderen",
            "voorbereid · kies een apparaat",
            "Apparaten en overdrachten verbergen",
            "Apparaten en overdrachten tonen",
            "Eén minuut zichtbaar — druk opnieuw om verzoeken te stoppen",
            "Bekijk een inkomend verzoek of word één minuut zichtbaar",
        ],
        "sv" => &[
            "Försök ansluta igen",
            "Välj målenheter",
            "Kasta förberedda filer",
            "förberett · välj en enhet",
            "Dölj enheter och överföringar",
            "Visa enheter och överföringar",
            "Synlig i en minut – tryck igen för att sluta ta emot förfrågningar",
            "Granska en inkommande förfrågan eller bli synlig i en minut",
        ],
        "tr" => &[
            "Bağlantıyı yeniden dene",
            "Hedef cihazları seçin",
            "Hazırlanan dosyaları kaldırın",
            "hazırlandı · cihaz seçin",
            "Cihazları ve aktarımları gizle",
            "Cihazları ve aktarımları göster",
            "Bir dakika görünür — istek almayı durdurmak için tekrar basın",
            "Gelen isteği inceleyin veya bir dakika görünür olun",
        ],
        "ko" => &[
            "연결 다시 시도",
            "대상 기기 선택",
            "준비된 파일 삭제",
            "준비됨 · 기기 선택",
            "기기와 전송 숨기기",
            "기기와 전송 표시",
            "1분 동안 표시 — 요청 수신을 중지하려면 다시 누르세요",
            "받은 요청을 검토하거나 1분 동안 기기를 표시하세요",
        ],
        "bn" => &[
            "আবার সংযোগের চেষ্টা করুন",
            "গন্তব্য ডিভাইস বেছে নিন",
            "প্রস্তুত করা ফাইল বাতিল করুন",
            "প্রস্তুত · ডিভাইস বেছে নিন",
            "ডিভাইস ও স্থানান্তর লুকান",
            "ডিভাইস ও স্থানান্তর দেখান",
            "এক মিনিট দৃশ্যমান — অনুরোধ গ্রহণ বন্ধ করতে আবার চাপুন",
            "আসা অনুরোধ পর্যালোচনা করুন বা এক মিনিট দৃশ্যমান থাকুন",
        ],
        "th" => &[
            "ลองเชื่อมต่ออีกครั้ง",
            "เลือกอุปกรณ์ปลายทาง",
            "ยกเลิกไฟล์ที่เตรียมไว้",
            "เตรียมแล้ว · เลือกอุปกรณ์",
            "ซ่อนอุปกรณ์และการถ่ายโอน",
            "แสดงอุปกรณ์และการถ่ายโอน",
            "มองเห็นได้หนึ่งนาที — กดอีกครั้งเพื่อหยุดรับคำขอ",
            "ตรวจสอบคำขอขาเข้าหรือเปิดการมองเห็นหนึ่งนาที",
        ],
        "id" => &[
            "Coba sambungkan kembali",
            "Pilih perangkat tujuan",
            "Buang berkas yang disiapkan",
            "disiapkan · pilih perangkat",
            "Sembunyikan perangkat dan transfer",
            "Tampilkan perangkat dan transfer",
            "Terlihat selama satu menit — tekan lagi untuk berhenti menerima permintaan",
            "Tinjau permintaan masuk atau tampilkan diri selama satu menit",
        ],
        "fil" => &[
            "Subukang kumonekta muli",
            "Piliin ang mga target na device",
            "Itapon ang mga inihandang file",
            "handa na · pumili ng device",
            "Itago ang mga device at paglilipat",
            "Ipakita ang mga device at paglilipat",
            "Nakikita sa loob ng isang minuto — pindutin muli upang ihinto ang mga kahilingan",
            "Suriin ang papasok na kahilingan o maging nakikita sa loob ng isang minuto",
        ],
        "ar" => &[
            "إعادة محاولة الاتصال",
            "اختر الأجهزة المستقبلة",
            "تجاهل الملفات المُعدّة",
            "تم الإعداد · اختر جهازًا",
            "إخفاء الأجهزة وعمليات النقل",
            "إظهار الأجهزة وعمليات النقل",
            "مرئي لمدة دقيقة — اضغط مجددًا لإيقاف استقبال الطلبات",
            "راجع طلبًا واردًا أو اجعل جهازك مرئيًا لمدة دقيقة",
        ],
        _ => &KEYS,
    };
    Some(row[index])
}

pub(super) fn files_for(locale: &str, file_count: u32) -> String {
    let files = super::dynamic::file_count_for(locale, file_count as usize);
    let caption = translated(locale, KEYS[3]).unwrap();
    format!("{files} · {caption}")
}

pub(super) fn content_for(locale: &str, is_link: bool, byte_count: u64) -> String {
    let kind = super::translated(locale, if is_link { "Link" } else { "Text" });
    let bytes = super::dynamic::bytes_for(byte_count);
    let caption = translated(locale, KEYS[3]).unwrap();
    format!("{kind} · {bytes} · {caption}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn prepared_controls_have_exact_disjoint_keys_and_complete_locale_rows() {
        assert_eq!(KEYS.len(), 8);
        for (i, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..i].contains(key));
            for locale in LOCALES {
                let value = translated(locale, key).expect("known source key");
                assert!(!value.trim().is_empty(), "{locale}: {key}");
                assert!(!value.contains("<"), "{locale}: {key}");
                assert!(!value.contains("{"), "{locale}: {key}");
            }
            assert_eq!(translated("en", key), Some(*key));
            assert_eq!(translated("pt-BR", key), Some(*key));
            assert_eq!(translated("zh-Hans", key), Some(*key));
        }
        assert_ne!(translated("ar", KEYS[2]), translated("ar", KEYS[1]));
        assert_ne!(translated("pt", KEYS[4]), translated("pt", KEYS[5]));
        assert_eq!(translated("xx", "not a key"), None);
    }

    #[test]
    fn source_summaries_keep_typed_counts_across_all_locales_and_extremes() {
        for locale in LOCALES {
            for count in [0, 1, 2, 5, u16::MAX as u32, u32::MAX] {
                let summary = files_for(locale, count);
                let expected = super::super::dynamic::file_count_for(locale, count as usize);
                assert!(summary.starts_with(&expected), "{locale}: {count}");
                assert!(summary.ends_with(translated(locale, KEYS[3]).unwrap()));
            }
            for bytes in [0, 1, 2, 1024, u64::MAX] {
                for is_link in [false, true] {
                    let summary = content_for(locale, is_link, bytes);
                    let source = if is_link { "Link" } else { "Text" };
                    let kind = super::super::translated(locale, source);
                    assert!(summary.starts_with(kind));
                    assert!(summary.contains(&super::super::dynamic::bytes_for(bytes)));
                    assert!(summary.ends_with(translated(locale, KEYS[3]).unwrap()));
                }
            }
        }
        assert_eq!(files_for("en", 2), "2 files · prepared · choose a device");
        assert_eq!(
            content_for("en", false, 3),
            "Text · 3 B · prepared · choose a device"
        );
        assert_eq!(
            content_for("en", true, 0),
            "Link · 0 B · prepared · choose a device"
        );
    }

    #[test]
    fn given_locale_and_kind_when_prepared_then_exact_words_and_counts_are_stable() {
        // Golden values are independent of the formatter under test.
        let file_cases = [
            ("en", 0, "0 files · prepared · choose a device"),
            ("ru", 1, "1 файл · подготовлено · выберите устройство"),
            ("ru", 2, "2 файла · подготовлено · выберите устройство"),
            ("ru", 5, "5 файлов · подготовлено · выберите устройство"),
            ("pt", 0, "0 ficheiros · preparado · escolha um dispositivo"),
            ("pt", 1, "1 ficheiro · preparado · escolha um dispositivo"),
            ("pt", 2, "2 ficheiros · preparado · escolha um dispositivo"),
            ("ar", 2, "2 ملفان · تم الإعداد · اختر جهازًا"),
            ("zh-Hant", 2, "2 個檔案 · 已準備 · 選擇裝置"),
            ("pt-BR", 1, "1 file · prepared · choose a device"),
            ("zh-Hans", 1, "1 file · prepared · choose a device"),
        ];
        for (locale, count, expected) in file_cases {
            assert_eq!(files_for(locale, count), expected, "{locale}: {count}");
        }

        let content_cases = [
            ("ru", false, 12, "Текст · 12 B · подготовлено · выберите устройство"),
            ("ru", true, 7, "Ссылка · 7 B · подготовлено · выберите устройство"),
            ("pt", false, 12, "Texto · 12 B · preparado · escolha um dispositivo"),
            ("pt", true, 7, "Ligação · 7 B · preparado · escolha um dispositivo"),
            ("ar", true, 0, "رابط · 0 B · تم الإعداد · اختر جهازًا"),
            ("zh-Hans", false, 0, "Text · 0 B · prepared · choose a device"),
        ];
        for (locale, link, bytes, expected) in content_cases {
            assert_eq!(
                content_for(locale, link, bytes),
                expected,
                "{locale}: {link} / {bytes}"
            );
            assert_ne!(
                content_for(locale, !link, bytes),
                expected,
                "{locale}: kind must not alias"
            );
        }
    }
}
