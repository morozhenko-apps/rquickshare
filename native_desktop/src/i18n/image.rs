//! Static image composer captions. No clipboard, selection or transport behavior.

pub(super) const KEYS: [&str; 10] = [
    "Send image",
    "Capture a clipboard image, review it, then choose recipients.",
    "Clipboard is read only when you capture. Nothing is sent yet.",
    "Capture clipboard image",
    "Prepare image",
    "No supported PNG was captured. Use a static image up to 32 MiB and 64 million pixels.",
    "Image capture failed validation or exceeded resource limits. The previous image is kept.",
    "Image source could not be prepared. Capture it again.",
    "Review the image, then prepare and choose recipients.",
    "Image preparation did not complete. Review Recover or discard the pending preparation.",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 10] = match locale {
        "ru" => &[
            "Отправить изображение",
            "Захватите изображение из буфера обмена, проверьте его и выберите получателей.",
            "Буфер обмена читается только при захвате. Пока ничего не отправлено.",
            "Захватить изображение из буфера",
            "Подготовить изображение",
            "Не удалось получить PNG. Используйте статичное изображение до 32 MiB и 64 миллионов пикселей.",
            "Изображение не прошло проверку или превысило лимиты. Предыдущее изображение сохранено.",
            "Не удалось подготовить источник изображения. Захватите его повторно.",
            "Проверьте изображение, затем подготовьте его и выберите получателей.",
            "Подготовка изображения не завершена. Проверьте Recover или отмените ожидающую подготовку.",
        ],
        "es" => &[
            "Enviar imagen",
            "Captura una imagen del portapapeles, revísala y elige los destinatarios.",
            "El portapapeles solo se lee al capturar. Todavía no se ha enviado nada.",
            "Capturar imagen del portapapeles",
            "Preparar imagen",
            "No se capturó ningún PNG compatible. Usa una imagen estática de hasta 32 MiB y 64 millones de píxeles.",
            "La captura de imagen no superó la validación o excedió los límites. Se conserva la imagen anterior.",
            "No se pudo preparar el origen de imagen. Vuelve a capturarlo.",
            "Revisa la imagen, luego prepárala y elige los destinatarios.",
            "No se completó la preparación de la imagen. Revisa Recover o descarta la preparación pendiente.",
        ],
        "pt" => &[
            "Enviar imagem",
            "Capture uma imagem da área de transferência, reveja-a e escolha os destinatários.",
            "A área de transferência só é lida ao capturar. Ainda não foi enviado nada.",
            "Capturar imagem da área de transferência",
            "Preparar imagem",
            "Não foi capturado nenhum PNG compatível. Utilize uma imagem estática até 32 MiB e 64 milhões de píxeis.",
            "A captura falhou a validação ou excedeu os limites. A imagem anterior é mantida.",
            "Não foi possível preparar a origem da imagem. Capture-a novamente.",
            "Reveja a imagem, depois prepare-a e escolha os destinatários.",
            "A preparação da imagem não foi concluída. Consulte Recover ou descarte a preparação pendente.",
        ],
        "de" => &[
            "Bild senden",
            "Erfassen Sie ein Bild aus der Zwischenablage, prüfen Sie es und wählen Sie Empfänger aus.",
            "Die Zwischenablage wird nur beim Erfassen gelesen. Es wurde noch nichts gesendet.",
            "Zwischenablagebild erfassen",
            "Bild vorbereiten",
            "Kein unterstütztes PNG erfasst. Verwenden Sie ein statisches Bild bis 32 MiB und 64 Millionen Pixel.",
            "Die Bilderfassung konnte nicht validiert werden oder überschritt Ressourcenlimits. Das vorherige Bild bleibt erhalten.",
            "Bildquelle konnte nicht vorbereitet werden. Erfassen Sie das Bild erneut.",
            "Prüfen Sie das Bild, bereiten Sie es vor und wählen Sie Empfänger aus.",
            "Die Bildvorbereitung wurde nicht abgeschlossen. Prüfen Sie Recover oder verwerfen Sie die ausstehende Vorbereitung.",
        ],
        "fr" => &[
            "Envoyer une image",
            "Capturez une image du presse-papiers, vérifiez-la, puis choisissez les destinataires.",
            "Le presse-papiers n’est lu que lors de la capture. Rien n’a encore été envoyé.",
            "Capturer une image du presse-papiers",
            "Préparer l’image",
            "Aucun PNG compatible capturé. Utilisez une image statique de 32 MiB maximum et 64 millions de pixels.",
            "La capture a échoué à la validation ou dépassé les limites. L’image précédente est conservée.",
            "Impossible de préparer la source de l’image. Capturez-la à nouveau.",
            "Vérifiez l’image, puis préparez-la et choisissez les destinataires.",
            "La préparation de l’image n’a pas abouti. Vérifiez Recover ou abandonnez la préparation en attente.",
        ],
        "it" => &[
            "Invia immagine",
            "Acquisisci un’immagine dagli appunti, controllala e scegli i destinatari.",
            "Gli appunti vengono letti solo durante l’acquisizione. Non è stato ancora inviato nulla.",
            "Acquisisci immagine dagli appunti",
            "Prepara immagine",
            "Nessun PNG supportato acquisito. Usa un’immagine statica fino a 32 MiB e 64 milioni di pixel.",
            "L’acquisizione non ha superato la verifica o ha oltrepassato i limiti. L’immagine precedente è conservata.",
            "Impossibile preparare la sorgente dell’immagine. Acquisiscila di nuovo.",
            "Controlla l’immagine, poi preparala e scegli i destinatari.",
            "Preparazione dell’immagine non completata. Controlla Recover o scarta la preparazione in sospeso.",
        ],
        "pl" => &[
            "Wyślij obraz",
            "Przechwyć obraz ze schowka, sprawdź go, a następnie wybierz odbiorców.",
            "Schowek jest odczytywany tylko przy przechwytywaniu. Nic jeszcze nie wysłano.",
            "Przechwyć obraz ze schowka",
            "Przygotuj obraz",
            "Nie przechwycono obsługiwanego PNG. Użyj nieruchomego obrazu do 32 MiB i 64 milionów pikseli.",
            "Przechwycony obraz nie przeszedł weryfikacji lub przekroczył limity. Poprzedni obraz zachowano.",
            "Nie udało się przygotować źródła obrazu. Przechwyć go ponownie.",
            "Sprawdź obraz, przygotuj go i wybierz odbiorców.",
            "Nie ukończono przygotowania obrazu. Sprawdź Recover lub odrzuć oczekujące przygotowanie.",
        ],
        "hi" => &[
            "छवि भेजें",
            "क्लिपबोर्ड से छवि कैप्चर करें, जाँचें, फिर प्राप्तकर्ता चुनें।",
            "क्लिपबोर्ड केवल कैप्चर करने पर पढ़ा जाता है। अभी कुछ भेजा नहीं गया है।",
            "क्लिपबोर्ड छवि कैप्चर करें",
            "छवि तैयार करें",
            "समर्थित PNG कैप्चर नहीं हुआ। अधिकतम 32 MiB और 64 मिलियन पिक्सेल की स्थिर छवि इस्तेमाल करें।",
            "छवि सत्यापन में विफल रही या संसाधन सीमा पार हुई। पिछली छवि सुरक्षित रखी गई है।",
            "छवि स्रोत तैयार नहीं किया जा सका। फिर से कैप्चर करें।",
            "छवि जाँचें, फिर तैयार करें और प्राप्तकर्ता चुनें।",
            "छवि तैयार करना पूरा नहीं हुआ। Recover देखें या लंबित तैयारी छोड़ें।",
        ],
        "zh-Hant" => &[
            "傳送圖片",
            "從剪貼簿擷取圖片、檢查內容，然後選擇接收對象。",
            "只有擷取時才會讀取剪貼簿。尚未傳送任何內容。",
            "擷取剪貼簿圖片",
            "準備圖片",
            "未擷取到支援的 PNG。請使用不超過 32 MiB 及 64 百萬像素的靜態圖片。",
            "圖片擷取未通過驗證或超出資源限制。先前的圖片會保留。",
            "無法準備圖片來源。請重新擷取。",
            "檢查圖片後，準備內容並選擇接收對象。",
            "圖片準備未完成。請查看 Recover 或捨棄待處理的準備工作。",
        ],
        "ja" => &[
            "画像を送信",
            "クリップボードの画像を取得して確認し、送信先を選択してください。",
            "クリップボードは取得時にのみ読み取ります。まだ何も送信していません。",
            "クリップボードの画像を取得",
            "画像を準備",
            "対応する PNG を取得できませんでした。32 MiB 以下かつ 64 百万ピクセル以下の静止画像を使用してください。",
            "画像の検証に失敗したか、リソース制限を超えました。以前の画像は保持されています。",
            "画像ソースを準備できませんでした。再度取得してください。",
            "画像を確認し、準備して送信先を選択してください。",
            "画像の準備が完了しませんでした。Recover を確認するか、保留中の準備を破棄してください。",
        ],
        "nl" => &[
            "Afbeelding verzenden",
            "Leg een afbeelding van het klembord vast, controleer deze en kies ontvangers.",
            "Het klembord wordt alleen bij vastleggen gelezen. Er is nog niets verzonden.",
            "Klembordafbeelding vastleggen",
            "Afbeelding voorbereiden",
            "Geen ondersteunde PNG vastgelegd. Gebruik een statische afbeelding tot 32 MiB en 64 miljoen pixels.",
            "De afbeelding kon niet worden gevalideerd of overschreed de limieten. De vorige afbeelding blijft behouden.",
            "Afbeeldingsbron kon niet worden voorbereid. Leg opnieuw vast.",
            "Controleer de afbeelding, bereid deze voor en kies ontvangers.",
            "Afbeeldingsvoorbereiding niet voltooid. Controleer Recover of verwijder de wachtende voorbereiding.",
        ],
        "sv" => &[
            "Skicka bild",
            "Hämta en bild från urklippet, granska den och välj mottagare.",
            "Urklippet läses bara när du hämtar bilden. Inget har skickats än.",
            "Hämta bild från urklipp",
            "Förbered bild",
            "Ingen PNG som stöds hämtades. Använd en statisk bild på högst 32 MiB och 64 miljoner pixlar.",
            "Bilden klarade inte valideringen eller överskred resursgränserna. Föregående bild behålls.",
            "Bildkällan kunde inte förberedas. Hämta bilden igen.",
            "Granska bilden, förbered den sedan och välj mottagare.",
            "Bildförberedelsen slutfördes inte. Kontrollera Recover eller kasta den väntande förberedelsen.",
        ],
        "tr" => &[
            "Görsel gönder",
            "Panodan görsel yakalayın, inceleyin ve alıcıları seçin.",
            "Pano yalnızca yakalama sırasında okunur. Henüz hiçbir şey gönderilmedi.",
            "Panodaki görseli yakala",
            "Görseli hazırla",
            "Desteklenen PNG yakalanamadı. En fazla 32 MiB ve 64 milyon piksel boyutunda durağan görsel kullanın.",
            "Görsel doğrulaması başarısız oldu veya kaynak sınırları aşıldı. Önceki görsel korunur.",
            "Görsel kaynağı hazırlanamadı. Yeniden yakalayın.",
            "Görseli inceleyin, ardından hazırlayın ve alıcıları seçin.",
            "Görsel hazırlığı tamamlanmadı. Recover seçeneğini inceleyin veya bekleyen hazırlığı iptal edin.",
        ],
        "ko" => &[
            "이미지 보내기",
            "클립보드 이미지를 가져와 확인한 다음 수신자를 선택하세요.",
            "클립보드는 가져올 때만 읽습니다. 아직 아무것도 전송되지 않았습니다.",
            "클립보드 이미지 가져오기",
            "이미지 준비",
            "지원되는 PNG를 가져오지 못했습니다. 최대 32 MiB 및 64백만 픽셀의 정지 이미지를 사용하세요.",
            "이미지가 검증을 통과하지 못했거나 리소스 제한을 초과했습니다. 이전 이미지는 유지됩니다.",
            "이미지 원본을 준비하지 못했습니다. 다시 가져오세요.",
            "이미지를 확인한 다음 준비하고 수신자를 선택하세요.",
            "이미지 준비가 완료되지 않았습니다. Recover를 확인하거나 대기 중인 준비를 폐기하세요.",
        ],
        "bn" => &[
            "ছবি পাঠান",
            "ক্লিপবোর্ড থেকে ছবি নিন, যাচাই করুন, তারপর প্রাপক নির্বাচন করুন।",
            "ক্লিপবোর্ড শুধু ছবি নেওয়ার সময় পড়া হয়। এখনও কিছু পাঠানো হয়নি।",
            "ক্লিপবোর্ডের ছবি নিন",
            "ছবি প্রস্তুত করুন",
            "সমর্থিত PNG নেওয়া যায়নি। সর্বোচ্চ 32 MiB এবং 64 মিলিয়ন পিক্সেলের স্থির ছবি ব্যবহার করুন।",
            "ছবির যাচাই ব্যর্থ হয়েছে বা সীমা ছাড়িয়েছে। আগের ছবি রাখা হয়েছে।",
            "ছবির উৎস প্রস্তুত করা যায়নি। আবার নিন।",
            "ছবি যাচাই করে প্রস্তুত করুন ও প্রাপক নির্বাচন করুন।",
            "ছবির প্রস্তুতি শেষ হয়নি। Recover দেখুন বা অপেক্ষমান প্রস্তুতি বাতিল করুন।",
        ],
        "th" => &[
            "ส่งรูปภาพ",
            "จับภาพจากคลิปบอร์ด ตรวจสอบ แล้วเลือกผู้รับ",
            "ระบบจะอ่านคลิปบอร์ดเฉพาะเมื่อจับภาพเท่านั้น ยังไม่มีการส่งข้อมูล",
            "จับภาพจากคลิปบอร์ด",
            "เตรียมรูปภาพ",
            "ไม่ได้รับ PNG ที่รองรับ ใช้ภาพนิ่งขนาดไม่เกิน 32 MiB และ 64 ล้านพิกเซล",
            "การจับภาพไม่ผ่านการตรวจสอบหรือเกินข้อจำกัด ภาพก่อนหน้ายังคงอยู่",
            "ไม่สามารถเตรียมแหล่งรูปภาพได้ โปรดจับภาพใหม่",
            "ตรวจสอบรูปภาพ แล้วเตรียมและเลือกผู้รับ",
            "การเตรียมรูปภาพยังไม่เสร็จ ตรวจสอบ Recover หรือยกเลิกการเตรียมที่ค้างอยู่",
        ],
        "id" => &[
            "Kirim gambar",
            "Ambil gambar dari papan klip, tinjau, lalu pilih penerima.",
            "Papan klip hanya dibaca saat mengambil gambar. Belum ada yang dikirim.",
            "Ambil gambar papan klip",
            "Siapkan gambar",
            "Tidak ada PNG yang didukung. Gunakan gambar statis hingga 32 MiB dan 64 juta piksel.",
            "Pengambilan gambar gagal divalidasi atau melebihi batas. Gambar sebelumnya tetap disimpan.",
            "Sumber gambar tidak dapat disiapkan. Ambil lagi.",
            "Tinjau gambar, lalu siapkan dan pilih penerima.",
            "Penyiapan gambar belum selesai. Periksa Recover atau buang penyiapan yang tertunda.",
        ],
        "fil" => &[
            "Magpadala ng larawan",
            "Kumuha ng larawan mula sa clipboard, suriin ito, at piliin ang mga tatanggap.",
            "Binabasa lang ang clipboard kapag kumukuha ng larawan. Wala pang naipapadala.",
            "Kumuha ng larawan sa clipboard",
            "Ihanda ang larawan",
            "Walang sinusuportahang PNG na nakuha. Gumamit ng static na larawan hanggang 32 MiB at 64 milyong pixel.",
            "Hindi pumasa sa validation o lumampas sa limitasyon ang larawan. Nananatili ang dating larawan.",
            "Hindi maihanda ang pinagmulan ng larawan. Kunin ito muli.",
            "Suriin ang larawan, pagkatapos ay ihanda at piliin ang mga tatanggap.",
            "Hindi natapos ang paghahanda ng larawan. Tingnan ang Recover o itapon ang nakabinbing paghahanda.",
        ],
        "ar" => &[
            "إرسال صورة",
            "التقط صورة من الحافظة وراجعها ثم اختر المستلمين.",
            "لا تُقرأ الحافظة إلا عند الالتقاط. لم يُرسل أي شيء بعد.",
            "التقاط صورة من الحافظة",
            "تجهيز الصورة",
            "لم تُلتقط صورة PNG مدعومة. استخدم صورة ثابتة بحجم لا يتجاوز 32 MiB و64 مليون بكسل.",
            "فشل التحقق من الصورة أو تجاوزت حدود الموارد. يُحتفظ بالصورة السابقة.",
            "تعذّر تجهيز مصدر الصورة. التقطها مرة أخرى.",
            "راجع الصورة ثم جهّزها واختر المستلمين.",
            "لم يكتمل تجهيز الصورة. راجع Recover أو تجاهل التجهيز المعلّق.",
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
    fn image_catalog_is_complete_and_limits_remain_exact() {
        for (index, key) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(key));
            assert_eq!(translated("en", key), Some(*key));
            for locale in LOCALES {
                let text = translated(locale, key).expect("known image key");
                assert!(!text.trim().is_empty(), "{locale}: {key}");
                assert!(!text.contains('{'), "{locale}: {key}");
            }
        }
        for locale in LOCALES {
            let limit = translated(locale, KEYS[5]).expect("image size warning");
            assert!(limit.contains("PNG"), "{locale}: PNG format");
            assert!(limit.contains("32 MiB"), "{locale}: size limit");
            assert!(limit.contains("64"), "{locale}: pixel limit");
            assert_ne!(translated(locale, KEYS[2]), translated(locale, KEYS[8]));
            assert_ne!(translated(locale, KEYS[3]), translated(locale, KEYS[4]));
            assert_ne!(translated(locale, KEYS[6]), translated(locale, KEYS[7]));
            assert_ne!(translated(locale, KEYS[7]), translated(locale, KEYS[9]));
        }
    }

    #[test]
    fn image_privacy_and_error_captions_have_known_portuguese_russian_readings() {
        assert_eq!(
            translated("ru", "Send image"),
            Some("Отправить изображение")
        );
        assert_eq!(translated("pt", "Send image"), Some("Enviar imagem"));
        assert_eq!(translated("pt", "Prepare image"), Some("Preparar imagem"));
        assert_eq!(
            translated("ru", "Prepare image"),
            Some("Подготовить изображение")
        );
        assert_eq!(
            translated("no-such-locale", "Send image"),
            Some("Send image")
        );
        assert_eq!(translated("ar", "Send image"), Some("إرسال صورة"));
    }
}
