use crate::error::Result0;
use crate::report::model::Report0;

pub fn dump0(r: &Report0) -> Result0<String> {
    Ok(serde_json::to_string_pretty(r)?)
}