//! Native GTK startup-locale catalog foundation. UI strings not yet cataloged
//! deliberately retain their English source text; this cannot affect consent
//! semantics, the daemon protocol, or the user's persisted settings.
//! This is the first menu slice of #17, NOT completion of 20-locale GTK coverage.

mod client_errors;
mod dynamic;
mod history;
mod image;
mod menu_extra;
mod operation_notices;
mod panel_cards;
mod panel_status;
mod reveal_notices;
mod settings;
mod settings_errors;
mod settings_residual;
mod settings_state;
mod settings_warning;
mod transfer_rows;
mod visibility;

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

const COMPOSE_KEYS: [&str; 11] = [
    "Send text",
    "Write or paste text, then prepare it and choose recipients.",
    "Clipboard is read only when you paste. Nothing is sent yet.",
    "Paste clipboard",
    "Prepare text",
    "Text must be UTF-8 without NUL and at most 128 KiB.",
    "Text captured. Review it, then prepare and choose recipients.",
    "No supported text was captured. Use 1–128 KiB of UTF-8 text without NUL.",
    "Enter 1–128 KiB of text without NUL.",
    "Preparation was not queued. Review the current source and connection.",
    "Write or paste text, then send it to the selected device.",
];
const ENGLISH_COMPOSE: [&str; 11] = COMPOSE_KEYS;

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

pub(crate) fn file_count(count: usize) -> String {
    dynamic::file_count_for(active(), count)
}

pub(crate) fn byte_count(count: u64) -> String {
    dynamic::bytes_for(count)
}

pub(crate) fn pin_code(pin: &str) -> String {
    dynamic::code_for(active(), pin)
}

pub(crate) fn isolate_untrusted(value: &str) -> String {
    dynamic::isolate_untrusted(value)
}

pub(crate) fn resolved_name(value: &str) -> String {
    dynamic::resolved_name_for(active(), value)
}

