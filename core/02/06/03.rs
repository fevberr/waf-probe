use futures::stream::{self, StreamExt};

use crate::config::Config0;
use crate::error::Result0;
use crate::http::client::build0;
use crate::http::fetch::Exchange0;
use crate::probe::baseline::base0;
use crate::probe::payloads::PROBES0;

#[derive(Debug)]
pub struct ProbeResult0 {
    pub target: String,
    pub baseline: Option<Exchange0>,
    pub responses: Vec<Exchange0>,
    pub errors: Vec<String>,
}

pub async fn run0(target: &str, cfg: &Config0) -> Result0<ProbeResult0> {
    let client = build0(cfg)?;
    let target = target.trim_end_matches('/').to_string();
    let mut out = ProbeResult0 {
        target: target.clone(),
        baseline: None,
        responses: Vec::new(),
        errors: Vec::new(),
    };

    match base0(&client, cfg, &target).await {
        Ok(ex) => out.baseline = Some(ex),
        Err(e) => out.errors.push(format!("baseline: {e}")),
    }

    let conc = cfg.concurrency.max(1);
    let items: Vec<(String, String)> = PROBES0
        .iter()
        .map(|p| (p.kind.to_string(), format!("{}{}", target, p.path)))
        .collect();

    let mut stream = stream::iter(items.into_iter().map(|(kind, url)| {
        let c = client.clone();
        let cfg = cfg.clone();
        async move {
            let r = crate::http::fetch::fetch0(&c, &cfg, &kind, &url).await;
            (kind, r)
        }
    }))
    .buffer_unordered(conc);

    while let Some((kind, r)) = stream.next().await {
        match r {
            Ok(ex) => out.responses.push(ex),
            Err(e) => out.errors.push(format!("{kind}: {e}")),
        }
    }

    Ok(out)
}