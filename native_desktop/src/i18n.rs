//! Native GTK startup-locale catalog foundation. UI strings not yet cataloged
//! deliberately retain their English source text; this cannot affect consent
//! semantics, the daemon protocol, or the user's persisted settings.
//! This is the first menu slice of #17, NOT completion of 20-locale GTK coverage.

const MENU_KEYS: [&str; 17] = [
    "Visible",
    "Hidden",
    "Receive files",
    "Send files…",
    "History",
    "Settings…",
    "Quit panel",
    "Discovery visibility",
    "Receive",
    "Send",
    "Devices",
    "Hide",
    "Nearby",
    "Incoming",
    "Add files…",
    "Folder…",
    "Cancel",
];
const ENGLISH: [&str; 17] = MENU_KEYS;

const REVIEW_KEYS: [&str; 8] = [
    "Received text",
    "Received link · plain text",
    "Copy",
    "Close",
    "Nothing has been copied. Use Copy to replace this clipboard.",
    "Copy unavailable. Unlock and review the current transfer.",
    "Copy failed. This clipboard did not accept the text.",
    "Copied to this clipboard",
];
const ENGLISH_REVIEW: [&str; 8] = REVIEW_KEYS;

/// Canonical PersonalLanding locale IDs; never infer pt-BR or zh-Hans.
pub(crate) const LOCALES: [&str; 20] = [
    "en", "ru", "es", "pt", "de", "fr", "it", "pl", "hi", "zh-Hant", "ja", "nl", "sv", "tr", "ko",
    "bn", "th", "id", "fil", "ar",
];

fn resolve(value: &str) -> Option<&'static str> {
    let short = value
        .split('.')
        .next()?
        .replace('_', "-")
        .to_ascii_lowercase();
    let short = short.split('@').next().unwrap_or(&short);
    if short == "pt-br"
        || short.starts_with("pt-br-")
        || short == "zh-cn"
        || short.starts_with("zh-cn-")
        || short == "zh-hans"
        || short.starts_with("zh-hans-")
    {
        return None;
    }
    if short == "zh-tw"
        || short.starts_with("zh-tw-")
        || short == "zh-hk"
        || short.starts_with("zh-hk-")
        || short == "zh-hant"
        || short.starts_with("zh-hant-")
    {
        return Some("zh-Hant");
    }
    let head = short.split('-').next()?;
    LOCALES.iter().copied().find(|locale| *locale == head)
}

/// System locale only, no new preferences or DBus/settings schema.
pub(crate) fn active() -> &'static str {
    for variable in ["LC_ALL", "LC_MESSAGES", "LANGUAGE", "LANG"] {
        if let Ok(value) = std::env::var(variable) {
            if value.trim().is_empty() {
                continue;
            }
            for candidate in value.split(':') {
                if let Some(locale) = resolve(candidate) {
                    return locale;
                }
            }
            // An explicitly selected unsupported locale falls back to English.
            return "en";
        }
    }
    "en"
}

