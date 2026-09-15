use regex::Regex;

pub struct Header0 {
    pub name: String,
    pub re: Regex,
}

pub struct Cookie0 {
    pub name_re: Regex,
    pub value_re: Regex,
}

pub struct Signature0 {
    pub name: String,
    pub weight: i32,
    pub headers: Vec<Header0>,
    pub cookies: Vec<Cookie0>,
    pub body: Option<Regex>,
}

pub struct Db0 {
    pub signatures: Vec<Signature0>,
}