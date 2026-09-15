use comfy_table::{presets::UTF8_FULL, Table};
use owo_colors::OwoColorize;

use crate::bypass::run::BypassResult0;
use crate::report::model::Report0;

pub fn bypass0(url: &str, b: Vec<BypassResult0>) -> Report0 {
    Report0 {
        version: 1,
        target: url.to_string(),
        scanned_at: chrono::Utc::now().to_rfc3339(),
        tool_version: env!("CARGO_PKG_VERSION").to_string(),
        baseline: None,
        matches: Vec::new(),
        bypass: Some(b),
        errors: Vec::new(),
    }
}

pub fn print0(r: &Report0) {
    let Some(results) = &r.bypass else { return };
    let mut t = Table::new();
    t.load_preset(UTF8_FULL);
    t.set_header(vec!["marker", "encoding", "status", "result", "note"]);
    let mut passed = 0usize;
    for b in results {
        let verdict = if b.blocked {
            "blocked".red().to_string()
        } else {
            passed += 1;
            "passed".green().to_string()
        };
        t.add_row(vec![
            b.marker.clone(),
            b.encoding.clone(),
            b.status.to_string(),
            verdict,
            b.note.clone(),
        ]);
    }
    println!("{t}");
    if passed > 0 {
        println!("\n{} encodings slipped through", passed.to_string().green().bold());
    } else {
        println!("\n{} no encodings passed", "!".yellow());
    }
}