pub(crate) fn label(source: &'static str) -> &'static str {
    translated(active(), source)
}

fn translated_review(locale: &str) -> &'static [&'static str; 8] {
    match locale {
        "ru" => &[
            "Полученный текст",
            "Полученная ссылка · обычный текст",
            "Копировать",
            "Закрыть",
            "Ничего не скопировано. Нажмите «Копировать», чтобы заменить содержимое буфера обмена.",
            "Копирование недоступно. Снимите блокировку и проверьте текущую передачу.",
            "Не удалось скопировать. Буфер обмена не принял текст.",
            "Скопировано в буфер обмена",
        ],
        "es" => &[
            "Texto recibido",
            "Enlace recibido · texto sin formato",
            "Copiar",
            "Cerrar",
            "No se ha copiado nada. Usa «Copiar» para sustituir el contenido del portapapeles.",
            "No se puede copiar. Desbloquea el dispositivo y comprueba la transferencia actual.",
            "No se pudo copiar. El portapapeles no aceptó el texto.",
            "Copiado al portapapeles",
        ],
        "pt" => &[
            "Texto recebido",
            "Ligação recebida · texto simples",
            "Copiar",
            "Fechar",
            "Ainda não foi copiado nada. Use «Copiar» para substituir o conteúdo da área de transferência.",
            "Não é possível copiar. Desbloqueie o dispositivo e reveja a transferência atual.",
            "Falha ao copiar. A área de transferência não aceitou o texto.",
            "Copiado para a área de transferência",
        ],
        "de" => &[
            "Empfangener Text",
            "Empfangener Link · Klartext",
            "Kopieren",
            "Schließen",
            "Es wurde nichts kopiert. Mit „Kopieren“ ersetzen Sie den Inhalt der Zwischenablage.",
            "Kopieren nicht verfügbar. Entsperren Sie das Gerät und prüfen Sie die aktuelle Übertragung.",
            "Kopieren fehlgeschlagen. Die Zwischenablage hat den Text nicht angenommen.",
            "In die Zwischenablage kopiert",
        ],
        "fr" => &[
            "Texte reçu",
            "Lien reçu · texte brut",
            "Copier",
            "Fermer",
            "Rien n’a été copié. Utilisez « Copier » pour remplacer le contenu du presse-papiers.",
            "Copie indisponible. Déverrouillez l’appareil et vérifiez le transfert en cours.",
            "Échec de la copie. Le presse-papiers n’a pas accepté le texte.",
            "Copié dans le presse-papiers",
        ],
        "it" => &[
            "Testo ricevuto",
            "Link ricevuto · testo semplice",
            "Copia",
            "Chiudi",
            "Non è stato copiato nulla. Usa «Copia» per sostituire il contenuto degli appunti.",
            "Copia non disponibile. Sblocca il dispositivo e verifica il trasferimento corrente.",
            "Copia non riuscita. Gli appunti non hanno accettato il testo.",
            "Copiato negli appunti",
        ],
        "pl" => &[
            "Odebrany tekst",
            "Odebrany link · zwykły tekst",
            "Kopiuj",
            "Zamknij",
            "Nic nie skopiowano. Wybierz „Kopiuj”, aby zastąpić zawartość schowka.",
            "Kopiowanie niedostępne. Odblokuj urządzenie i sprawdź bieżący transfer.",
            "Nie udało się skopiować. Schowek nie przyjął tekstu.",
            "Skopiowano do schowka",
        ],
        "hi" => &[
            "प्राप्त टेक्स्ट",
            "प्राप्त लिंक · सादा टेक्स्ट",
            "कॉपी करें",
            "बंद करें",
            "अभी कुछ भी कॉपी नहीं किया गया है। क्लिपबोर्ड की सामग्री बदलने के लिए «कॉपी करें» चुनें।",
            "कॉपी उपलब्ध नहीं है। डिवाइस अनलॉक करें और मौजूदा ट्रांसफ़र जाँचें।",
            "कॉपी नहीं हो सका। क्लिपबोर्ड ने टेक्स्ट स्वीकार नहीं किया।",
            "क्लिपबोर्ड पर कॉपी किया गया",
        ],
        "zh-Hant" => &[
            "已接收的文字",
            "已接收的連結 · 純文字",
            "複製",
            "關閉",
            "尚未複製任何內容。按「複製」將取代剪貼簿中的內容。",
            "無法複製。請解鎖裝置並檢查目前的傳輸。",
            "複製失敗。剪貼簿未接受文字。",
            "已複製到剪貼簿",
        ],
        "ja" => &[
            "受信したテキスト",
            "受信したリンク · プレーンテキスト",
            "コピー",
            "閉じる",
            "まだコピーしていません。「コピー」を押すとクリップボードの内容が置き換わります。",
            "コピーできません。端末のロックを解除し、現在の転送を確認してください。",
            "コピーに失敗しました。クリップボードがテキストを受け付けませんでした。",
            "クリップボードにコピーしました",
        ],
        "nl" => &[
            "Ontvangen tekst",
            "Ontvangen link · platte tekst",
            "Kopiëren",
            "Sluiten",
            "Er is niets gekopieerd. Gebruik 'Kopiëren' om de inhoud van het klembord te vervangen.",
            "Kopiëren niet beschikbaar. Ontgrendel het apparaat en controleer de huidige overdracht.",
            "Kopiëren mislukt. Het klembord accepteerde de tekst niet.",
            "Gekopieerd naar het klembord",
        ],
        "sv" => &[
            "Mottagen text",
            "Mottagen länk · oformaterad text",
            "Kopiera",
            "Stäng",
            "Ingenting har kopierats. Använd ”Kopiera” för att ersätta urklippets innehåll.",
            "Kopiering är inte tillgänglig. Lås upp enheten och kontrollera den aktuella överföringen.",
            "Kopieringen misslyckades. Urklippet tog inte emot texten.",
            "Kopierat till urklipp",
        ],
        "tr" => &[
            "Alınan metin",
            "Alınan bağlantı · düz metin",
            "Kopyala",
            "Kapat",
            "Henüz hiçbir şey kopyalanmadı. Panodaki içeriği değiştirmek için “Kopyala”yı kullanın.",
            "Kopyalama kullanılamıyor. Cihazın kilidini açın ve geçerli aktarımı kontrol edin.",
            "Kopyalama başarısız oldu. Pano metni kabul etmedi.",
            "Panoya kopyalandı",
        ],
        "ko" => &[
            "받은 텍스트",
            "받은 링크 · 일반 텍스트",
            "복사",
            "닫기",
            "아직 복사한 내용이 없습니다. '복사'를 누르면 클립보드 내용이 바뀝니다.",
            "복사할 수 없습니다. 기기 잠금을 해제하고 현재 전송을 확인하세요.",
            "복사하지 못했습니다. 클립보드가 텍스트를 받아들이지 않았습니다.",
            "클립보드에 복사했습니다",
        ],
        "bn" => &[
            "প্রাপ্ত লেখা",
            "প্রাপ্ত লিংক · সাধারণ লেখা",
            "কপি করুন",
            "বন্ধ করুন",
            "এখনও কিছু কপি করা হয়নি। ক্লিপবোর্ডের বিষয়বস্তু বদলাতে ‘কপি করুন’ চাপুন।",
            "কপি করা যাচ্ছে না। ডিভাইস আনলক করে বর্তমান স্থানান্তর পরীক্ষা করুন।",
            "কপি ব্যর্থ হয়েছে। ক্লিপবোর্ড লেখাটি গ্রহণ করেনি।",
            "ক্লিপবোর্ডে কপি হয়েছে",
        ],
        "th" => &[
            "ข้อความที่ได้รับ",
            "ลิงก์ที่ได้รับ · ข้อความธรรมดา",
            "คัดลอก",
            "ปิด",
            "ยังไม่ได้คัดลอกอะไร กด «คัดลอก» เพื่อแทนที่เนื้อหาในคลิปบอร์ด",
            "คัดลอกไม่ได้ ปลดล็อกอุปกรณ์และตรวจสอบการถ่ายโอนปัจจุบัน",
            "คัดลอกไม่สำเร็จ คลิปบอร์ดไม่ยอมรับข้อความ",
            "คัดลอกไปยังคลิปบอร์ดแล้ว",
        ],
        "id" => &[
            "Teks diterima",
            "Tautan diterima · teks biasa",
            "Salin",
            "Tutup",
            "Belum ada yang disalin. Gunakan “Salin” untuk mengganti isi papan klip.",
            "Tidak dapat menyalin. Buka kunci perangkat dan periksa transfer saat ini.",
            "Gagal menyalin. Papan klip tidak menerima teks.",
            "Disalin ke papan klip",
        ],
        "fil" => &[
            "Natanggap na text",
            "Natanggap na link · plain text",
            "Kopyahin",
            "Isara",
            "Wala pang nakopya. Gamitin ang “Kopyahin” upang palitan ang laman ng clipboard.",
            "Hindi makakopya. I-unlock ang device at suriin ang kasalukuyang paglilipat.",
            "Hindi nakopya. Hindi tinanggap ng clipboard ang text.",
            "Nakopya sa clipboard",
        ],
        "ar" => &[
            "النص المستلَم",
            "الرابط المستلَم · نص عادي",
            "نسخ",
            "إغلاق",
            "لم يُنسخ أي شيء. استخدم «نسخ» لاستبدال محتوى الحافظة.",
            "النسخ غير متاح. افتح قفل الجهاز وتحقق من عملية النقل الحالية.",
            "فشل النسخ. لم تقبل الحافظة النص.",
            "تم النسخ إلى الحافظة",
        ],
        _ => &ENGLISH_REVIEW,
    }
}

