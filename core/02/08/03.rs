use crate::fingerprint::evaluate::Match0;

pub fn rank0(v: &mut Vec<Match0>) {
    v.sort_by(|a, b| b.score.cmp(&a.score));
}