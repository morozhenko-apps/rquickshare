//! Native GTK menu and About presentation labels. Never localizes action IDs or URLs.

pub(super) const KEYS: [&str; 6] = [
    "Hide card",
    "Files…",
    "Text or link…",
    "Project website",
    "About",
    "Quit",
];

pub(super) fn translated(locale: &str, source: &str) -> Option<&'static str> {
    let index = KEYS.iter().position(|key| *key == source)?;
    let row: &[&str; 6] = match locale {
        "ru" => &[
            "Скрыть карточку",
            "Файлы…",
            "Текст или ссылка…",
            "Сайт проекта",
            "О программе",
            "Выход",
        ],
        "es" => &[
            "Ocultar tarjeta",
            "Archivos…",
            "Texto o enlace…",
            "Sitio web del proyecto",
            "Acerca de",
            "Salir",
        ],
        "pt" => &[
            "Ocultar cartão",
            "Ficheiros…",
            "Texto ou ligação…",
            "Site do projeto",
            "Sobre",
            "Sair",
        ],
        "de" => &[
            "Karte ausblenden",
            "Dateien…",
            "Text oder Link…",
            "Projektwebsite",
            "Über",
            "Beenden",
        ],
        "fr" => &[
            "Masquer la carte",
            "Fichiers…",
            "Texte ou lien…",
            "Site du projet",
            "À propos",
            "Quitter",
        ],
        "it" => &[
            "Nascondi scheda",
            "File…",
            "Testo o link…",
            "Sito del progetto",
            "Informazioni",
            "Esci",
        ],
        "pl" => &[
            "Ukryj kartę",
            "Pliki…",
            "Tekst lub link…",
            "Witryna projektu",
            "O programie",
            "Wyjdź",
        ],
        "hi" => &[
            "कार्ड छिपाएँ",
            "फ़ाइलें…",
            "टेक्स्ट या लिंक…",
            "प्रोजेक्ट वेबसाइट",
            "परिचय",
            "बाहर निकलें",
        ],
        "zh-Hant" => &[
            "隱藏卡片",
            "檔案…",
            "文字或連結…",
            "專案網站",
            "關於",
            "退出",
        ],
        "ja" => &[
            "カードを非表示",
            "ファイル…",
            "テキストまたはリンク…",
            "プロジェクトのウェブサイト",
            "このアプリについて",
            "終了",
        ],
        "nl" => &[
            "Kaart verbergen",
            "Bestanden…",
            "Tekst of link…",
            "Projectwebsite",
            "Over",
            "Afsluiten",
        ],
        "sv" => &[
            "Dölj kort",
            "Filer…",
            "Text eller länk…",
            "Projektwebbplats",
            "Om",
            "Avsluta",
        ],
        "tr" => &[
            "Kartı gizle",
            "Dosyalar…",
            "Metin veya bağlantı…",
            "Proje web sitesi",
            "Hakkında",
            "Çıkış",
        ],
        "ko" => &[
            "카드 숨기기",
            "파일…",
            "텍스트 또는 링크…",
            "프로젝트 웹사이트",
            "정보",
            "종료",
        ],
        "bn" => &[
            "কার্ড লুকান",
            "ফাইল…",
            "লেখা বা লিংক…",
            "প্রকল্পের ওয়েবসাইট",
            "সম্পর্কে",
            "প্রস্থান",
        ],
        "th" => &[
            "ซ่อนการ์ด",
            "ไฟล์…",
            "ข้อความหรือลิงก์…",
            "เว็บไซต์โครงการ",
            "เกี่ยวกับ",
            "ออก",
        ],
        "id" => &[
            "Sembunyikan kartu",
            "File…",
            "Teks atau tautan…",
            "Situs web proyek",
            "Tentang",
            "Keluar",
        ],
        "fil" => &[
            "Itago ang card",
            "Mga file…",
            "Text o link…",
            "Website ng proyekto",
            "Tungkol",
            "Lumabas",
        ],
        "ar" => &[
            "إخفاء البطاقة",
            "الملفات…",
            "نص أو رابط…",
            "موقع المشروع",
            "حول",
            "خروج",
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
    fn given_twenty_locales_when_rendering_menu_then_labels_stay_distinct() {
        assert_eq!(KEYS.len(), 6);
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
        assert_eq!(translated("ru", "About"), Some("О программе"));
        assert_eq!(translated("ru", "Quit"), Some("Выход"));
        assert_eq!(translated("pt", "About"), Some("Sobre"));
        assert_eq!(translated("en", "Quit"), Some("Quit"));
        for locale in LOCALES {
            assert_ne!(translated(locale, "About"), translated(locale, "Quit"));
        }
        assert_eq!(translated("pt-BR", KEYS[0]), Some(KEYS[0]));
        assert_eq!(translated("zh-Hans", KEYS[3]), Some(KEYS[3]));
        assert_eq!(translated("no", "panel.hide"), None);
    }
}
