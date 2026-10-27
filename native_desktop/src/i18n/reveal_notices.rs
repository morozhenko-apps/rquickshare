//! Capability-guarded received-file reveal notices, display only.

pub(super) const KEYS: [&str; 7] = [
    "Cannot show this file right now. Review the current transfer and try again.",
    "Cannot show this file. It may have moved, changed, or become unavailable.",
    "Cannot access this received file right now.",
    "Containing folder shown.",
    "Folder request cancelled.",
    "Your desktop could not show this folder. Try again.",
    "Opening containing folder…",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 7] = match locale {
        "ru" => &[
            "Сейчас нельзя показать этот файл. Проверьте текущую передачу и повторите попытку.",
            "Нельзя показать файл. Возможно, он перемещён, изменён или недоступен.",
            "Сейчас нет доступа к полученному файлу.",
            "Папка с файлом открыта.",
            "Запрос на открытие папки отменён.",
            "Рабочий стол не смог открыть эту папку. Повторите попытку.",
            "Открытие папки с файлом…",
        ],
        "es" => &[
            "No se puede mostrar este archivo ahora. Revisa la transferencia actual y vuelve a intentarlo.",
            "No se puede mostrar este archivo. Puede haberse movido, cambiado o dejado de estar disponible.",
            "No se puede acceder ahora a este archivo recibido.",
            "Se mostró la carpeta contenedora.",
            "Solicitud de carpeta cancelada.",
            "El escritorio no pudo mostrar esta carpeta. Inténtalo de nuevo.",
            "Abriendo la carpeta contenedora…",
        ],
        "pt" => &[
            "Não é possível mostrar este ficheiro agora. Reveja a transferência atual e tente novamente.",
            "Não é possível mostrar este ficheiro. Pode ter sido movido, alterado ou estar indisponível.",
            "Não é possível aceder agora a este ficheiro recebido.",
            "Pasta que contém o ficheiro apresentada.",
            "Pedido de abertura da pasta cancelado.",
            "O ambiente de trabalho não conseguiu mostrar esta pasta. Tente novamente.",
            "A abrir a pasta que contém o ficheiro…",
        ],
        "de" => &[
            "Diese Datei kann gerade nicht angezeigt werden. Prüfen Sie die aktuelle Übertragung und versuchen Sie es erneut.",
            "Diese Datei kann nicht angezeigt werden. Sie wurde möglicherweise verschoben, geändert oder ist nicht mehr verfügbar.",
            "Auf die empfangene Datei kann derzeit nicht zugegriffen werden.",
            "Enthaltender Ordner angezeigt.",
            "Ordneranforderung abgebrochen.",
            "Die Desktopumgebung konnte diesen Ordner nicht anzeigen. Versuchen Sie es erneut.",
            "Enthaltenden Ordner öffnen…",
        ],
        "fr" => &[
            "Impossible d’afficher ce fichier pour le moment. Vérifiez le transfert actuel et réessayez.",
            "Impossible d’afficher ce fichier. Il a peut-être été déplacé, modifié ou rendu indisponible.",
            "Impossible d’accéder à ce fichier reçu pour le moment.",
            "Dossier contenant le fichier affiché.",
            "Demande d’ouverture du dossier annulée.",
            "Le bureau n’a pas pu afficher ce dossier. Réessayez.",
            "Ouverture du dossier contenant le fichier…",
        ],
        "it" => &[
            "Impossibile mostrare questo file ora. Controlla il trasferimento corrente e riprova.",
            "Impossibile mostrare questo file. Potrebbe essere stato spostato, modificato o non essere più disponibile.",
            "Impossibile accedere ora al file ricevuto.",
            "Cartella contenente il file mostrata.",
            "Richiesta di apertura cartella annullata.",
            "Il desktop non ha potuto mostrare questa cartella. Riprova.",
            "Apertura della cartella contenente il file…",
        ],
        "pl" => &[
            "Nie można teraz pokazać tego pliku. Sprawdź bieżący transfer i spróbuj ponownie.",
            "Nie można pokazać tego pliku. Być może został przeniesiony, zmieniony lub jest niedostępny.",
            "Nie można teraz uzyskać dostępu do odebranego pliku.",
            "Wyświetlono folder zawierający plik.",
            "Żądanie otwarcia folderu anulowano.",
            "Pulpit nie mógł wyświetlić tego folderu. Spróbuj ponownie.",
            "Otwieranie folderu zawierającego plik…",
        ],
        "hi" => &[
            "अभी यह फ़ाइल दिखाई नहीं जा सकती। वर्तमान ट्रांसफ़र की जाँच करें और फिर कोशिश करें।",
            "यह फ़ाइल दिखाई नहीं जा सकती। संभव है यह हटाई, बदली या अनुपलब्ध हो गई हो।",
            "अभी प्राप्त फ़ाइल तक पहुँच नहीं है।",
            "फ़ाइल वाला फ़ोल्डर दिखाया गया।",
            "फ़ोल्डर खोलने का अनुरोध रद्द हुआ।",
            "डेस्कटॉप इस फ़ोल्डर को नहीं दिखा सका। फिर कोशिश करें।",
            "फ़ाइल वाला फ़ोल्डर खोल रहे हैं…",
        ],
        "zh-Hant" => &[
            "目前無法顯示此檔案。請檢查目前的傳輸並重試。",
            "無法顯示此檔案。檔案可能已移動、變更或無法使用。",
            "目前無法存取這個已接收的檔案。",
            "已顯示檔案所在的資料夾。",
            "已取消開啟資料夾的要求。",
            "桌面環境無法顯示此資料夾。請重試。",
            "正在開啟檔案所在的資料夾…",
        ],
        "ja" => &[
            "現在このファイルを表示できません。転送状況を確認して再試行してください。",
            "このファイルを表示できません。移動、変更、または利用不可になった可能性があります。",
            "現在、受信したファイルにアクセスできません。",
            "ファイルのあるフォルダーを表示しました。",
            "フォルダーを開く要求をキャンセルしました。",
            "デスクトップでこのフォルダーを表示できませんでした。再試行してください。",
            "ファイルのあるフォルダーを開いています…",
        ],
        "nl" => &[
            "Kan dit bestand nu niet tonen. Controleer de huidige overdracht en probeer opnieuw.",
            "Kan dit bestand niet tonen. Het is mogelijk verplaatst, gewijzigd of niet meer beschikbaar.",
            "Kan momenteel niet bij dit ontvangen bestand.",
            "De map met het bestand is weergegeven.",
            "Mapverzoek geannuleerd.",
            "De desktop kon deze map niet tonen. Probeer opnieuw.",
            "De map met het bestand openen…",
        ],
        "sv" => &[
            "Kan inte visa filen just nu. Kontrollera den aktuella överföringen och försök igen.",
            "Kan inte visa filen. Den kan ha flyttats, ändrats eller blivit otillgänglig.",
            "Kan inte komma åt den mottagna filen just nu.",
            "Mappen som innehåller filen har visats.",
            "Mappbegäran avbruten.",
            "Skrivbordsmiljön kunde inte visa mappen. Försök igen.",
            "Öppnar mappen som innehåller filen…",
        ],
        "tr" => &[
            "Bu dosya şu anda gösterilemiyor. Geçerli aktarımı kontrol edip yeniden deneyin.",
            "Bu dosya gösterilemiyor. Taşınmış, değiştirilmiş veya kullanılamaz olmuş olabilir.",
            "Alınan dosyaya şu anda erişilemiyor.",
            "Dosyayı içeren klasör gösterildi.",
            "Klasör isteği iptal edildi.",
            "Masaüstü bu klasörü gösteremedi. Yeniden deneyin.",
            "Dosyayı içeren klasör açılıyor…",
        ],
        "ko" => &[
            "지금은 이 파일을 표시할 수 없습니다. 현재 전송 상태를 확인하고 다시 시도하세요.",
            "이 파일을 표시할 수 없습니다. 이동되었거나 변경되었거나 사용할 수 없을 수 있습니다.",
            "지금은 받은 파일에 접근할 수 없습니다.",
            "파일이 있는 폴더를 표시했습니다.",
            "폴더 요청이 취소되었습니다.",
            "데스크톱에서 이 폴더를 표시하지 못했습니다. 다시 시도하세요.",
            "파일이 있는 폴더 여는 중…",
        ],
        "bn" => &[
            "এই মুহূর্তে ফাইলটি দেখানো যাচ্ছে না। বর্তমান স্থানান্তর যাচাই করে আবার চেষ্টা করুন।",
            "ফাইলটি দেখানো যাচ্ছে না। এটি সরানো, বদলানো বা অনুপলব্ধ হতে পারে।",
            "এই মুহূর্তে প্রাপ্ত ফাইলে প্রবেশ করা যাচ্ছে না।",
            "ফাইলের ফোল্ডার দেখানো হয়েছে।",
            "ফোল্ডারের অনুরোধ বাতিল হয়েছে।",
            "ডেস্কটপ এই ফোল্ডার দেখাতে পারেনি। আবার চেষ্টা করুন।",
            "ফাইলের ফোল্ডার খোলা হচ্ছে…",
        ],
        "th" => &[
            "ตอนนี้แสดงไฟล์นี้ไม่ได้ ตรวจสอบการถ่ายโอนปัจจุบันแล้วลองอีกครั้ง",
            "แสดงไฟล์นี้ไม่ได้ ไฟล์อาจถูกย้าย เปลี่ยนแปลง หรือไม่พร้อมใช้งาน",
            "ตอนนี้เข้าถึงไฟล์ที่ได้รับไม่ได้",
            "แสดงโฟลเดอร์ที่มีไฟล์แล้ว",
            "ยกเลิกคำขอเปิดโฟลเดอร์แล้ว",
            "เดสก์ท็อปไม่สามารถแสดงโฟลเดอร์นี้ได้ ลองอีกครั้ง",
            "กำลังเปิดโฟลเดอร์ที่มีไฟล์…",
        ],
        "id" => &[
            "Tidak dapat menampilkan file ini sekarang. Periksa transfer saat ini dan coba lagi.",
            "Tidak dapat menampilkan file ini. Mungkin telah dipindah, diubah, atau tidak tersedia.",
            "Tidak dapat mengakses file yang diterima saat ini.",
            "Folder yang berisi file telah ditampilkan.",
            "Permintaan folder dibatalkan.",
            "Desktop tidak dapat menampilkan folder ini. Coba lagi.",
            "Membuka folder yang berisi file…",
        ],
        "fil" => &[
            "Hindi maipakita ang file na ito ngayon. Suriin ang kasalukuyang paglilipat at subukang muli.",
            "Hindi maipakita ang file na ito. Maaaring inilipat, binago o hindi na available.",
            "Hindi ma-access ngayon ang natanggap na file.",
            "Naipakita ang folder na naglalaman ng file.",
            "Kinansela ang kahilingan sa folder.",
            "Hindi maipakita ng desktop ang folder na ito. Subukang muli.",
            "Binubuksan ang folder na naglalaman ng file…",
        ],
        "ar" => &[
            "لا يمكن إظهار هذا الملف الآن. راجع عملية النقل الحالية وحاول مجددًا.",
            "لا يمكن إظهار هذا الملف. ربما نُقل أو تغيّر أو أصبح غير متاح.",
            "لا يمكن الوصول إلى الملف المستلَم الآن.",
            "تم إظهار المجلد الذي يحتوي على الملف.",
            "تم إلغاء طلب فتح المجلد.",
            "تعذّر على سطح المكتب إظهار هذا المجلد. حاول مجددًا.",
            "جارٍ فتح المجلد الذي يحتوي على الملف…",
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
    fn reveal_notice_catalog_is_complete_and_has_fallback() {
        for (index, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(key), "duplicate reveal notice");
            assert_eq!(translated("en", key), Some(*key));
            for locale in LOCALES {
                let value = translated(locale, key).expect("known reveal notice");
                assert!(!value.trim().is_empty(), "{locale}: {key}");
                assert!(!value.contains('{'), "{locale}: {key}");
            }
        }
        assert_eq!(translated("pt-BR", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("zh-Hans", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("unknown", "Show in folder"), None);
    }

    #[test]
    fn cancelled_pending_success_and_portal_failure_do_not_collide() {
        for locale in LOCALES {
            let value = |index| translated(locale, KEYS[index]).expect("known reveal notice");
            for (first, second) in [(0, 1), (1, 2), (2, 3), (3, 4), (3, 5), (3, 6), (4, 6)] {
                assert_ne!(value(first), value(second), "{locale}: {first} vs {second}");
            }
        }
        assert_eq!(
            translated("ru", KEYS[4]),
            Some("Запрос на открытие папки отменён.")
        );
        assert_eq!(
            translated("pt", KEYS[3]),
            Some("Pasta que contém o ficheiro apresentada.")
        );
        assert_eq!(
            translated("ar", KEYS[6]),
            Some("جارٍ فتح المجلد الذي يحتوي على الملف…")
        );
    }
}
