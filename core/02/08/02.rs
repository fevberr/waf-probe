pub fn push0(v: &mut Vec<String>, s: String) {
    if v.len() < 8 {
        v.push(s);
    }
}

pub fn trunc0(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}...", &s[..n])
    }
}