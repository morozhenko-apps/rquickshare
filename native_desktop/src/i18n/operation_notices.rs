//! Display-only notices for outbound preparation and card lifecycle failures.
//! The untranslated canonical source remains the model's status token.

pub(super) const KEYS: [&str; 6] = [
    "Content preparation did not complete. Review Recover or discard the pending preparation.",
    "The target or connection changed. Content was kept; choose a device explicitly to send it.",
    "Clipboard content is unavailable, unsupported or exceeds limits. Nothing was sent; review any pending preparation before retrying.",
    "Card dismissal was not confirmed. It may appear again after reconnecting.",
    "Card dismissal was not queued. Review the panel before reconnecting.",
    "Cannot place the card on an available monitor. Open the tray to review requests.",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 6] = match locale {
        "ru" => &[
            "Подготовка содержимого не завершилась. Проверьте ожидающую подготовку: восстановите её или отмените.",
            "Получатель или соединение изменились. Содержимое сохранено; явно выберите устройство для отправки.",
            "Содержимое буфера обмена недоступно, не поддерживается или превышает лимит. Ничего не отправлено; перед повтором проверьте ожидающую подготовку.",
            "Закрытие карточки не подтверждено. После повторного подключения она может появиться снова.",
            "Запрос на закрытие карточки не поставлен в очередь. Проверьте панель перед повторным подключением.",
            "Не удалось разместить карточку на доступном мониторе. Откройте трей для просмотра запросов.",
],
        "es" => &[
            "La preparación del contenido no terminó. Revisa la preparación pendiente para recuperarla o descartarla.",
            "El destinatario o la conexión cambió. El contenido se conservó; elige explícitamente un dispositivo para enviarlo.",
            "El portapapeles no está disponible, no es compatible o supera los límites. No se envió nada; revisa la preparación pendiente antes de reintentar.",
            "No se confirmó el cierre de la tarjeta. Puede volver a aparecer tras la reconexión.",
            "No se puso en cola el cierre de la tarjeta. Revisa el panel antes de reconectar.",
            "No se pudo colocar la tarjeta en ningún monitor disponible. Abre la bandeja para revisar las solicitudes.",
],
        "pt" => &[
            "A preparação do conteúdo não foi concluída. Reveja a preparação pendente para a recuperar ou descartar.",
            "O destinatário ou a ligação mudou. O conteúdo foi mantido; escolha explicitamente um dispositivo para o enviar.",
            "O conteúdo da área de transferência está indisponível, não é suportado ou excede os limites. Nada foi enviado; reveja a preparação pendente antes de tentar novamente.",
            "O encerramento do cartão não foi confirmado. Poderá reaparecer após restabelecer a ligação.",
            "O encerramento do cartão não entrou na fila. Reveja o painel antes de restabelecer a ligação.",
            "Não foi possível posicionar o cartão num monitor disponível. Abra a área de notificação para rever os pedidos.",
],
        "de" => &[
            "Die Inhaltsvorbereitung wurde nicht abgeschlossen. Prüfen Sie die ausstehende Vorbereitung und stellen Sie sie wieder her oder verwerfen Sie sie.",
            "Empfänger oder Verbindung haben sich geändert. Der Inhalt blieb erhalten; wählen Sie das Gerät zum Senden ausdrücklich aus.",
            "Der Zwischenablageinhalt ist nicht verfügbar, nicht unterstützt oder zu groß. Es wurde nichts gesendet; prüfen Sie vor dem Wiederholen die ausstehende Vorbereitung.",
            "Das Schließen der Karte wurde nicht bestätigt. Nach erneuter Verbindung kann sie wieder erscheinen.",
            "Das Schließen der Karte wurde nicht eingereiht. Prüfen Sie das Panel vor der erneuten Verbindung.",
            "Die Karte konnte auf keinem verfügbaren Bildschirm platziert werden. Öffnen Sie das Tray, um Anfragen zu prüfen.",
],
        "fr" => &[
            "La préparation du contenu n'a pas abouti. Vérifiez la préparation en attente pour la récupérer ou l'abandonner.",
            "Le destinataire ou la connexion a changé. Le contenu a été conservé ; choisissez explicitement un appareil pour l'envoyer.",
            "Le presse-papiers est indisponible, non pris en charge ou dépasse les limites. Rien n'a été envoyé ; vérifiez la préparation en attente avant de réessayer.",
            "La fermeture de la carte n'a pas été confirmée. Elle peut réapparaître après reconnexion.",
            "La fermeture de la carte n'a pas été mise en attente. Vérifiez le panneau avant de vous reconnecter.",
            "Impossible de placer la carte sur un écran disponible. Ouvrez la zone de notification pour consulter les demandes.",
],
        "it" => &[
            "La preparazione del contenuto non è stata completata. Controlla la preparazione in sospeso per recuperarla o scartarla.",
            "Il destinatario o la connessione è cambiato. Il contenuto è stato conservato; scegli esplicitamente un dispositivo per inviarlo.",
            "Il contenuto degli appunti non è disponibile, non è supportato o supera i limiti. Non è stato inviato nulla; controlla la preparazione in sospeso prima di riprovare.",
            "La chiusura della scheda non è stata confermata. Potrebbe riapparire dopo la riconnessione.",
            "La chiusura della scheda non è stata accodata. Controlla il pannello prima di riconnetterti.",
            "Impossibile posizionare la scheda su un monitor disponibile. Apri l'area di notifica per controllare le richieste.",
],
        "pl" => &[
            "Przygotowanie zawartości nie zostało zakończone. Sprawdź oczekujące przygotowanie, aby je odzyskać lub odrzucić.",
            "Odbiorca lub połączenie się zmieniło. Zawartość zachowano; wyraźnie wybierz urządzenie do wysłania.",
            "Zawartość schowka jest niedostępna, nieobsługiwana lub przekracza limity. Nic nie wysłano; sprawdź oczekujące przygotowanie przed ponowieniem.",
            "Zamknięcie karty nie zostało potwierdzone. Może pojawić się ponownie po połączeniu.",
            "Nie dodano zamknięcia karty do kolejki. Sprawdź panel przed ponownym połączeniem.",
            "Nie można umieścić karty na dostępnym monitorze. Otwórz zasobnik systemowy, aby przejrzeć żądania.",
],
        "hi" => &[
            "सामग्री की तैयारी पूरी नहीं हुई। लंबित तैयारी की जाँच करें और उसे पुनर्स्थापित करें या छोड़ दें।",
            "प्राप्तकर्ता या कनेक्शन बदल गया। सामग्री सुरक्षित है; भेजने के लिए स्पष्ट रूप से एक डिवाइस चुनें।",
            "क्लिपबोर्ड सामग्री अनुपलब्ध, असमर्थित या सीमा से अधिक है। कुछ भी नहीं भेजा गया; दोबारा कोशिश से पहले लंबित तैयारी जाँचें।",
            "कार्ड बंद होने की पुष्टि नहीं हुई। फिर से कनेक्ट होने पर यह दोबारा दिख सकता है।",
            "कार्ड बंद करने का अनुरोध कतार में नहीं गया। फिर से कनेक्ट होने से पहले पैनल जाँचें।",
            "कार्ड को उपलब्ध मॉनिटर पर नहीं रखा जा सका। अनुरोध देखने के लिए सिस्टम ट्रे खोलें।",
],
        "zh-Hant" => &[
            "內容準備未完成。請檢查待處理的準備作業，選擇復原或捨棄。",
            "收件裝置或連線已變更。內容已保留；請明確選取要傳送的裝置。",
            "剪貼簿內容無法使用、不受支援或超過限制。尚未傳送任何內容；重試前請檢查待處理的準備作業。",
            "尚未確認卡片已關閉。重新連線後，卡片可能再次出現。",
            "卡片關閉要求未加入佇列。重新連線前請檢查面板。",
            "無法將卡片放在可用的螢幕上。請開啟系統匣查看要求。",
],
        "ja" => &[
            "コンテンツの準備が完了しませんでした。保留中の準備を確認し、復元または破棄してください。",
            "送信先または接続が変わりました。コンテンツは保持されています。送信先デバイスを明示的に選択してください。",
            "クリップボードの内容が利用できない、非対応、または制限超過です。何も送信されていません。再試行前に保留中の準備を確認してください。",
            "カードが閉じられたことを確認できませんでした。再接続後に再表示される場合があります。",
            "カードを閉じる要求はキューに追加されませんでした。再接続前にパネルを確認してください。",
            "利用可能なモニターにカードを配置できませんでした。システムトレイを開いて要求を確認してください。",
],
        "nl" => &[
            "De voorbereiding van de inhoud is niet voltooid. Controleer de wachtende voorbereiding om die te herstellen of te verwijderen.",
            "De ontvanger of verbinding is gewijzigd. De inhoud bleef bewaard; kies expliciet een apparaat om te verzenden.",
            "De klembordinhoud is niet beschikbaar, wordt niet ondersteund of overschrijdt de limiet. Er is niets verzonden; controleer de wachtende voorbereiding voordat je het opnieuw probeert.",
            "Het sluiten van de kaart is niet bevestigd. De kaart kan na opnieuw verbinden terugkomen.",
            "Het sluiten van de kaart is niet in de wachtrij geplaatst. Controleer het paneel voordat je opnieuw verbindt.",
            "De kaart kan niet op een beschikbaar scherm worden geplaatst. Open het systeemvak om verzoeken te bekijken.",
],
        "sv" => &[
            "Förberedelsen av innehållet slutfördes inte. Granska den väntande förberedelsen för att återställa eller kasta bort den.",
            "Mottagaren eller anslutningen ändrades. Innehållet behölls; välj uttryckligen en enhet att skicka till.",
            "Urklippsinnehållet är otillgängligt, stöds inte eller överskrider gränserna. Inget skickades; kontrollera den väntande förberedelsen innan du försöker igen.",
            "Stängningen av kortet bekräftades inte. Det kan visas igen efter återanslutning.",
            "Stängningen av kortet köades inte. Granska panelen innan du återansluter.",
            "Det gick inte att placera kortet på en tillgänglig skärm. Öppna systemfältet för att granska förfrågningar.",
],
        "tr" => &[
            "İçerik hazırlığı tamamlanmadı. Bekleyen hazırlığı geri almak veya silmek için kontrol edin.",
            "Hedef veya bağlantı değişti. İçerik korundu; göndermek için açıkça bir cihaz seçin.",
            "Pano içeriği kullanılamıyor, desteklenmiyor veya sınırı aşıyor. Hiçbir şey gönderilmedi; yeniden denemeden önce bekleyen hazırlığı kontrol edin.",
            "Kartın kapatıldığı doğrulanmadı. Yeniden bağlandıktan sonra tekrar görünebilir.",
            "Kart kapatma isteği kuyruğa alınmadı. Yeniden bağlanmadan önce paneli kontrol edin.",
            "Kart kullanılabilir bir ekrana yerleştirilemedi. İstekleri görmek için sistem tepsisini açın.",
],
        "ko" => &[
            "콘텐츠 준비가 완료되지 않았습니다. 보류 중인 준비를 확인하여 복구하거나 폐기하세요.",
            "대상 또는 연결이 변경되었습니다. 콘텐츠는 보존되었습니다. 보낼 기기를 명시적으로 선택하세요.",
            "클립보드 콘텐츠를 사용할 수 없거나 지원하지 않거나 제한을 초과했습니다. 전송된 것은 없습니다. 다시 시도하기 전에 보류 중인 준비를 확인하세요.",
            "카드 닫기가 확인되지 않았습니다. 다시 연결한 후 재등장할 수 있습니다.",
            "카드 닫기 요청이 대기열에 추가되지 않았습니다. 다시 연결하기 전에 패널을 확인하세요.",
            "사용 가능한 모니터에 카드를 배치할 수 없습니다. 시스템 트레이를 열어 요청을 확인하세요.",
],
        "bn" => &[
            "বিষয়বস্তু প্রস্তুতি সম্পূর্ণ হয়নি। অপেক্ষমাণ প্রস্তুতি পরীক্ষা করে পুনরুদ্ধার বা বাতিল করুন।",
            "প্রাপক বা সংযোগ বদলেছে। বিষয়বস্তু সংরক্ষিত আছে; পাঠাতে স্পষ্টভাবে একটি ডিভাইস বেছে নিন।",
            "ক্লিপবোর্ডের বিষয়বস্তু অনুপলব্ধ, অসমর্থিত বা সীমার বেশি। কিছুই পাঠানো হয়নি; আবার চেষ্টা করার আগে অপেক্ষমাণ প্রস্তুতি পরীক্ষা করুন।",
            "কার্ড বন্ধ হওয়া নিশ্চিত করা যায়নি। পুনরায় সংযোগের পরে আবার দেখা যেতে পারে।",
            "কার্ড বন্ধ করার অনুরোধ সারিতে যায়নি। পুনরায় সংযোগের আগে প্যানেল পরীক্ষা করুন।",
            "কার্ডটি উপলব্ধ মনিটরে দেখানো যায়নি। অনুরোধ দেখতে সিস্টেম ট্রে খুলুন।",
],
        "th" => &[
            "การเตรียมเนื้อหาไม่เสร็จสมบูรณ์ ตรวจสอบรายการเตรียมที่ค้างอยู่เพื่อกู้คืนหรือยกเลิก",
            "ผู้รับหรือการเชื่อมต่อเปลี่ยนไป เนื้อหายังถูกเก็บไว้ โปรดเลือกอุปกรณ์ที่จะส่งอย่างชัดเจน",
            "เนื้อหาคลิปบอร์ดไม่พร้อมใช้ ไม่รองรับ หรือเกินขีดจำกัด ยังไม่ได้ส่งสิ่งใด ตรวจสอบการเตรียมที่ค้างอยู่ก่อนลองอีกครั้ง",
            "ยังไม่ได้ยืนยันว่าปิดการ์ดแล้ว การ์ดอาจปรากฏอีกหลังเชื่อมต่อใหม่",
            "คำขอปิดการ์ดไม่ได้เข้าคิว ตรวจสอบแผงก่อนเชื่อมต่อใหม่",
            "ไม่สามารถวางการ์ดบนหน้าจอที่พร้อมใช้งาน เปิดถาดระบบเพื่อดูคำขอ",
],
        "id" => &[
            "Penyiapan konten tidak selesai. Periksa penyiapan tertunda untuk memulihkan atau membuangnya.",
            "Tujuan atau koneksi berubah. Konten tetap tersimpan; pilih perangkat secara tegas untuk mengirimnya.",
            "Konten papan klip tidak tersedia, tidak didukung, atau melebihi batas. Tidak ada yang dikirim; periksa penyiapan tertunda sebelum mencoba lagi.",
            "Penutupan kartu belum dikonfirmasi. Kartu mungkin muncul lagi setelah tersambung ulang.",
            "Penutupan kartu tidak masuk antrean. Periksa panel sebelum menyambung ulang.",
            "Kartu tidak dapat ditempatkan pada monitor yang tersedia. Buka baki sistem untuk meninjau permintaan.",
],
        "fil" => &[
            "Hindi natapos ang paghahanda ng nilalaman. Suriin ang nakabinbing paghahanda para ibalik o itapon ito.",
            "Nagbago ang tatanggap o koneksyon. Napanatili ang nilalaman; tahasang pumili ng device na padadalhan.",
            "Hindi available, hindi suportado, o lampas sa limitasyon ang clipboard. Walang naipadala; suriin ang nakabinbing paghahanda bago subukang muli.",
            "Hindi nakumpirma ang pagsasara ng card. Maaari itong lumitaw muli pagkatapos muling kumonekta.",
            "Hindi naipasok sa pila ang pagsasara ng card. Suriin ang panel bago muling kumonekta.",
            "Hindi mailagay ang card sa available na monitor. Buksan ang system tray para suriin ang mga kahilingan.",
],
        "ar" => &[
            "لم يكتمل إعداد المحتوى. راجع الإعداد المعلّق لاستعادته أو تجاهله.",
            "تغيّر المستلم أو الاتصال. تم الاحتفاظ بالمحتوى؛ اختر جهازًا صراحةً لإرساله.",
            "محتوى الحافظة غير متاح أو غير مدعوم أو يتجاوز الحدود. لم يُرسل أي شيء؛ راجع الإعداد المعلّق قبل إعادة المحاولة.",
            "لم يتأكد إغلاق البطاقة. قد تظهر مرة أخرى بعد إعادة الاتصال.",
            "لم يُدرج طلب إغلاق البطاقة في قائمة الانتظار. راجع اللوحة قبل إعادة الاتصال.",
            "تعذّر وضع البطاقة على شاشة متاحة. افتح منطقة النظام لمراجعة الطلبات.",
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
    fn given_twenty_locales_when_rendering_operation_errors_then_every_key_is_distinct() {
        assert_eq!(KEYS.len(), 6);
        for (index, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(key));
            for locale in LOCALES {
                let text = translated(locale, key).expect("known operation error");
                assert!(!text.trim().is_empty(), "{locale}: {key}");
                assert!(!text.contains('{'), "{locale}: {key}");
            }
        }
        for locale in LOCALES {
            let value = |index| translated(locale, KEYS[index]).expect("known error");
            for first in 0..KEYS.len() {
                for second in (first + 1)..KEYS.len() {
                    assert_ne!(value(first), value(second), "{locale}: {first} and {second}");
                }
            }
        }
        assert_eq!(translated("en", KEYS[2]), Some(KEYS[2]));
        assert_eq!(translated("pt-BR", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("zh-Hans", KEYS[5]), Some(KEYS[5]));
        assert_eq!(translated("other", "Uncataloged notice"), None);
    }

    #[test]
    fn given_user_visible_failures_when_localized_then_recipients_and_dismissal_remain_explicit() {
        assert_eq!(
            translated("ru", KEYS[1]),
            Some("Получатель или соединение изменились. Содержимое сохранено; явно выберите устройство для отправки.")
        );
        assert_eq!(
            translated("pt", KEYS[2]),
            Some("O conteúdo da área de transferência está indisponível, não é suportado ou excede os limites. Nada foi enviado; reveja a preparação pendente antes de tentar novamente.")
        );
        assert_eq!(
            translated("ar", KEYS[3]),
            Some("لم يتأكد إغلاق البطاقة. قد تظهر مرة أخرى بعد إعادة الاتصال.")
        );
        for locale in LOCALES {
            assert_ne!(translated(locale, KEYS[3]), translated(locale, KEYS[4]), "{locale}");
            assert_ne!(translated(locale, KEYS[1]), translated(locale, KEYS[2]), "{locale}");
        }
    }
}
