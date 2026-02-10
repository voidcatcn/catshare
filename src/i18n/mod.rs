//! Internationalization module with support for 10 common languages
//! Languages: English, Chinese, Spanish, Hindi, Arabic, Portuguese, Russian, Japanese, French, German

pub mod translations;

pub use translations::{init, t, Language, TranslationKey};

use rand::Rng;

/// Adjectives for random name generation (10 adjectives)
const ADJECTIVES_EN: &[&str] = &[
    "Happy", "Swift", "Brave", "Clever", "Gentle", "Bright", "Calm", "Noble", "Wise", "Kind",
];

const ADJECTIVES_ZH: &[&str] = &[
    "快乐的",
    "敏捷的",
    "勇敢的",
    "聪明的",
    "温柔的",
    "明亮的",
    "平静的",
    "高贵的",
    "智慧的",
    "善良的",
];

const ADJECTIVES_ES: &[&str] = &[
    "Feliz",
    "Veloz",
    "Valiente",
    "Listo",
    "Gentil",
    "Brillante",
    "Calmo",
    "Noble",
    "Sabio",
    "Amable",
];

const ADJECTIVES_HI: &[&str] = &[
    "खुश",
    "तेज़",
    "बहादुर",
    "चतुर",
    "सौम्य",
    "उज्ज्वल",
    "शांत",
    "महान",
    "बुद्धिमान",
    "दयालु",
];

const ADJECTIVES_AR: &[&str] = &[
    "سعيد", "سريع", "شجاع", "ذكي", "لطيف", "مشرق", "هادئ", "نبيل", "حكيم", "كريم",
];

const ADJECTIVES_PT: &[&str] = &[
    "Feliz",
    "Veloz",
    "Valente",
    "Esperto",
    "Gentil",
    "Brilhante",
    "Calmo",
    "Nobre",
    "Sábio",
    "Bondoso",
];

const ADJECTIVES_RU: &[&str] = &[
    "Счастливый",
    "Быстрый",
    "Храбрый",
    "Умный",
    "Нежный",
    "Яркий",
    "Спокойный",
    "Благородный",
    "Мудрый",
    "Добрый",
];

const ADJECTIVES_JA: &[&str] = &[
    "幸せな",
    "素早い",
    "勇敢な",
    "賢い",
    "優しい",
    "明るい",
    "穏やかな",
    "高貴な",
    "賢明な",
    "親切な",
];

const ADJECTIVES_DE: &[&str] = &[
    "Fröhlich",
    "Schnell",
    "Mutig",
    "Klug",
    "Sanft",
    "Hell",
    "Ruhig",
    "Edel",
    "Weise",
    "Gütig",
];

const ADJECTIVES_FR: &[&str] = &[
    "Heureux", "Rapide", "Brave", "Malin", "Doux", "Brillant", "Calme", "Noble", "Sage", "Gentil",
];

/// Nouns for random name generation (10 nouns - animals)
const NOUNS_EN: &[&str] = &[
    "Cat", "Dog", "Bird", "Fox", "Bear", "Wolf", "Owl", "Deer", "Hawk", "Lion",
];

const NOUNS_ZH: &[&str] = &[
    "猫",
    "狗",
    "鸟",
    "狐狸",
    "熊",
    "狼",
    "猫头鹰",
    "鹿",
    "鹰",
    "狮子",
];

const NOUNS_ES: &[&str] = &[
    "Gato", "Perro", "Pájaro", "Zorro", "Oso", "Lobo", "Búho", "Ciervo", "Halcón", "León",
];

const NOUNS_HI: &[&str] = &[
    "बिल्ली",
    "कुत्ता",
    "पक्षी",
    "लोमड़ी",
    "भालू",
    "भेड़िया",
    "उल्लू",
    "हिरण",
    "बाज़",
    "शेर",
];

const NOUNS_AR: &[&str] = &[
    "قط", "كلب", "طائر", "ثعلب", "دب", "ذئب", "بومة", "غزال", "صقر", "أسد",
];

