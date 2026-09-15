use std::collections::HashMap;

use regex::Regex;
use serde::Deserialize;

use crate::error::Result0;
use crate::signature::model::{Cookie0, Db0, Header0, Signature0};

const RAW0: &str = include_str!("../../../../signatures/00.yaml");

#[derive(Deserialize)]
struct RawDb0 {
    signatures: Vec<RawSig0>,
}

#[derive(Deserialize)]
struct RawSig0 {
    name: String,
    weight: i32,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    cookies: HashMap<String, String>,
    #[serde(default)]
    body: Option<String>,
}

pub fn load0() -> Result0<Db0> {
    let raw: RawDb0 = serde_yaml::from_str(RAW0)?;
    let mut sigs = Vec::with_capacity(raw.signatures.len());
    for r in raw.signatures {
        let mut headers = Vec::new();
        for (k, v) in r.headers {
            headers.push(Header0 {
                name: k.to_ascii_lowercase(),
                re: Regex::new(&v)?,
            });
        }
        let mut cookies = Vec::new();
        for (k, v) in r.cookies {
            cookies.push(Cookie0 {
                name_re: Regex::new(&k)?,
                value_re: Regex::new(&v)?,
            });
        }
        let body = match r.body {
            Some(p) => Some(Regex::new(&p)?),
            None => None,
        };
        sigs.push(Signature0 {
            name: r.name,
            weight: r.weight,
            headers,
            cookies,
            body,
        });
    }
    Ok(Db0 { signatures: sigs })
}