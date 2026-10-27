//! Native GTK menu and About presentation labels. Never localizes action IDs or URLs.

pub(super) const KEYS: [&str; 4] = ["Hide card", "Files…", "Text or link…", "Project website"];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 4] = match locale {
        "ru" => &[
            "Скрыть карточку",
            "Файлы…",
            "Текст или ссылка…",
            "Сайт проекта",
        ],
        "es" => &[
            "Ocultar tarjeta",
            "Archivos…",
            "Texto o enlace…",
            "Sitio web del proyecto",
        ],
        "pt" => &[
            "Ocultar cartão",
            "Ficheiros…",
            "Texto ou ligação…",
            "Site do projeto",
        ],
        "de" => &[
            "Karte ausblenden",
            "Dateien…",
            "Text oder Link…",
            "Projektwebsite",
        ],
        "fr" => &[
            "Masquer la carte",
            "Fichiers…",
            "Texte ou lien…",
            "Site du projet",
        ],
        "it" => &[
            "Nascondi scheda",
            "File…",
            "Testo o link…",
            "Sito del progetto",
        ],
        "pl" => &[
            "Ukryj kartę",
            "Pliki…",
            "Tekst lub link…",
            "Witryna projektu",
        ],
        "hi" => &["कार्ड छिपाएँ", "फ़ाइलें…", "टेक्स्ट या लिंक…", "प्रोजेक्ट वेबसाइट"],
        "zh-Hant" => &["隱藏卡片", "檔案…", "文字或連結…", "專案網站"],
        "ja" => &[
            "カードを非表示",
            "ファイル…",
            "テキストまたはリンク…",
            "プロジェクトのウェブサイト",
        ],
        "nl" => &[
            "Kaart verbergen",
            "Bestanden…",
            "Tekst of link…",
            "Projectwebsite",
        ],
        "sv" => &[
            "Dölj kort",
            "Filer…",
            "Text eller länk…",
            "Projektwebbplats",
        ],
        "tr" => &[
            "Kartı gizle",
            "Dosyalar…",
            "Metin veya bağlantı…",
            "Proje web sitesi",
        ],
        "ko" => &[
            "카드 숨기기",
            "파일…",
            "텍스트 또는 링크…",
            "프로젝트 웹사이트",
        ],
        "bn" => &["কার্ড লুকান", "ফাইল…", "লেখা বা লিংক…", "প্রকল্পের ওয়েবসাইট"],
        "th" => &["ซ่อนการ์ด", "ไฟล์…", "ข้อความหรือลิงก์…", "เว็บไซต์โครงการ"],
        "id" => &[
            "Sembunyikan kartu",
            "File…",
            "Teks atau tautan…",
            "Situs web proyek",
        ],
        "fil" => &[
            "Itago ang card",
            "Mga file…",
            "Text o link…",
            "Website ng proyekto",
        ],
        "ar" => &["إخفاء البطاقة", "الملفات…", "نص أو رابط…", "موقع المشروع"],
        _ => &KEYS,
    };
    Some(row[index])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::LOCALES;

    #[test]
    fn given_twenty_locales_when_rendering_menu_then_labels_stay_distinct() {
        assert_eq!(KEYS.len(), 4);
        for (index, source) in KEYS.iter().enumerate() {
            assert!(!KEYS[..index].contains(source));
            for locale in LOCALES {
                let output = translated(locale, source).expect("known menu key");
                assert!(!output.trim().is_empty(), "{locale}: {source}");
                assert!(!output.contains('{'), "{locale}: {source}");
            }
        }
        for locale in LOCALES {
            let get = |index| translated(locale, KEYS[index]).expect("menu label");
            for (left, right) in [(0, 1), (0, 2), (1, 2), (2, 3)] {
                assert_ne!(get(left), get(right), "{locale}: {left} vs {right}");
            }
        }
        assert_eq!(translated("ru", KEYS[0]), Some("Скрыть карточку"));
        assert_eq!(translated("pt", KEYS[2]), Some("Texto ou ligação…"));
        assert_eq!(translated("ar", KEYS[3]), Some("موقع المشروع"));
        assert_eq!(translated("en", KEYS[1]), Some("Files…"));
        assert_eq!(translated("pt-BR", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("zh-Hans", KEYS[3]), Some(KEYS[3]));
        assert_eq!(translated("no", "panel.hide"), None);
    }
}
