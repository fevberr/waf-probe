use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Config0 {
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub max_body: usize,
    pub concurrency: usize,
    pub user_agent: String,
}

impl Default for Config0 {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            connect_timeout: Duration::from_secs(5),
            max_body: 65536,
            concurrency: 4,
            user_agent: concat!("waf-probe/", env!("CARGO_PKG_VERSION")).to_string(),
        }
    }
}

pub fn config0(timeout_s: f64) -> Config0 {
    let mut c = Config0::default();
    c.timeout = Duration::from_secs_f64(timeout_s);
    c.connect_timeout = c.timeout.min(Duration::from_secs(5));
    c
}