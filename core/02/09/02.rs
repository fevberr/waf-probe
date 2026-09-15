use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub struct Encoder0 {
    pub name: &'static str,
    pub func: fn(&str) -> String,
}

pub const ENCODERS0: &[Encoder0] = &[
    Encoder0 { name: "raw", func: |s| s.to_string() },
    Encoder0 { name: "url", func: |s| utf8_percent_encode(s, NON_ALPHANUMERIC).to_string() },
    Encoder0 {
        name: "url-double",
        func: |s| {
            utf8_percent_encode(&utf8_percent_encode(s, NON_ALPHANUMERIC).to_string(), NON_ALPHANUMERIC)
                .to_string()
        },
    },
    Encoder0 {
        name: "html-decimal",
        func: |s| s.chars().map(|c| format!("&#{};", c as u32)).collect(),
    },
    Encoder0 {
        name: "html-hex",
        func: |s| s.chars().map(|c| format!("&#x{:x};", c as u32)).collect(),
    },
    Encoder0 {
        name: "unicode-escape",
        func: |s| s.chars().map(|c| format!("\\u{:04x}", c as u32)).collect(),
    },
    Encoder0 {
        name: "utf8-overlong",
        func: |s| s.replace('<', "%c0%bc").replace('>', "%c0%be"),
    },
    Encoder0 {
        name: "case-toggle",
        func: |s| {
            s.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c.to_ascii_lowercase()
                    } else {
                        c.to_ascii_uppercase()
                    }
                })
                .collect()
        },
    },
    Encoder0 { name: "whitespace-tab", func: |s| s.replace(' ', "\t") },
    Encoder0 { name: "comment-inline", func: |s| s.replace(' ', "/**/") },
    Encoder0 { name: "null-byte", func: |s| s.replace(' ', "%00") },
    Encoder0 { name: "plus-space", func: |s| s.replace(' ', "+") },
];