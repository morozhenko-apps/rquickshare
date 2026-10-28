//! Details-card captions; display only. Consent, PIN admission and manifest actions
//! are unchanged. Sender and file names are isolated by the caller.

pub(super) const KEYS: [&str; 4] = [
    "Waiting for sender and payload details",
    "This payload cannot be reviewed safely. Decline or cancel it.",
    "advertised device",
    "Review all files",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 4] = match locale {
        "ru" => &[
            "Ожидание сведений об отправителе и содержимом",
            "Нельзя безопасно просмотреть это содержимое. Отклоните или отмените передачу.",
            "объявленное устройство",
            "Просмотреть все файлы",
        ],
        "es" => &[
            "Esperando datos del remitente y del contenido",
            "Este contenido no puede revisarse de forma segura. Recházalo o cancélalo.",
            "dispositivo anunciado",
            "Revisar todos los archivos",
        ],
        "pt" => &[
            "À espera de informações do remetente e do conteúdo",
            "Não é possível rever este conteúdo em segurança. Recuse ou cancele a transferência.",
            "dispositivo anunciado",
            "Rever todos os ficheiros",
        ],
        "de" => &[
            "Warten auf Absender- und Inhaltsdetails",
            "Dieser Inhalt kann nicht sicher geprüft werden. Lehnen Sie ab oder brechen Sie ab.",
            "angekündigtes Gerät",
            "Alle Dateien prüfen",
        ],
        "fr" => &[
            "En attente des détails de l’expéditeur et du contenu",
            "Ce contenu ne peut pas être examiné en toute sécurité. Refusez ou annulez.",
            "appareil annoncé",
            "Examiner tous les fichiers",
        ],
        "it" => &[
            "In attesa dei dettagli sul mittente e sul contenuto",
            "Impossibile esaminare il contenuto in modo sicuro. Rifiuta o annulla.",
            "dispositivo annunciato",
            "Esamina tutti i file",
        ],
        "pl" => &[
            "Oczekiwanie na dane nadawcy i zawartości",
            "Nie można bezpiecznie sprawdzić tej zawartości. Odrzuć lub anuluj transfer.",
            "zgłoszone urządzenie",
            "Przejrzyj wszystkie pliki",
        ],
        "hi" => &[
            "भेजने वाले और सामग्री के विवरण की प्रतीक्षा",
            "इस सामग्री की सुरक्षित समीक्षा नहीं हो सकती। इसे अस्वीकार करें या रद्द करें।",
            "विज्ञापित डिवाइस",
            "सभी फ़ाइलों की समीक्षा करें",
        ],
        "zh-Hant" => &[
            "等待傳送者及內容詳細資料",
            "無法安全檢視此內容。請拒絕或取消傳輸。",
            "廣播中的裝置",
            "檢視所有檔案",
        ],
        "ja" => &[
            "送信元と内容の詳細を待機中",
            "この内容は安全に確認できません。拒否するかキャンセルしてください。",
            "通知されたデバイス",
            "すべてのファイルを確認",
        ],
        "nl" => &[
            "Wachten op gegevens van afzender en inhoud",
            "Deze inhoud kan niet veilig worden bekeken. Weiger of annuleer de overdracht.",
            "aangemeld apparaat",
            "Alle bestanden bekijken",
        ],
        "sv" => &[
            "Väntar på avsändar- och innehållsinformation",
            "Innehållet kan inte granskas säkert. Avvisa eller avbryt överföringen.",
            "annonserad enhet",
            "Granska alla filer",
        ],
        "tr" => &[
            "Gönderen ve içerik ayrıntıları bekleniyor",
            "Bu içerik güvenle incelenemiyor. Reddedin veya aktarımı iptal edin.",
            "duyurulan cihaz",
            "Tüm dosyaları incele",
        ],
        "ko" => &[
            "보낸 사람과 콘텐츠 세부정보 대기 중",
            "이 콘텐츠를 안전하게 검토할 수 없습니다. 거부하거나 전송을 취소하세요.",
            "광고된 기기",
            "모든 파일 검토",
        ],
        "bn" => &[
            "প্রেরক ও বিষয়বস্তুর বিবরণের অপেক্ষা",
            "এই বিষয়বস্তু নিরাপদে পর্যালোচনা করা যাচ্ছে না। প্রত্যাখ্যান বা বাতিল করুন।",
            "ঘোষিত ডিভাইস",
            "সব ফাইল পর্যালোচনা করুন",
        ],
        "th" => &[
            "กำลังรอข้อมูลผู้ส่งและเนื้อหา",
            "ไม่สามารถตรวจสอบเนื้อหานี้ได้อย่างปลอดภัย โปรดปฏิเสธหรือยกเลิก",
            "อุปกรณ์ที่ประกาศ",
            "ตรวจสอบไฟล์ทั้งหมด",
        ],
        "id" => &[
            "Menunggu detail pengirim dan konten",
            "Konten ini tidak dapat ditinjau dengan aman. Tolak atau batalkan transfer.",
            "perangkat yang diumumkan",
            "Tinjau semua berkas",
        ],
        "fil" => &[
            "Naghihintay ng detalye ng nagpadala at nilalaman",
            "Hindi ligtas suriin ang nilalamang ito. Tanggihan o kanselahin ang paglilipat.",
            "inihayag na device",
            "Suriin ang lahat ng file",
        ],
        "ar" => &[
            "في انتظار تفاصيل المرسل والمحتوى",
            "لا يمكن مراجعة هذا المحتوى بأمان. ارفض النقل أو ألغِه.",
            "جهاز مُعلن عنه",
            "مراجعة جميع الملفات",
        ],
        _ => &KEYS,
    };
    Some(row[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn details_labels_are_complete_and_preserve_reject_versus_waiting() {
        assert_eq!(KEYS.len(), 4);
        for (i, source) in KEYS.iter().enumerate() {
            assert!(!KEYS[..i].contains(source));
            for locale in LOCALES {
                let caption = translated(locale, source).expect("known key");
                assert!(!caption.trim().is_empty(), "{locale}: {source}");
                assert!(!caption.contains('<'), "{locale}: {source}");
                assert!(!caption.contains('{'), "{locale}: {source}");
            }
            assert_eq!(translated("en", source), Some(*source));
            assert_eq!(translated("pt-BR", source), Some(*source));
            assert_eq!(translated("zh-Hans", source), Some(*source));
        }
        for locale in LOCALES {
            assert_ne!(
                translated(locale, KEYS[0]),
                translated(locale, KEYS[1]),
                "{locale}: waiting must not look like an unsafe payload"
            );
        }
        assert_eq!(translated("ru", KEYS[3]), Some("Просмотреть все файлы"));
        assert_eq!(translated("pt", KEYS[2]), Some("dispositivo anunciado"));
        assert_eq!(translated("ar", KEYS[3]), Some("مراجعة جميع الملفات"));
        assert_eq!(translated("xx", "a missing key"), None);
    }
}
