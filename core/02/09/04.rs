use futures::stream::{self, StreamExt};
use serde::Serialize;

use crate::bypass::classify::blocked0;
use crate::bypass::encoders::ENCODERS0;
use crate::bypass::markers::MARKERS0;
use crate::config::Config0;
use crate::error::Result0;

#[derive(Debug, Clone, Serialize)]
pub struct BypassResult0 {
    pub marker: String,
    pub encoding: String,
    pub payload: String,
    pub status: u16,
    pub blocked: bool,
    pub note: String,
}

fn with_param0(url: &str, param: &str, value: &str) -> Result0<String> {
    let mut u = url::Url::parse(url)?;
    let pairs: Vec<(String, String)> = u
        .query_pairs()
        .filter(|(k, _)| k != param)
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    u.query_pairs_mut().clear();
    for (k, v) in pairs {
        u.query_pairs_mut().append_pair(&k, &v);
    }
    u.query_pairs_mut().append_pair(param, value);
    Ok(u.to_string())
}

pub async fn run0(url: &str, param: &str, cfg: &Config0) -> Result0<Vec<BypassResult0>> {
    let client = crate::http::client::build0(cfg)?;
    let baseline_url = with_param0(url, param, "wafprobe-baseline")?;
    let baseline_status = match client.get(&baseline_url).send().await {
        Ok(r) => r.status().as_u16(),
        Err(_) => 0,
    };

    let mut jobs: Vec<(String, String, String)> = Vec::new();
    for m in MARKERS0 {
        for e in ENCODERS0 {
            let payload = (e.func)(m.value);
            jobs.push((m.name.to_string(), e.name.to_string(), payload));
        }
    }

    let conc = cfg.concurrency.max(1);
    let mut stream = stream::iter(jobs.into_iter().map(|(marker, encoding, payload)| {
        let c = client.clone();
        let url = url.to_string();
        let param = param.to_string();
        async move {
            let u = match with_param0(&url, &param, &payload) {
                Ok(u) => u,
                Err(e) => {
                    return BypassResult0 {
                        marker,
                        encoding,
                        payload,
                        status: 0,
                        blocked: true,
                        note: format!("url: {e}"),
                    }
                }
            };
            match c.get(&u).send().await {
                Ok(r) => {
                    let status = r.status().as_u16();
                    let body = r.text().await.unwrap_or_default();
                    let (blocked, note) = blocked0(status, &body, baseline_status);
                    BypassResult0 {
                        marker,
                        encoding,
                        payload,
                        status,
                        blocked,
                        note,
                    }
                }
                Err(e) => BypassResult0 {
                    marker,
                    encoding,
                    payload,
                    status: 0,
                    blocked: true,
                    note: format!("req: {e}"),
                },
            }
        }
    }))
    .buffer_unordered(conc);

    let mut out = Vec::new();
    while let Some(r) = stream.next().await {
        out.push(r);
    }
    Ok(out)
}