const NOUNS_PT: &[&str] = &[
    "Gato", "Cão", "Pássaro", "Raposa", "Urso", "Lobo", "Coruja", "Cervo", "Falcão", "Leão",
];

const NOUNS_RU: &[&str] = &[
    "Кот",
    "Пёс",
    "Птица",
    "Лиса",
    "Медведь",
    "Волк",
    "Сова",
    "Олень",
    "Ястреб",
    "Лев",
];

const NOUNS_JA: &[&str] = &[
    "猫",
    "犬",
    "鳥",
    "狐",
    "熊",
    "狼",
    "フクロウ",
    "鹿",
    "鷹",
    "獅子",
];

const NOUNS_DE: &[&str] = &[
    "Katze", "Hund", "Vogel", "Fuchs", "Bär", "Wolf", "Eule", "Hirsch", "Falke", "Löwe",
];

const NOUNS_FR: &[&str] = &[
    "Chat", "Chien", "Oiseau", "Renard", "Ours", "Loup", "Hibou", "Cerf", "Faucon", "Lion",
];

/// Get adjectives for a language
fn get_adjectives(lang: Language) -> &'static [&'static str] {
    match lang {
        Language::English => ADJECTIVES_EN,
        Language::Chinese => ADJECTIVES_ZH,
        Language::Spanish => ADJECTIVES_ES,
        Language::Hindi => ADJECTIVES_HI,
        Language::Arabic => ADJECTIVES_AR,
        Language::Portuguese => ADJECTIVES_PT,
        Language::Russian => ADJECTIVES_RU,
        Language::Japanese => ADJECTIVES_JA,
        Language::German => ADJECTIVES_DE,
        Language::French => ADJECTIVES_FR,
    }
}

/// Get nouns for a language
fn get_nouns(lang: Language) -> &'static [&'static str] {
    match lang {
        Language::English => NOUNS_EN,
        Language::Chinese => NOUNS_ZH,
        Language::Spanish => NOUNS_ES,
        Language::Hindi => NOUNS_HI,
        Language::Arabic => NOUNS_AR,
        Language::Portuguese => NOUNS_PT,
        Language::Russian => NOUNS_RU,
        Language::Japanese => NOUNS_JA,
        Language::German => NOUNS_DE,
        Language::French => NOUNS_FR,
    }
}

/// Generate a random name in the format "Adjective + Noun"
/// Uses IP last octet as seed for consistent naming
pub fn generate_name(ip_last_octet: u8, lang: Language) -> String {
    let adjectives = get_adjectives(lang);
    let nouns = get_nouns(lang);

    // Use IP octet to deterministically select words
    let adj_idx = (ip_last_octet as usize) % adjectives.len();
    let noun_idx = (ip_last_octet as usize / 10) % nouns.len();

    let adjective = adjectives[adj_idx];
    let noun = nouns[noun_idx];

    // Format based on language (some languages don't need space)
    match lang {
        Language::Chinese | Language::Japanese => format!("{}{}", adjective, noun),
        _ => format!("{} {}", adjective, noun),
    }
}

/// Generate a completely random name
pub fn generate_random_name(lang: Language) -> String {
    let adjectives = get_adjectives(lang);
    let nouns = get_nouns(lang);

    let mut rng = rand::thread_rng();
    let adj_idx = rng.gen_range(0..adjectives.len());
    let noun_idx = rng.gen_range(0..nouns.len());

    let adjective = adjectives[adj_idx];
    let noun = nouns[noun_idx];

    match lang {
        Language::Chinese | Language::Japanese => format!("{}{}", adjective, noun),
        _ => format!("{} {}", adjective, noun),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_name() {
        init();
        let name = generate_name(114, Language::English);
        assert!(!name.is_empty());
        println!("Generated name for 114: {}", name);

        let name_zh = generate_name(114, Language::Chinese);
        assert!(!name_zh.is_empty());
        println!("Generated name for 114 (Chinese): {}", name_zh);
    }

    #[test]
    fn test_name_consistency() {
        init();
        // Same IP should generate same name
        let name1 = generate_name(100, Language::English);
        let name2 = generate_name(100, Language::English);
        assert_eq!(name1, name2);
    }
}
