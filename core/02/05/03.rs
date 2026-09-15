use std::collections::BTreeMap;

use futures::StreamExt;
use serde::Serialize;

use crate::config::Config0;
use crate::error::Result0;
use crate::http::limits::MAX_BODY;

#[derive(Debug, Clone, Serialize)]
pub struct Exchange0 {
    pub kind: String,
    pub url: String,
    pub status: u16,
    pub headers: BTreeMap<String, String>,
    pub cookies: BTreeMap<String, String>,
    #[serde(skip_serializing)]
    pub body: String,
}

pub async fn fetch0(
    c: &reqwest::Client,
    cfg: &Config0,
    kind: &str,
    url: &str,
) -> Result0<Exchange0> {
    let resp = c.get(url).send().await?;
    let status = resp.status().as_u16();
    let mut headers = BTreeMap::new();
    for (k, v) in resp.headers().iter() {
        if let Ok(s) = v.to_str() {
            headers.insert(k.as_str().to_ascii_lowercase(), s.to_string());
        }
    }
    let mut cookies = BTreeMap::new();
    for ck in resp.cookies() {
        cookies.insert(ck.name().to_string(), ck.value().to_string());
    }
    let cap = cfg.max_body.min(MAX_BODY);
    let mut buf: Vec<u8> = Vec::with_capacity(8192);
    let mut s = resp.bytes_stream();
    while let Some(chunk) = s.next().await {
        let chunk = chunk?;
        let rem = cap.saturating_sub(buf.len());
        if rem == 0 {
            break;
        }
        let take = chunk.len().min(rem);
        buf.extend_from_slice(&chunk[..take]);
        if take < chunk.len() {
            break;
        }
    }
    Ok(Exchange0 {
        kind: kind.to_string(),
        url: url.to_string(),
        status,
        headers,
        cookies,
        body: String::from_utf8_lossy(&buf).into_owned(),
    })
}