use crate::config::Config0;
use crate::error::Result0;
use crate::http::fetch::{fetch0, Exchange0};
use crate::util::rand::rand0;

pub async fn base0(c: &reqwest::Client, cfg: &Config0, target: &str) -> Result0<Exchange0> {
    let url = format!("{}/{}", target.trim_end_matches('/'), rand0());
    fetch0(c, cfg, "baseline", &url).await
}