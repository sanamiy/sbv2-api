use once_cell::sync::Lazy;
use std::collections::HashMap;

static REPLACE_MAP: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("：", ",");
    map.insert("；", ",");
    map.insert("，", ",");
    map.insert("。", ".");
    map.insert("！", "!");
    map.insert("？", "?");
    map.insert("\n", ".");
    map.insert("．", ".");
    map.insert("…", "...");
    map.insert("···", "...");
    map.insert("・・・", "...");
    map.insert("·", ",");
    map.insert("・", ",");
    map.insert("、", ",");
    map.insert("$", ".");
    map.insert("“", "'");
    map.insert("”", "'");
    map.insert("\"", "'");
    map.insert("‘", "'");
    map.insert("’", "'");
    map.insert("（", "'");
    map.insert("）", "'");
    map.insert("(", "'");
    map.insert(")", "'");
    map.insert("《", "'");
    map.insert("》", "'");
    map.insert("【", "'");
    map.insert("】", "'");
    map.insert("[", "'");
    map.insert("]", "'");
    // NFKC 正規化後のハイフン・ダッシュの変種を全て通常半角ハイフン - \u002d に変換
    map.insert("\u{02d7}", "\u{002d}"); // ˗, Modifier Letter Minus Sign
    map.insert("\u{2010}", "\u{002d}"); // ‐, Hyphen,
    map.insert("\u{2012}", "\u{002d}"); // ‒, Figure Dash
    map.insert("\u{2013}", "\u{002d}"); // –, En Dash
    map.insert("\u{2014}", "\u{002d}"); // —, Em Dash
    map.insert("\u{2015}", "\u{002d}"); // ―, Horizontal Bar
    map.insert("\u{2043}", "\u{002d}"); // ⁃, Hyphen Bullet
    map.insert("\u{2212}", "\u{002d}"); // −, Minus Sign
    map.insert("\u{23af}", "\u{002d}"); // ⎯, Horizontal Line Extension
    map.insert("\u{23e4}", "\u{002d}"); // ⏤, Straightness
    map.insert("\u{2500}", "\u{002d}"); // ─, Box Drawings Light Horizontal
    map.insert("\u{2501}", "\u{002d}"); // ━, Box Drawings Heavy Horizontal
    map.insert("\u{2e3a}", "\u{002d}"); // ⸺, Two-Em Dash
    map.insert("\u{2e3b}", "\u{002d}"); // ⸻, Three-Em Dash
    map.insert("「", "'");
    map.insert("」", "'");

    map
});

/*
__PUNCTUATION_CLEANUP_PATTERN = re.compile(
    # ↓ ひらがな、カタカナ、漢字
    r"[^\u3040-\u309F\u30A0-\u30FF\u4E00-\u9FFF\u3400-\u4DBF\u3005"
    # ↓ 半角アルファベット（大文字と小文字）
    + r"\u0041-\u005A\u0061-\u007A"
    # ↓ 全角アルファベット（大文字と小文字）
    + r"\uFF21-\uFF3A\uFF41-\uFF5A"
    # ↓ ギリシャ文字
    + r"\u0370-\u03FF\u1F00-\u1FFF"
    # ↓ "!", "?", "…", ",", ".", "'", "-", 但し`…`はすでに`...`に変換されている
    + "".join(PUNCTUATIONS) + r"]+",  # fmt: skip
)
*/

pub const ZH_SYMBOLS: [&str; 65] = [
    "E", "En", "a", "ai", "an", "ang", "ao", "b", "c", "ch", "d", "e", "ei", "en", "eng", "er",
    "f", "g", "h", "i", "i0", "ia", "ian", "iang", "iao", "ie", "in", "ing", "iong", "ir", "iu",
    "j", "k", "l", "m", "n", "o", "ong", "ou", "p", "q", "r", "s", "sh", "t", "u", "ua", "uai",
    "uan", "uang", "ui", "un", "uo", "v", "van", "ve", "vn", "w", "x", "y", "z", "zh", "AA", "EE",
    "OO",
];
pub const JP_SYMBOLS: [&str; 42] = [
    "N", "a", "a:", "b", "by", "ch", "d", "dy", "e", "e:", "f", "g", "gy", "h", "hy", "i", "i:",
    "j", "k", "ky", "m", "my", "n", "ny", "o", "o:", "p", "py", "q", "r", "ry", "s", "sh", "t",
    "ts", "ty", "u", "u:", "w", "y", "z", "zy",
];

pub static PUNCTUATIONS: [&str; 7] = ["!", "?", "…", ",", ".", "'", "-"];
pub static PUNCTUATION_SYMBOLS: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut symbols = PUNCTUATIONS.to_vec();
    symbols.append(&mut vec!["SP", "UNK"]);
    symbols
});
const PAD: &str = "_";
pub static SYMBOLS: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut symbols = vec![PAD];
    symbols.append(&mut JP_SYMBOLS.to_vec());
    symbols.append(&mut PUNCTUATION_SYMBOLS.to_vec());
    symbols
});

static PUNCTUATION_CLEANUP_PATTERN: Lazy<regex::Regex> = Lazy::new(|| {
    let pattern = r"[^\u{3040}-\u{309F}\u{30A0}-\u{30FF}\u{4E00}-\u{9FFF}\u{3400}-\u{4DBF}\u{3005}"
        .to_owned()
        + r"\u{0041}-\u{005A}\u{0061}-\u{007A}"
        + r"\u{FF21}-\u{FF3A}\u{FF41}-\u{FF5A}"
        + r"\u{0370}-\u{03FF}\u{1F00}-\u{1FFF}"
        + &PUNCTUATIONS.join("")
        + r"]+";
    regex::Regex::new(&pattern).unwrap()
});

pub fn normalize_text(text: &str) -> String {
    // 日本語のテキストを正規化する
    let text = text.replace('~', "ー");
    let text = text.replace('～', "ー");

    let text = text.replace('〜', "ー");

    replace_punctuation(text)
}

pub fn replace_punctuation(mut text: String) -> String {
    for (k, v) in REPLACE_MAP.iter() {
        text = text.replace(k, v);
    }
    PUNCTUATION_CLEANUP_PATTERN
        .replace_all(&text, "")
        .to_string()
}
