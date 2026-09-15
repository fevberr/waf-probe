use std::sync::OnceLock;

use crate::error::Result0;
use crate::signature::model::Db0;

static DB0: OnceLock<Db0> = OnceLock::new();

pub fn get0() -> Result0<&'static Db0> {
    if let Some(d) = DB0.get() {
        return Ok(d);
    }
    let d = crate::signature::load::load0()?;
    let _ = DB0.set(d);
    Ok(DB0.get().unwrap())
}