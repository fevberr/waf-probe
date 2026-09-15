use reqwest::redirect::Policy;

use crate::config::Config0;
use crate::error::Result0;

pub fn build0(cfg: &Config0) -> Result0<reqwest::Client> {
    let c = reqwest::Client::builder()
        .user_agent(cfg.user_agent.clone())
        .redirect(Policy::none())
        .danger_accept_invalid_certs(true)
        .timeout(cfg.timeout)
        .connect_timeout(cfg.connect_timeout)
        .pool_max_idle_per_host(super::limits::MAX_IDLE_PER_HOST)
        .build()?;
    Ok(c)
}