fn translated(locale: &str, source: &'static str) -> &'static str {
    if let Some(index) = REVIEW_KEYS.iter().position(|key| *key == source) {
        return translated_review(locale)[index];
    }
    let Some(index) = MENU_KEYS.iter().position(|key| *key == source) else {
        return source;
    };
    let row: &[&str; 17] = match locale {
        "ru" => &[
            "Виден",
            "Скрыт",
            "Получить файлы",
            "Отправить файлы…",
            "История",
            "Настройки…",
            "Выйти из приложения",
            "Видимость для других устройств",
            "Получить",
            "Отправить",
            "Устройства",
            "Скрыть",
            "Рядом",
            "Входящие",
            "Добавить файлы…",
            "Папка…",
            "Отмена",
        ],
        "es" => &[
            "Visible",
            "Oculto",
            "Recibir archivos",
            "Enviar archivos…",
            "Historial",
            "Configuración…",
            "Salir de la aplicación",
            "Visibilidad para dispositivos cercanos",
            "Recibir",
            "Enviar",
            "Dispositivos",
            "Ocultar",
            "Cercanos",
            "Entrantes",
            "Añadir archivos…",
            "Carpeta…",
            "Cancelar",
        ],
        "pt" => &[
            "Visível",
            "Oculto",
            "Receber ficheiros",
            "Enviar ficheiros…",
            "Histórico",
            "Definições…",
            "Sair da aplicação",
            "Visibilidade para dispositivos próximos",
            "Receber",
            "Enviar",
            "Dispositivos",
            "Ocultar",
            "Perto de si",
            "A receber",
            "Adicionar ficheiros…",
            "Pasta…",
            "Cancelar",
        ],
        "de" => &[
            "Sichtbar",
            "Verborgen",
            "Dateien empfangen",
            "Dateien senden…",
            "Verlauf",
            "Einstellungen…",
            "Anwendung beenden",
            "Sichtbarkeit für Geräte in der Nähe",
            "Empfangen",
            "Senden",
            "Geräte",
            "Ausblenden",
            "In der Nähe",
            "Eingehend",
            "Dateien hinzufügen…",
            "Ordner…",
            "Abbrechen",
        ],
        "fr" => &[
            "Visible",
            "Masqué",
            "Recevoir des fichiers",
            "Envoyer des fichiers…",
            "Historique",
            "Paramètres…",
            "Quitter l’application",
            "Visibilité pour les appareils à proximité",
            "Recevoir",
            "Envoyer",
            "Appareils",
            "Masquer",
            "À proximité",
            "Entrants",
            "Ajouter des fichiers…",
            "Dossier…",
            "Annuler",
        ],
        "it" => &[
            "Visibile",
            "Nascosto",
            "Ricevi file",
            "Invia file…",
            "Cronologia",
            "Impostazioni…",
            "Esci dall’applicazione",
            "Visibilità per i dispositivi vicini",
            "Ricevi",
            "Invia",
            "Dispositivi",
            "Nascondi",
            "Nelle vicinanze",
            "In arrivo",
            "Aggiungi file…",
            "Cartella…",
            "Annulla",
        ],
        "pl" => &[
            "Widoczny",
            "Ukryty",
            "Odbierz pliki",
            "Wyślij pliki…",
            "Historia",
            "Ustawienia…",
            "Zakończ aplikację",
            "Widoczność dla urządzeń w pobliżu",
            "Odbierz",
            "Wyślij",
            "Urządzenia",
            "Ukryj",
            "W pobliżu",
            "Przychodzące",
            "Dodaj pliki…",
            "Folder…",
            "Anuluj",
        ],
        "hi" => &[
            "दृश्यमान",
            "छिपा हुआ",
            "फ़ाइलें प्राप्त करें",
            "फ़ाइलें भेजें…",
            "इतिहास",
            "सेटिंग…",
            "ऐप बंद करें",
            "आस-पास के डिवाइस के लिए दृश्यता",
            "प्राप्त करें",
            "भेजें",
            "डिवाइस",
            "छिपाएँ",
            "आस-पास",
            "आने वाले",
            "फ़ाइलें जोड़ें…",
            "फ़ोल्डर…",
            "रद्द करें",
        ],
        "zh-Hant" => &[
            "可見",
            "隱藏",
            "接收檔案",
            "傳送檔案…",
            "記錄",
            "設定…",
            "結束應用程式",
            "附近裝置的可見性",
            "接收",
            "傳送",
            "裝置",
            "隱藏",
            "附近",
            "傳入",
            "新增檔案…",
            "資料夾…",
            "取消",
        ],
        "ja" => &[
            "表示中",
            "非表示",
            "ファイルを受信",
            "ファイルを送信…",
            "履歴",
            "設定…",
            "アプリを終了",
            "付近のデバイスへの公開設定",
            "受信",
            "送信",
            "デバイス",
            "隠す",
            "近くのデバイス",
            "受信中",
            "ファイルを追加…",
            "フォルダー…",
            "キャンセル",
        ],
        "nl" => &[
            "Zichtbaar",
            "Verborgen",
            "Bestanden ontvangen",
            "Bestanden verzenden…",
            "Geschiedenis",
            "Instellingen…",
            "App afsluiten",
            "Zichtbaarheid voor apparaten in de buurt",
            "Ontvangen",
            "Verzenden",
            "Apparaten",
            "Verbergen",
            "In de buurt",
            "Binnenkomend",
            "Bestanden toevoegen…",
            "Map…",
            "Annuleren",
        ],
        "sv" => &[
            "Synlig",
            "Dold",
            "Ta emot filer",
            "Skicka filer…",
            "Historik",
            "Inställningar…",
            "Avsluta appen",
            "Synlighet för enheter i närheten",
            "Ta emot",
            "Skicka",
            "Enheter",
            "Dölj",
            "I närheten",
            "Inkommande",
            "Lägg till filer…",
            "Mapp…",
            "Avbryt",
        ],
        "tr" => &[
            "Görünür",
            "Gizli",
            "Dosyaları al",
            "Dosyaları gönder…",
            "Geçmiş",
            "Ayarlar…",
            "Uygulamadan çık",
            "Yakındaki cihazlara görünürlük",
            "Al",
            "Gönder",
            "Cihazlar",
            "Gizle",
            "Yakındakiler",
            "Gelenler",
            "Dosya ekle…",
            "Klasör…",
            "İptal",
        ],
        "ko" => &[
            "표시",
            "숨김",
            "파일 받기",
            "파일 보내기…",
            "기록",
            "설정…",
            "앱 종료",
            "주변 기기에 표시",
            "받기",
            "보내기",
            "기기",
            "숨기기",
            "주변",
            "받는 중",
            "파일 추가…",
            "폴더…",
            "취소",
        ],
        "bn" => &[
            "দৃশ্যমান",
            "লুকানো",
            "ফাইল গ্রহণ করুন",
            "ফাইল পাঠান…",
            "ইতিহাস",
            "সেটিংস…",
            "অ্যাপ বন্ধ করুন",
            "কাছাকাছি ডিভাইসের কাছে দৃশ্যমানতা",
            "গ্রহণ করুন",
            "পাঠান",
            "ডিভাইস",
            "লুকান",
            "কাছাকাছি",
            "আসছে",
            "ফাইল যোগ করুন…",
            "ফোল্ডার…",
            "বাতিল",
        ],
        "th" => &[
            "มองเห็นได้",
            "ซ่อนอยู่",
            "รับไฟล์",
            "ส่งไฟล์…",
            "ประวัติ",
            "การตั้งค่า…",
            "ออกจากแอป",
            "การมองเห็นโดยอุปกรณ์ใกล้เคียง",
            "รับ",
            "ส่ง",
            "อุปกรณ์",
            "ซ่อน",
            "ใกล้เคียง",
            "ขาเข้า",
            "เพิ่มไฟล์…",
            "โฟลเดอร์…",
            "ยกเลิก",
        ],
        "id" => &[
            "Terlihat",
            "Tersembunyi",
            "Terima berkas",
            "Kirim berkas…",
            "Riwayat",
            "Pengaturan…",
            "Keluar dari aplikasi",
            "Visibilitas bagi perangkat sekitar",
            "Terima",
            "Kirim",
            "Perangkat",
            "Sembunyikan",
            "Di sekitar",
            "Masuk",
            "Tambahkan berkas…",
            "Folder…",
            "Batal",
        ],
        "fil" => &[
            "Nakikita",
            "Nakatago",
            "Tumanggap ng mga file",
            "Magpadala ng mga file…",
            "Kasaysayan",
            "Mga setting…",
            "Isara ang app",
            "Pagiging nakikita ng mga kalapit na device",
            "Tumanggap",
            "Magpadala",
            "Mga device",
            "Itago",
            "Malapit",
            "Papasok",
            "Magdagdag ng mga file…",
            "Folder…",
            "Kanselahin",
        ],
        "ar" => &[
            "مرئي",
            "مخفي",
            "استلام الملفات",
            "إرسال الملفات…",
            "السجل",
            "الإعدادات…",
            "إنهاء التطبيق",
            "إمكانية الظهور للأجهزة القريبة",
            "استلام",
            "إرسال",
            "الأجهزة",
            "إخفاء",
            "القريبة",
            "الواردة",
            "إضافة ملفات…",
            "مجلد…",
            "إلغاء",
        ],
        _ => &ENGLISH,
    };
    row[index]
}

/// Apply only static Builder labels; dynamic data and consent are not translated
/// by scanning arbitrary widgets. Uncataloged strings preserve English.
pub(crate) fn localize_panel(builder: &gtk::Builder) {
    use gtk::prelude::*;

    for (id, source) in [("files", "Add files…"), ("folder", "Folder…")] {
        if let Some(button) = builder.object::<gtk::Button>(id) {
            button.set_label(label(source));
        }
    }
    for (id, source) in [
        ("quick-receive-label", "Receive"),
        ("quick-send-label", "Send"),
        ("quick-devices-label", "Devices"),
        ("quick-hide-label", "Hide"),
    ] {
        if let Some(widget) = builder.object::<gtk::Label>(id) {
            widget.set_label(label(source));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_locale_inventory_is_unique_and_complete() {
        assert_eq!(LOCALES.len(), 20);
        for (index, locale) in LOCALES.iter().enumerate() {
            assert!(!locale.is_empty());
            assert!(!LOCALES[..index].contains(locale));
            for source in MENU_KEYS {
                assert!(!translated(locale, source).is_empty());
            }
            for source in REVIEW_KEYS {
                assert!(!translated(locale, source).is_empty());
            }
        }
    }

    #[test]
    fn unsupported_territories_and_unknown_labels_never_invent_translation() {
        assert_eq!(resolve("pt_BR.UTF-8"), None);
        assert_eq!(resolve("zh_CN.UTF-8"), None);
        assert_eq!(resolve("zh_Hans"), None);
        assert_eq!(resolve("pt_PT.UTF-8"), Some("pt"));
        assert_eq!(resolve("zh_TW.UTF-8"), Some("zh-Hant"));
        assert_eq!(resolve("zh_Hant_TW"), Some("zh-Hant"));
        assert_eq!(resolve("ru_RU.UTF-8"), Some("ru"));
        assert_eq!(resolve("xx_XX"), None);
        for locale in LOCALES {
            assert_eq!(
                translated(locale, "Uncataloged safety text"),
                "Uncataloged safety text"
            );
        }
    }

    #[test]
    fn received_text_review_catalog_preserves_copy_consent_in_every_locale() {
        assert_eq!(REVIEW_KEYS.len(), 8);
        for (index, source) in REVIEW_KEYS.iter().enumerate() {
            assert!(!REVIEW_KEYS[..index].contains(source));
            assert!(!MENU_KEYS.contains(source));
            assert_eq!(translated("en", source), *source);
            for locale in LOCALES {
                let translation = translated(locale, source);
                assert!(!translation.trim().is_empty(), "{locale}: {source}");
                assert!(!translation.contains('{'), "{locale}: {source}");
            }
        }
        for locale in LOCALES {
            assert_ne!(
                translated(locale, "Copy"),
                translated(locale, "Close"),
                "{locale}: action collision"
            );
            assert_ne!(
                translated(locale, "Copied to this clipboard"),
                translated(
                    locale,
                    "Copy failed. This clipboard did not accept the text."
                ),
                "{locale}: success/failure collision"
            );
            assert_ne!(
                translated(locale, "Copied to this clipboard"),
                translated(
                    locale,
                    "Copy unavailable. Unlock and review the current transfer."
                ),
                "{locale}: success/blocked collision"
            );
            assert_ne!(
                translated(locale, "Received text"),
                translated(locale, "Received link · plain text"),
                "{locale}: payload type collision"
            );
        }
        assert_eq!(translated("ru", "Copy"), "Копировать");
        assert_eq!(translated("pt", "Copy"), "Copiar");
        assert_eq!(translated("pt", "Close"), "Fechar");
        assert_eq!(
            translated("ru", "Copied to this clipboard"),
            "Скопировано в буфер обмена"
        );
        assert_eq!(
            translated("pt", "Copied to this clipboard"),
            "Copiado para a área de transferência"
        );
        assert_eq!(
            translated("en", "Copy failed. This clipboard did not accept the text."),
            "Copy failed. This clipboard did not accept the text."
        );
        assert_eq!(
            translated(
                "en",
                "Nothing has been copied. Use Copy to replace this clipboard."
            ),
            "Nothing has been copied. Use Copy to replace this clipboard."
        );
        assert_eq!(translated("not-a-locale", "Copy"), "Copy");
    }

    #[test]
    fn russian_and_portuguese_menu_labels_preserve_operation_meanings() {
        assert_eq!(translated("ru", "Receive files"), "Получить файлы");
        assert_eq!(translated("ru", "Send files…"), "Отправить файлы…");
        assert_eq!(translated("pt", "Receive files"), "Receber ficheiros");
        assert_eq!(translated("pt", "Send files…"), "Enviar ficheiros…");
        assert_eq!(translated("pt", "Add files…"), "Adicionar ficheiros…");
        assert_eq!(translated("ru", "Cancel"), "Отмена");
        assert_eq!(translated("ar", "Quit panel"), "إنهاء التطبيق");
        assert_eq!(translated("zh-Hant", "History"), "記錄");
    }
}
