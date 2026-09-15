use std::collections::BTreeMap;

use serde::Serialize;

use crate::fingerprint::{evidence, rank, score};
use crate::probe::run::ProbeResult0;
use crate::signature::model::Db0;

#[derive(Debug, Clone, Serialize)]
pub struct Match0 {
    pub vendor: String,
    pub score: i32,
    pub evidence: Vec<String>,
}

pub fn eval0(db: &Db0, res: &ProbeResult0) -> Vec<Match0> {
    let mut headers: BTreeMap<String, String> = BTreeMap::new();
    let mut cookies: Vec<(String, String)> = Vec::new();
    let mut bodies: Vec<String> = Vec::new();

    if let Some(b) = &res.baseline {
        for (k, v) in &b.headers {
            headers.entry(k.clone()).or_insert_with(|| v.clone());
        }
        for (k, v) in &b.cookies {
            cookies.push((k.clone(), v.clone()));
        }
        bodies.push(b.body.clone());
    }
    for r in &res.responses {
        for (k, v) in &r.headers {
            headers.entry(k.clone()).or_insert_with(|| v.clone());
        }
        for (k, v) in &r.cookies {
            cookies.push((k.clone(), v.clone()));
        }
        bodies.push(r.body.clone());
    }

    let mut out: Vec<Match0> = Vec::new();
    for sig in &db.signatures {
        let mut sc = 0i32;
        let mut ev: Vec<String> = Vec::new();

        for h in &sig.headers {
            if let Some(v) = headers.get(&h.name) {
                if h.re.is_match(v) {
                    score::add0(&mut sc, sig.weight);
                    evidence::push0(
                        &mut ev,
                        format!("header {}: {}", h.name, evidence::trunc0(v, 80)),
                    );
                }
            }
        }
        for ck in &sig.cookies {
            for (n, v) in &cookies {
                if ck.name_re.is_match(n) && ck.value_re.is_match(v) {
                    score::add0(&mut sc, sig.weight);
                    evidence::push0(&mut ev, format!("cookie {}", n));
                    break;
                }
            }
        }
        if let Some(re) = &sig.body {
            for b in &bodies {
                if let Some(m) = re.find(b) {
                    score::add0(&mut sc, sig.weight);
                    evidence::push0(
                        &mut ev,
                        format!("body: {:?}", evidence::trunc0(m.as_str(), 80)),
                    );
                    break;
                }
            }
        }

        if sc > 0 {
            out.push(Match0 {
                vendor: sig.name.clone(),
                score: sc,
                evidence: ev,
            });
        }
    }
    rank::rank0(&mut out);
    out
}