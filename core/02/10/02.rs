use chrono::Utc;
use comfy_table::{presets::UTF8_FULL, Table};
use owo_colors::OwoColorize;

use crate::fingerprint::evaluate::Match0;
use crate::probe::run::ProbeResult0;
use crate::report::model::Report0;

pub fn scan0(target: &str, p: &ProbeResult0, m: &[Match0]) -> Report0 {
    Report0 {
        version: 1,
        target: target.to_string(),
        scanned_at: Utc::now().to_rfc3339(),
        tool_version: env!("CARGO_PKG_VERSION").to_string(),
        baseline: p.baseline.clone(),
        matches: m.to_vec(),
        bypass: None,
        errors: p.errors.clone(),
    }
}

pub fn print0(r: &Report0) {
    if r.matches.is_empty() {
        println!("{}", "no signatures matched".yellow());
        return;
    }
    let mut t = Table::new();
    t.load_preset(UTF8_FULL);
    t.set_header(vec!["#", "vendor", "score", "evidence"]);
    for (i, m) in r.matches.iter().enumerate() {
        let name = if i == 0 {
            m.vendor.green().bold().to_string()
        } else {
            m.vendor.clone()
        };
        t.add_row(vec![
            format!("#{}", i + 1),
            name,
            m.score.to_string(),
            m.evidence.join("\n"),
        ]);
    }
    println!("{t}");
    let top = &r.matches[0];
    let conf = (top.score * 10).min(100);
    println!(
        "\nmost likely: {} (confidence ~{}%)",
        top.vendor.green().bold(),
        conf
    );
}