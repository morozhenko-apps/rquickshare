//! Native GTK startup-locale catalog foundation. UI strings not yet cataloged
//! deliberately retain their English source text; this cannot affect consent
//! semantics, the daemon protocol, or the user's persisted settings.
//! This is the first menu slice of #17, NOT completion of 20-locale GTK coverage.

const MENU_KEYS: [&str; 8] = ["Visible", "Hidden", "Receive files", "Send files…", "History", "Settings…", "Quit panel", "Discovery visibility"];
const ENGLISH: [&str; 8] = MENU_KEYS;

/// Canonical PersonalLanding locale IDs; never infer pt-BR or zh-Hans.
pub(crate) const LOCALES: [&str; 20] = [
    "en", "ru", "es", "pt", "de", "fr", "it", "pl", "hi", "zh-Hant", "ja",
    "nl", "sv", "tr", "ko", "bn", "th", "id", "fil", "ar",
];

fn resolve(value: &str) -> Option<&'static str> {
    let short = value.split('.').next()?.replace('_', "-").to_ascii_lowercase();
    let short = short.split('@').next().unwrap_or(&short);
    if short == "pt-br" || short.starts_with("pt-br-")
        || short == "zh-cn" || short.starts_with("zh-cn-")
        || short == "zh-hans" || short.starts_with("zh-hans-")
    {
        return None;
    }
    if short == "zh-tw" || short.starts_with("zh-tw-")
        || short == "zh-hk" || short.starts_with("zh-hk-")
        || short == "zh-hant" || short.starts_with("zh-hant-")
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

fn translated(locale: &str, source: &'static str) -> &'static str {
    let Some(index) = MENU_KEYS.iter().position(|key| *key == source) else {
        return source;
    };
    let row: &[&str; 8] = match locale {
        "ru" => &["Виден", "Скрыт", "Получить файлы", "Отправить файлы…", "История", "Настройки…", "Выйти из приложения", "Видимость для других устройств"],
        "es" => &["Visible", "Oculto", "Recibir archivos", "Enviar archivos…", "Historial", "Configuración…", "Salir de la aplicación", "Visibilidad para dispositivos cercanos"],
        "pt" => &["Visível", "Oculto", "Receber ficheiros", "Enviar ficheiros…", "Histórico", "Definições…", "Sair da aplicação", "Visibilidade para dispositivos próximos"],
        "de" => &["Sichtbar", "Verborgen", "Dateien empfangen", "Dateien senden…", "Verlauf", "Einstellungen…", "Anwendung beenden", "Sichtbarkeit für Geräte in der Nähe"],
        "fr" => &["Visible", "Masqué", "Recevoir des fichiers", "Envoyer des fichiers…", "Historique", "Paramètres…", "Quitter l’application", "Visibilité pour les appareils à proximité"],
        "it" => &["Visibile", "Nascosto", "Ricevi file", "Invia file…", "Cronologia", "Impostazioni…", "Esci dall’applicazione", "Visibilità per i dispositivi vicini"],
        "pl" => &["Widoczny", "Ukryty", "Odbierz pliki", "Wyślij pliki…", "Historia", "Ustawienia…", "Zakończ aplikację", "Widoczność dla urządzeń w pobliżu"],
        "hi" => &["दृश्यमान", "छिपा हुआ", "फ़ाइलें प्राप्त करें", "फ़ाइलें भेजें…", "इतिहास", "सेटिंग…", "ऐप बंद करें", "आस-पास के डिवाइस के लिए दृश्यता"],
        "zh-Hant" => &["可見", "隱藏", "接收檔案", "傳送檔案…", "記錄", "設定…", "結束應用程式", "附近裝置的可見性"],
        "ja" => &["表示中", "非表示", "ファイルを受信", "ファイルを送信…", "履歴", "設定…", "アプリを終了", "付近のデバイスへの公開設定"],
        "nl" => &["Zichtbaar", "Verborgen", "Bestanden ontvangen", "Bestanden verzenden…", "Geschiedenis", "Instellingen…", "App afsluiten", "Zichtbaarheid voor apparaten in de buurt"],
        "sv" => &["Synlig", "Dold", "Ta emot filer", "Skicka filer…", "Historik", "Inställningar…", "Avsluta appen", "Synlighet för enheter i närheten"],
        "tr" => &["Görünür", "Gizli", "Dosyaları al", "Dosyaları gönder…", "Geçmiş", "Ayarlar…", "Uygulamadan çık", "Yakındaki cihazlara görünürlük"],
        "ko" => &["표시", "숨김", "파일 받기", "파일 보내기…", "기록", "설정…", "앱 종료", "주변 기기에 표시"],
        "bn" => &["দৃশ্যমান", "লুকানো", "ফাইল গ্রহণ করুন", "ফাইল পাঠান…", "ইতিহাস", "সেটিংস…", "অ্যাপ বন্ধ করুন", "কাছাকাছি ডিভাইসের কাছে দৃশ্যমানতা"],
        "th" => &["มองเห็นได้", "ซ่อนอยู่", "รับไฟล์", "ส่งไฟล์…", "ประวัติ", "การตั้งค่า…", "ออกจากแอป", "การมองเห็นโดยอุปกรณ์ใกล้เคียง"],
        "id" => &["Terlihat", "Tersembunyi", "Terima berkas", "Kirim berkas…", "Riwayat", "Pengaturan…", "Keluar dari aplikasi", "Visibilitas bagi perangkat sekitar"],
        "fil" => &["Nakikita", "Nakatago", "Tumanggap ng mga file", "Magpadala ng mga file…", "Kasaysayan", "Mga setting…", "Isara ang app", "Pagiging nakikita ng mga kalapit na device"],
        "ar" => &["مرئي", "مخفي", "استلام الملفات", "إرسال الملفات…", "السجل", "الإعدادات…", "إنهاء التطبيق", "إمكانية الظهور للأجهزة القريبة"],
        _ => &ENGLISH,
    };
    row[index]
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
            assert_eq!(translated(locale, "Uncataloged safety text"), "Uncataloged safety text");
        }
    }

    #[test]
    fn russian_and_portuguese_menu_labels_preserve_operation_meanings() {
        assert_eq!(translated("ru", "Receive files"), "Получить файлы");
        assert_eq!(translated("ru", "Send files…"), "Отправить файлы…");
        assert_eq!(translated("pt", "Receive files"), "Receber ficheiros");
        assert_eq!(translated("pt", "Send files…"), "Enviar ficheiros…");
        assert_eq!(translated("ar", "Quit panel"), "إنهاء التطبيق");
        assert_eq!(translated("zh-Hant", "History"), "記錄");
    }
}
