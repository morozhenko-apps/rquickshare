//! Display-only labels for native transfer cards and explicit consent controls.
//! The original actions, eligibility, file counters and consent revisions remain authoritative.

pub(super) const KEYS: [&str; 8] = [
    "Preview only",
    "Waiting for complete service state",
    "Decline",
    "Accept",
    "No complete files were saved. Ask the sender to send again.",
    "Completed files were kept. Ask the sender to resend the remaining files.",
    "Review text",
    "Cancel folder request",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 8] = match locale {
        "ru" => &[
            "Только предпросмотр",
            "Ожидание полного состояния службы",
            "Отклонить",
            "Принять",
            "Ни один файл не был полностью сохранён. Попросите отправителя повторить отправку.",
            "Завершённые файлы сохранены. Попросите отправителя повторно отправить остальные файлы.",
            "Просмотреть текст",
            "Отменить запрос на открытие папки",
        ],
        "es" => &[
            "Solo vista previa",
            "Esperando el estado completo del servicio",
            "Rechazar",
            "Aceptar",
            "No se guardó ningún archivo completo. Pide al remitente que vuelva a enviarlos.",
            "Se conservaron los archivos completados. Pide al remitente que vuelva a enviar los restantes.",
            "Revisar texto",
            "Cancelar solicitud de carpeta",
        ],
        "pt" => &[
            "Apenas pré-visualização",
            "À espera do estado completo do serviço",
            "Recusar",
            "Aceitar",
            "Não foi guardado nenhum ficheiro completo. Peça ao remetente para enviar novamente.",
            "Os ficheiros concluídos foram mantidos. Peça ao remetente para reenviar os restantes.",
            "Rever texto",
            "Cancelar pedido de abertura da pasta",
        ],
        "de" => &[
            "Nur Vorschau",
            "Warten auf vollständigen Dienststatus",
            "Ablehnen",
            "Annehmen",
            "Keine vollständige Datei wurde gespeichert. Bitten Sie den Sender, erneut zu senden.",
            "Vollständige Dateien wurden behalten. Bitten Sie den Sender, die übrigen Dateien erneut zu senden.",
            "Text prüfen",
            "Ordneranfrage abbrechen",
        ],
        "fr" => &[
            "Aperçu uniquement",
            "En attente de l'état complet du service",
            "Refuser",
            "Accepter",
            "Aucun fichier complet n'a été enregistré. Demandez à l'expéditeur de renvoyer les fichiers.",
            "Les fichiers terminés ont été conservés. Demandez à l'expéditeur de renvoyer les autres.",
            "Consulter le texte",
            "Annuler la demande de dossier",
        ],
        "it" => &[
            "Solo anteprima",
            "In attesa dello stato completo del servizio",
            "Rifiuta",
            "Accetta",
            "Nessun file completo è stato salvato. Chiedi al mittente di inviarlo di nuovo.",
            "I file completati sono stati conservati. Chiedi al mittente di inviare di nuovo i rimanenti.",
            "Rivedi testo",
            "Annulla richiesta cartella",
        ],
        "pl" => &[
            "Tylko podgląd",
            "Oczekiwanie na pełny stan usługi",
            "Odrzuć",
            "Akceptuj",
            "Nie zapisano żadnego kompletnego pliku. Poproś nadawcę o ponowne wysłanie.",
            "Ukończone pliki zachowano. Poproś nadawcę o ponowne wysłanie pozostałych.",
            "Przejrzyj tekst",
            "Anuluj żądanie folderu",
        ],
        "hi" => &[
            "केवल पूर्वावलोकन",
            "सेवा की पूर्ण स्थिति की प्रतीक्षा",
            "अस्वीकार करें",
            "स्वीकार करें",
            "कोई पूरी फ़ाइल सहेजी नहीं गई। भेजने वाले से दोबारा भेजने को कहें।",
            "पूरी हुई फ़ाइलें सुरक्षित रखी गई हैं। भेजने वाले से बाकी फ़ाइलें दोबारा भेजने को कहें।",
            "टेक्स्ट की समीक्षा करें",
            "फ़ोल्डर अनुरोध रद्द करें",
        ],
        "zh-Hant" => &[
            "僅預覽",
            "等待完整的服務狀態",
            "拒絕",
            "接受",
            "沒有儲存任何完整檔案。請傳送者重新傳送。",
            "已保留完成的檔案。請傳送者重新傳送剩餘檔案。",
            "檢閱文字",
            "取消資料夾要求",
        ],
        "ja" => &[
            "プレビューのみ",
            "サービスの完全な状態を待機中",
            "拒否",
            "承認",
            "完全なファイルは保存されませんでした。送信元に再送を依頼してください。",
            "完了したファイルは保持されました。残りのファイルを再送するよう送信元に依頼してください。",
            "テキストを確認",
            "フォルダー要求をキャンセル",
        ],
        "nl" => &[
            "Alleen voorbeeld",
            "Wachten op volledige dienststatus",
            "Weigeren",
            "Accepteren",
            "Er zijn geen volledige bestanden opgeslagen. Vraag de afzender opnieuw te verzenden.",
            "Voltooide bestanden zijn bewaard. Vraag de afzender de overige bestanden opnieuw te verzenden.",
            "Tekst bekijken",
            "Mapverzoek annuleren",
        ],
        "sv" => &[
            "Endast förhandsvisning",
            "Väntar på fullständigt tjänstetillstånd",
            "Avböj",
            "Acceptera",
            "Inga fullständiga filer sparades. Be avsändaren skicka igen.",
            "Färdiga filer behölls. Be avsändaren skicka resterande filer igen.",
            "Granska text",
            "Avbryt mappbegäran",
        ],
        "tr" => &[
            "Yalnızca önizleme",
            "Tam hizmet durumu bekleniyor",
            "Reddet",
            "Kabul et",
            "Tamamlanmış hiçbir dosya kaydedilmedi. Göndericiden yeniden göndermesini isteyin.",
            "Tamamlanan dosyalar korundu. Göndericiden kalan dosyaları yeniden göndermesini isteyin.",
            "Metni incele",
            "Klasör isteğini iptal et",
        ],
        "ko" => &[
            "미리 보기 전용",
            "서비스 전체 상태 대기 중",
            "거절",
            "수락",
            "완료된 파일이 저장되지 않았습니다. 발신자에게 다시 보내 달라고 요청하세요.",
            "완료된 파일은 유지되었습니다. 발신자에게 나머지 파일의 재전송을 요청하세요.",
            "텍스트 확인",
            "폴더 요청 취소",
        ],
        "bn" => &[
            "শুধু পূর্বরূপ",
            "পরিষেবার পূর্ণ অবস্থার অপেক্ষা",
            "প্রত্যাখ্যান",
            "গ্রহণ",
            "কোনো সম্পূর্ণ ফাইল সংরক্ষিত হয়নি। প্রেরককে আবার পাঠাতে বলুন।",
            "সম্পূর্ণ ফাইলগুলো রাখা হয়েছে। প্রেরককে বাকি ফাইল আবার পাঠাতে বলুন।",
            "লেখা পর্যালোচনা",
            "ফোল্ডার অনুরোধ বাতিল",
        ],
        "th" => &[
            "ดูตัวอย่างเท่านั้น",
            "กำลังรอสถานะบริการที่สมบูรณ์",
            "ปฏิเสธ",
            "ยอมรับ",
            "ไม่มีไฟล์ที่เสร็จสมบูรณ์ถูกบันทึก ขอให้ผู้ส่งส่งอีกครั้ง",
            "เก็บไฟล์ที่เสร็จสมบูรณ์ไว้แล้ว ขอให้ผู้ส่งส่งไฟล์ที่เหลืออีกครั้ง",
            "ตรวจสอบข้อความ",
            "ยกเลิกคำขอโฟลเดอร์",
        ],
        "id" => &[
            "Hanya pratinjau",
            "Menunggu status layanan lengkap",
            "Tolak",
            "Terima",
            "Tidak ada file lengkap yang tersimpan. Minta pengirim mengirim ulang.",
            "File yang selesai tetap disimpan. Minta pengirim mengirim ulang sisanya.",
            "Tinjau teks",
            "Batalkan permintaan folder",
        ],
        "fil" => &[
            "Preview lang",
            "Naghihintay ng kumpletong estado ng serbisyo",
            "Tanggihan",
            "Tanggapin",
            "Walang kumpletong file na na-save. Hilingin sa nagpadala na ipadala muli.",
            "Napanatili ang mga nakumpletong file. Hilingin sa nagpadala na ipadala muli ang natitira.",
            "Suriin ang text",
            "Kanselahin ang kahilingan ng folder",
        ],
        "ar" => &[
            "معاينة فقط",
            "في انتظار الحالة الكاملة للخدمة",
            "رفض",
            "قبول",
            "لم يُحفظ أي ملف مكتمل. اطلب من المرسل إعادة الإرسال.",
            "احتُفظ بالملفات المكتملة. اطلب من المرسل إعادة إرسال الملفات المتبقية.",
            "مراجعة النص",
            "إلغاء طلب المجلد",
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
    fn given_all_locales_when_rendering_card_controls_then_every_caption_is_distinct() {
        assert_eq!(KEYS.len(), 8);
        for (index, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(key));
            for locale in LOCALES {
                let shown = translated(locale, key).expect("known card control");
                assert!(!shown.trim().is_empty(), "{locale}: {key}");
                assert!(!shown.contains('{'), "{locale}: {key}");
            }
        }
        for locale in LOCALES {
            let value = |i| translated(locale, KEYS[i]).expect("known card label");
            for (a, b) in [(0, 1), (2, 3), (4, 5), (6, 7)] {
                assert_ne!(value(a), value(b), "{locale}: {a}/{b}");
            }
        }
        assert_eq!(translated("en", KEYS[3]), Some("Accept"));
        assert_eq!(translated("pt-BR", KEYS[2]), Some(KEYS[2]));
        assert_eq!(translated("zh-Hans", KEYS[6]), Some(KEYS[6]));
        assert_eq!(translated("unknown", "Unsupported card"), None);
    }

    #[test]
    fn given_consent_and_partially_saved_files_when_localized_then_actions_stay_distinct() {
        assert_eq!(translated("ru", KEYS[2]), Some("Отклонить"));
        assert_eq!(translated("pt", KEYS[3]), Some("Aceitar"));
        assert_eq!(translated("ar", KEYS[7]), Some("إلغاء طلب المجلد"));
        assert_eq!(
            translated("ru", KEYS[4]),
            Some(
                "Ни один файл не был полностью сохранён. Попросите отправителя повторить отправку."
            )
        );
        assert_eq!(
            translated("pt", KEYS[5]),
            Some("Os ficheiros concluídos foram mantidos. Peça ao remetente para reenviar os restantes.")
        );
    }
}
