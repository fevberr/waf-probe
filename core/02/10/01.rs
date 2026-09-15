use serde::Serialize;

use crate::bypass::run::BypassResult0;
use crate::fingerprint::evaluate::Match0;
use crate::http::fetch::Exchange0;

#[derive(Debug, Serialize)]
pub struct Report0 {
    pub version: u32,
    pub target: String,
    pub scanned_at: String,
    pub tool_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub baseline: Option<Exchange0>,
    pub matches: Vec<Match0>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass: Option<Vec<BypassResult0>>,
    pub errors: Vec<String>,
}