pub(crate) fn visibility_remaining(seconds: u64) -> String {
    visibility::countdown_for(active(), seconds)
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

fn translated_compose(locale: &str) -> &'static [&'static str; 11] {
    match locale {
        "ru" => &[
            "Отправить текст",
            "Введите или вставьте текст, затем подготовьте его и выберите получателей.",
            "Буфер обмена считывается только при вставке. Пока ничего не отправлено.",
            "Вставить из буфера обмена",
            "Подготовить текст",
            "Текст должен быть в UTF-8, без NUL и размером не более 128 KiB.",
            "Текст получен. Проверьте его, затем подготовьте и выберите получателей.",
            "Поддерживаемый текст не получен. Используйте текст UTF-8 размером 1–128 KiB без NUL.",
            "Введите текст размером 1–128 KiB без NUL.",
            "Подготовка не поставлена в очередь. Проверьте текущий источник и соединение.",
            "Введите или вставьте текст, затем отправьте его выбранному устройству.",
        ],
        "es" => &[
            "Enviar texto",
            "Escribe o pega texto; después prepáralo y elige los destinatarios.",
            "El portapapeles solo se lee cuando pegas. Todavía no se ha enviado nada.",
            "Pegar del portapapeles",
            "Preparar texto",
            "El texto debe estar en UTF-8, no contener NUL y ocupar como máximo 128 KiB.",
            "Texto capturado. Revísalo, después prepáralo y elige los destinatarios.",
            "No se capturó texto compatible. Usa texto UTF-8 de 1–128 KiB sin NUL.",
            "Introduce texto de 1–128 KiB sin NUL.",
            "La preparación no se puso en cola. Comprueba el origen actual y la conexión.",
            "Escribe o pega texto y envíalo al dispositivo seleccionado.",
        ],
        "pt" => &[
            "Enviar texto",
            "Escreva ou cole texto, depois prepare-o e escolha os destinatários.",
            "A área de transferência só é lida quando cola. Ainda não foi enviado nada.",
            "Colar da área de transferência",
            "Preparar texto",
            "O texto tem de estar em UTF-8, não conter NUL e ter no máximo 128 KiB.",
            "Texto obtido. Reveja-o, depois prepare-o e escolha os destinatários.",
            "Não foi obtido texto compatível. Utilize texto UTF-8 de 1–128 KiB sem NUL.",
            "Introduza texto de 1–128 KiB sem NUL.",
            "A preparação não entrou na fila. Verifique a origem atual e a ligação.",
            "Escreva ou cole texto, depois envie-o para o dispositivo selecionado.",
        ],
        "de" => &[
            "Text senden",
            "Text schreiben oder einfügen, dann vorbereiten und Empfänger auswählen.",
            "Die Zwischenablage wird nur beim Einfügen gelesen. Noch wurde nichts gesendet.",
            "Aus Zwischenablage einfügen",
            "Text vorbereiten",
            "Der Text muss UTF-8 sein, darf kein NUL enthalten und höchstens 128 KiB groß sein.",
            "Text erfasst. Prüfen Sie ihn, bereiten Sie ihn vor und wählen Sie Empfänger aus.",
            "Kein unterstützter Text erfasst. Verwenden Sie UTF-8-Text mit 1–128 KiB ohne NUL.",
            "Geben Sie Text mit 1–128 KiB ohne NUL ein.",
            "Vorbereitung wurde nicht eingereiht. Prüfen Sie die aktuelle Quelle und Verbindung.",
            "Text schreiben oder einfügen und an das ausgewählte Gerät senden.",
        ],
        "fr" => &[
            "Envoyer du texte",
            "Saisissez ou collez du texte, puis préparez-le et choisissez les destinataires.",
            "Le presse-papiers n’est lu que lors du collage. Rien n’a encore été envoyé.",
            "Coller depuis le presse-papiers",
            "Préparer le texte",
            "Le texte doit être en UTF-8, sans NUL et ne pas dépasser 128 KiB.",
            "Texte récupéré. Vérifiez-le, puis préparez-le et choisissez les destinataires.",
            "Aucun texte compatible récupéré. Utilisez du texte UTF-8 de 1–128 KiB sans NUL.",
            "Saisissez du texte de 1–128 KiB sans NUL.",
            "La préparation n’a pas été mise en file d’attente. Vérifiez la source et la connexion actuelles.",
            "Saisissez ou collez du texte, puis envoyez-le à l’appareil sélectionné.",
        ],
        "it" => &[
            "Invia testo",
            "Scrivi o incolla il testo, poi preparalo e scegli i destinatari.",
            "Gli appunti vengono letti solo quando incolli. Non è stato ancora inviato nulla.",
            "Incolla dagli appunti",
            "Prepara testo",
            "Il testo deve essere in UTF-8, senza NUL e di massimo 128 KiB.",
            "Testo acquisito. Controllalo, poi preparalo e scegli i destinatari.",
            "Nessun testo supportato acquisito. Usa testo UTF-8 da 1–128 KiB senza NUL.",
            "Inserisci testo da 1–128 KiB senza NUL.",
            "La preparazione non è stata accodata. Controlla la sorgente e la connessione attuali.",
            "Scrivi o incolla il testo, poi invialo al dispositivo selezionato.",
        ],
        "pl" => &[
            "Wyślij tekst",
            "Wpisz lub wklej tekst, a następnie przygotuj go i wybierz odbiorców.",
            "Schowek jest odczytywany tylko podczas wklejania. Nic jeszcze nie wysłano.",
            "Wklej ze schowka",
            "Przygotuj tekst",
            "Tekst musi być w UTF-8, bez NUL i mieć maksymalnie 128 KiB.",
            "Tekst pobrano. Sprawdź go, a następnie przygotuj i wybierz odbiorców.",
            "Nie pobrano obsługiwanego tekstu. Użyj tekstu UTF-8 o rozmiarze 1–128 KiB bez NUL.",
            "Wpisz tekst o rozmiarze 1–128 KiB bez NUL.",
            "Przygotowanie nie zostało dodane do kolejki. Sprawdź bieżące źródło i połączenie.",
            "Wpisz lub wklej tekst, a następnie wyślij go do wybranego urządzenia.",
        ],
        "hi" => &[
            "टेक्स्ट भेजें",
            "टेक्स्ट लिखें या पेस्ट करें, फिर उसे तैयार करें और प्राप्तकर्ता चुनें।",
            "क्लिपबोर्ड केवल पेस्ट करने पर पढ़ा जाता है। अभी कुछ भी भेजा नहीं गया है।",
            "क्लिपबोर्ड से पेस्ट करें",
            "टेक्स्ट तैयार करें",
            "टेक्स्ट UTF-8 में, NUL के बिना और अधिकतम 128 KiB का होना चाहिए।",
            "टेक्स्ट प्राप्त हुआ। जाँचें, फिर तैयार करें और प्राप्तकर्ता चुनें।",
            "समर्थित टेक्स्ट नहीं मिला। NUL के बिना 1–128 KiB का UTF-8 टेक्स्ट इस्तेमाल करें।",
            "NUL के बिना 1–128 KiB का टेक्स्ट दर्ज करें।",
            "तैयारी कतार में नहीं लगी। वर्तमान स्रोत और कनेक्शन जाँचें।",
            "टेक्स्ट लिखें या पेस्ट करें, फिर चुने गए डिवाइस पर भेजें।",
        ],
        "zh-Hant" => &[
            "傳送文字",
            "輸入或貼上文字，然後準備內容並選擇接收對象。",
            "只有貼上時才會讀取剪貼簿。目前尚未傳送任何內容。",
            "從剪貼簿貼上",
            "準備文字",
            "文字必須為 UTF-8、不得包含 NUL，且大小不可超過 128 KiB。",
            "已取得文字。請先檢查，再準備內容並選擇接收對象。",
            "未取得支援的文字。請使用不含 NUL、大小為 1–128 KiB 的 UTF-8 文字。",
            "請輸入不含 NUL、大小為 1–128 KiB 的文字。",
            "準備作業未加入佇列。請檢查目前的來源及連線。",
            "輸入或貼上文字，然後傳送至所選裝置。",
        ],
        "ja" => &[
            "テキストを送信",
            "テキストを入力または貼り付けてから、準備して送信先を選択してください。",
            "クリップボードは貼り付け時にのみ読み取られます。まだ何も送信していません。",
            "クリップボードから貼り付け",
            "テキストを準備",
            "テキストは UTF-8 形式で NUL を含まず、最大 128 KiB にしてください。",
            "テキストを取得しました。内容を確認し、準備して送信先を選択してください。",
            "対応するテキストを取得できませんでした。NUL を含まない 1–128 KiB の UTF-8 テキストを使用してください。",
            "NUL を含まない 1–128 KiB のテキストを入力してください。",
            "準備処理をキューに登録できませんでした。現在の送信元と接続を確認してください。",
            "テキストを入力または貼り付けてから、選択したデバイスに送信してください。",
        ],
        "nl" => &[
            "Tekst verzenden",
            "Typ of plak tekst, bereid deze daarna voor en kies ontvangers.",
            "Het klembord wordt alleen gelezen bij plakken. Er is nog niets verzonden.",
            "Van klembord plakken",
            "Tekst voorbereiden",
            "Tekst moet UTF-8 zijn, geen NUL bevatten en maximaal 128 KiB groot zijn.",
            "Tekst opgehaald. Controleer deze, bereid deze voor en kies ontvangers.",
            "Geen ondersteunde tekst opgehaald. Gebruik UTF-8-tekst van 1–128 KiB zonder NUL.",
            "Voer tekst van 1–128 KiB zonder NUL in.",
            "Voorbereiding is niet in de wachtrij geplaatst. Controleer de huidige bron en verbinding.",
            "Typ of plak tekst en verzend deze naar het geselecteerde apparaat.",
        ],
        "sv" => &[
            "Skicka text",
            "Skriv eller klistra in text, förbered den sedan och välj mottagare.",
            "Urklippet läses bara när du klistrar in. Ingenting har skickats ännu.",
            "Klistra in från urklipp",
            "Förbered text",
            "Texten måste vara UTF-8, sakna NUL och vara högst 128 KiB.",
            "Text hämtad. Granska den, förbered den sedan och välj mottagare.",
            "Ingen text som stöds hämtades. Använd UTF-8-text på 1–128 KiB utan NUL.",
            "Ange text på 1–128 KiB utan NUL.",
            "Förberedelsen lades inte i kön. Kontrollera aktuell källa och anslutning.",
            "Skriv eller klistra in text och skicka den till den valda enheten.",
        ],
        "tr" => &[
            "Metin gönder",
            "Metin yazın veya yapıştırın, ardından hazırlayıp alıcıları seçin.",
            "Pano yalnızca yapıştırdığınızda okunur. Henüz hiçbir şey gönderilmedi.",
            "Panodan yapıştır",
            "Metni hazırla",
            "Metin UTF-8 biçiminde, NUL içermeyen ve en fazla 128 KiB boyutunda olmalıdır.",
            "Metin alındı. Gözden geçirin, ardından hazırlayıp alıcıları seçin.",
            "Desteklenen metin alınamadı. NUL içermeyen 1–128 KiB boyutunda UTF-8 metin kullanın.",
            "NUL içermeyen 1–128 KiB boyutunda metin girin.",
            "Hazırlama kuyruğa alınmadı. Geçerli kaynağı ve bağlantıyı kontrol edin.",
            "Metin yazın veya yapıştırın, ardından seçili cihaza gönderin.",
        ],
        "ko" => &[
            "텍스트 보내기",
            "텍스트를 입력하거나 붙여넣고 준비한 다음 수신자를 선택하세요.",
            "클립보드는 붙여넣을 때만 읽습니다. 아직 아무것도 전송되지 않았습니다.",
            "클립보드에서 붙여넣기",
            "텍스트 준비",
            "텍스트는 UTF-8 형식이어야 하며 NUL을 포함하지 않고 최대 128 KiB여야 합니다.",
            "텍스트를 가져왔습니다. 확인한 다음 준비하고 수신자를 선택하세요.",
            "지원되는 텍스트를 가져오지 못했습니다. NUL이 없는 1–128 KiB의 UTF-8 텍스트를 사용하세요.",
            "NUL이 없는 1–128 KiB의 텍스트를 입력하세요.",
            "준비 작업이 대기열에 추가되지 않았습니다. 현재 소스와 연결을 확인하세요.",
            "텍스트를 입력하거나 붙여넣은 다음 선택한 기기로 보내세요.",
        ],
        "bn" => &[
            "লেখা পাঠান",
            "লেখা লিখুন বা পেস্ট করুন, তারপর প্রস্তুত করে প্রাপক নির্বাচন করুন।",
            "ক্লিপবোর্ড শুধু পেস্ট করার সময় পড়া হয়। এখনও কিছু পাঠানো হয়নি।",
            "ক্লিপবোর্ড থেকে পেস্ট করুন",
            "লেখা প্রস্তুত করুন",
            "লেখা UTF-8 হতে হবে, NUL ছাড়া এবং সর্বোচ্চ 128 KiB আকারের।",
            "লেখা নেওয়া হয়েছে। যাচাই করুন, তারপর প্রস্তুত করে প্রাপক নির্বাচন করুন।",
            "সমর্থিত লেখা পাওয়া যায়নি। NUL ছাড়া 1–128 KiB আকারের UTF-8 লেখা ব্যবহার করুন।",
            "NUL ছাড়া 1–128 KiB আকারের লেখা লিখুন।",
            "প্রস্তুতি সারিতে যোগ করা হয়নি। বর্তমান উৎস ও সংযোগ পরীক্ষা করুন।",
            "লেখা লিখুন বা পেস্ট করুন, তারপর নির্বাচিত ডিভাইসে পাঠান।",
        ],
        "th" => &[
            "ส่งข้อความ",
            "พิมพ์หรือวางข้อความ จากนั้นเตรียมและเลือกผู้รับ",
            "ระบบจะอ่านคลิปบอร์ดเฉพาะเมื่อคุณวางข้อความเท่านั้น ยังไม่มีการส่งข้อมูล",
            "วางจากคลิปบอร์ด",
            "เตรียมข้อความ",
            "ข้อความต้องเป็น UTF-8 ไม่มี NUL และมีขนาดไม่เกิน 128 KiB",
            "รับข้อความมาแล้ว ตรวจสอบ จากนั้นเตรียมและเลือกผู้รับ",
            "ไม่ได้รับข้อความที่รองรับ ใช้ข้อความ UTF-8 ขนาด 1–128 KiB ที่ไม่มี NUL",
            "ป้อนข้อความขนาด 1–128 KiB ที่ไม่มี NUL",
            "ไม่ได้เพิ่มการเตรียมลงในคิว โปรดตรวจสอบแหล่งข้อมูลและการเชื่อมต่อปัจจุบัน",
            "พิมพ์หรือวางข้อความ แล้วส่งไปยังอุปกรณ์ที่เลือก",
        ],
        "id" => &[
            "Kirim teks",
            "Ketik atau tempel teks, lalu siapkan dan pilih penerima.",
            "Papan klip hanya dibaca saat Anda menempel. Belum ada yang dikirim.",
            "Tempel dari papan klip",
            "Siapkan teks",
            "Teks harus berformat UTF-8, tanpa NUL, dan berukuran maksimal 128 KiB.",
            "Teks berhasil diambil. Tinjau, lalu siapkan dan pilih penerima.",
            "Tidak ada teks yang didukung yang diambil. Gunakan teks UTF-8 berukuran 1–128 KiB tanpa NUL.",
            "Masukkan teks berukuran 1–128 KiB tanpa NUL.",
            "Persiapan tidak masuk antrean. Periksa sumber dan koneksi saat ini.",
            "Ketik atau tempel teks, lalu kirim ke perangkat yang dipilih.",
        ],
        "fil" => &[
            "Magpadala ng text",
            "Mag-type o mag-paste ng text, pagkatapos ay ihanda ito at piliin ang mga tatanggap.",
            "Binabasa lang ang clipboard kapag nagpa-paste ka. Wala pang naipapadala.",
            "I-paste mula sa clipboard",
            "Ihanda ang text",
            "Ang text ay dapat UTF-8, walang NUL, at hindi hihigit sa 128 KiB.",
            "Nakuha ang text. Suriin ito, pagkatapos ay ihanda at piliin ang mga tatanggap.",
            "Walang nakuhang sinusuportahang text. Gumamit ng UTF-8 text na 1–128 KiB at walang NUL.",
            "Maglagay ng text na 1–128 KiB at walang NUL.",
            "Hindi nailagay sa pila ang paghahanda. Suriin ang kasalukuyang pinagmulan at koneksyon.",
            "Mag-type o mag-paste ng text, pagkatapos ay ipadala sa napiling device.",
        ],
        "ar" => &[
            "إرسال نص",
            "اكتب النص أو الصقه، ثم جهّزه واختر المستلمين.",
            "لا تُقرأ الحافظة إلا عند اللصق. لم يُرسل أي شيء بعد.",
            "لصق من الحافظة",
            "تجهيز النص",
            "يجب أن يكون النص بتنسيق UTF-8 ومن دون NUL وألا يتجاوز 128 KiB.",
            "تم التقاط النص. راجعه ثم جهّزه واختر المستلمين.",
            "لم يتم التقاط نص مدعوم. استخدم نص UTF-8 بحجم 1–128 KiB من دون NUL.",
            "أدخل نصًا بحجم 1–128 KiB من دون NUL.",
            "لم تُضف عملية التجهيز إلى قائمة الانتظار. تحقق من المصدر والاتصال الحاليين.",
            "اكتب النص أو الصقه، ثم أرسله إلى الجهاز المحدد.",
        ],
        _ => &ENGLISH_COMPOSE,
    }
}

fn translated(locale: &str, source: &'static str) -> &'static str {
    if let Some(value) = history::translated(locale, source) {
        return value;
    }
    if let Some(value) = image::translated(locale, source) {
        return value;
    }
    if let Some(value) = reveal_notices::translated(locale, source) {
        return value;
    }
    if let Some(value) = settings::translated(locale, source) {
        return value;
    }
    if let Some(value) = settings_state::translated(locale, source) {
        return value;
    }
    if let Some(value) = settings_errors::translated(locale, source) {
        return value;
    }
    if let Some(value) = settings_warning::translated(locale, source) {
        return value;
    }
    if let Some(value) = settings_residual::translated(locale, source) {
        return value;
    }
    if let Some(value) = transfer_rows::translated(locale, source) {
        return value;
    }
    if let Some(value) = visibility::translated(locale, source) {
        return value;
    }
    if let Some(value) = menu_extra::translated(locale, source) {
        return value;
    }
    if let Some(value) = operation_notices::translated(locale, source) {
        return value;
    }
    if let Some(value) = panel_status::translated(locale, source) {
        return value;
    }
    if let Some(value) = client_errors::translated(locale, source) {
        return value;
    }
    if let Some(value) = panel_cards::translated(locale, source) {
        return value;
    }
    if let Some(index) = COMPOSE_KEYS.iter().position(|key| *key == source) {
        return translated_compose(locale)[index];
    }
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
            for source in history::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
            }
            for source in image::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
            }
            for source in settings::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
            }
            for source in settings_state::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
            }
            for source in settings_errors::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
            }
            for source in settings_warning::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
            }
            for source in settings_residual::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
            }
            for source in transfer_rows::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
            }
            for source in reveal_notices::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
                assert!(!transfer_rows::KEYS.contains(&source));
            }
            for source in visibility::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
                assert!(!transfer_rows::KEYS.contains(&source));
                assert!(!reveal_notices::KEYS.contains(&source));
            }
            for source in menu_extra::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
                assert!(!transfer_rows::KEYS.contains(&source));
                assert!(!reveal_notices::KEYS.contains(&source));
                assert!(!visibility::KEYS.contains(&source));
            }
            for source in operation_notices::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
                assert!(!transfer_rows::KEYS.contains(&source));
                assert!(!reveal_notices::KEYS.contains(&source));
                assert!(!visibility::KEYS.contains(&source));
                assert!(!menu_extra::KEYS.contains(&source));
            }
            for source in panel_status::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
                assert!(!transfer_rows::KEYS.contains(&source));
                assert!(!reveal_notices::KEYS.contains(&source));
                assert!(!visibility::KEYS.contains(&source));
                assert!(!menu_extra::KEYS.contains(&source));
                assert!(!operation_notices::KEYS.contains(&source));
            }
            for source in client_errors::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
                assert!(!transfer_rows::KEYS.contains(&source));
                assert!(!reveal_notices::KEYS.contains(&source));
                assert!(!visibility::KEYS.contains(&source));
                assert!(!menu_extra::KEYS.contains(&source));
                assert!(!operation_notices::KEYS.contains(&source));
                assert!(!panel_status::KEYS.contains(&source));
            }
            for source in panel_cards::KEYS {
                assert!(!translated(locale, source).is_empty());
                assert!(!MENU_KEYS.contains(&source));
                assert!(!REVIEW_KEYS.contains(&source));
                assert!(!COMPOSE_KEYS.contains(&source));
                assert!(!history::KEYS.contains(&source));
                assert!(!image::KEYS.contains(&source));
                assert!(!settings::KEYS.contains(&source));
                assert!(!settings_state::KEYS.contains(&source));
                assert!(!settings_errors::KEYS.contains(&source));
                assert!(!settings_warning::KEYS.contains(&source));
                assert!(!settings_residual::KEYS.contains(&source));
                assert!(!transfer_rows::KEYS.contains(&source));
                assert!(!reveal_notices::KEYS.contains(&source));
                assert!(!visibility::KEYS.contains(&source));
                assert!(!menu_extra::KEYS.contains(&source));
                assert!(!operation_notices::KEYS.contains(&source));
                assert!(!panel_status::KEYS.contains(&source));
                assert!(!client_errors::KEYS.contains(&source));
            }
            assert_ne!(
                settings_residual::translated(locale, "Start sharing"),
                settings::translated(locale, "Restart sharing"),
                "{locale}: Start versus Restart"
            );
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
    fn text_composer_catalog_preserves_opt_in_unsent_and_size_contracts() {
        assert_eq!(COMPOSE_KEYS.len(), 11);
        for (index, source) in COMPOSE_KEYS.iter().enumerate() {
            assert!(!COMPOSE_KEYS[..index].contains(source));
            assert!(!MENU_KEYS.contains(source));
            assert!(!REVIEW_KEYS.contains(source));
            assert_eq!(translated("en", source), *source);
            for locale in LOCALES {
                assert!(
                    !translated(locale, source).trim().is_empty(),
                    "{locale}: {source}"
                );
            }
        }
        for locale in LOCALES {
            for source in [
                "Text must be UTF-8 without NUL and at most 128 KiB.",
                "No supported text was captured. Use 1–128 KiB of UTF-8 text without NUL.",
                "Enter 1–128 KiB of text without NUL.",
            ] {
                let message = translated(locale, source);
                assert!(message.contains("128 KiB"), "{locale}: {source}");
                assert!(message.contains("NUL"), "{locale}: {source}");
            }
            for source in [
                "Text must be UTF-8 without NUL and at most 128 KiB.",
                "No supported text was captured. Use 1–128 KiB of UTF-8 text without NUL.",
            ] {
                assert!(
                    translated(locale, source).contains("UTF-8"),
                    "{locale}: {source}"
                );
            }
            assert_ne!(
                translated(locale, "Paste clipboard"),
                translated(locale, "Prepare text"),
                "{locale}: paste/prepare collision"
            );
            assert_ne!(
                translated(
                    locale,
                    "Text captured. Review it, then prepare and choose recipients."
                ),
                translated(
                    locale,
                    "Preparation was not queued. Review the current source and connection."
                ),
                "{locale}: captured/failed collision"
            );
            assert_ne!(
                translated(
                    locale,
                    "Clipboard is read only when you paste. Nothing is sent yet."
                ),
                translated(
                    locale,
                    "Text captured. Review it, then prepare and choose recipients."
                ),
                "{locale}: unsent/captured collision"
            );
            assert_ne!(
                translated(
                    locale,
                    "Write or paste text, then prepare it and choose recipients."
                ),
                translated(
                    locale,
                    "Write or paste text, then send it to the selected device."
                ),
                "{locale}: prepare/send collision"
            );
        }
        assert_eq!(translated("ru", "Send text"), "Отправить текст");
        assert_eq!(translated("pt", "Send text"), "Enviar texto");
        assert_eq!(
            translated("pt", "Paste clipboard"),
            "Colar da área de transferência"
        );
        assert_eq!(translated("ru", "Send"), "Отправить");
        assert_eq!(translated("pt", "Cancel"), "Cancelar");
        assert_eq!(translated("no-such-locale", "Prepare text"), "Prepare text");
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
