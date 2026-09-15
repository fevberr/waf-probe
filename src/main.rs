use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "waf-probe", version, about = "Fingerprints WAFs from HTTP responses")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scan a URL and report which WAF(s) appear to be in front of it
    Scan {
        url: String,

        /// Path to the signatures YAML file
        #[arg(long, default_value = "signatures/00.yaml")]
        signatures: PathBuf,

        /// Emit JSON instead of a human-readable report
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Deserialize)]
struct SignatureFile {
    #[allow(dead_code)]
    version: u32,
    signatures: Vec<Signature>,
}

#[derive(Debug, Deserialize)]
struct Signature {
    name: String,
    #[serde(default = "default_weight")]
    weight: u32,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    cookies: HashMap<String, String>,
    #[serde(default)]
    body: Option<String>,
}

fn default_weight() -> u32 {
    1
}

#[derive(Debug)]
struct Hit {
    name: String,
    weight: u32,
    evidence: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Scan { url, signatures, json } => scan(&url, &signatures, json).await,
    }
}

async fn scan(url: &str, sig_path: &PathBuf, as_json: bool) -> Result<()> {
    // 1. Load + parse signatures
    let raw = std::fs::read_to_string(sig_path)
        .with_context(|| format!("reading signatures file: {}", sig_path.display()))?;
    let sig_file: SignatureFile =
        serde_yaml::from_str(&raw).context("parsing signatures YAML")?;

    // 2. Fetch the target
    let client = reqwest::Client::builder()
        .user_agent("waf-probe/0.1")
        .build()?;
    let resp = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("requesting {url}"))?;

    let status = resp.status();
    let headers = resp.headers().clone();
    let set_cookies: Vec<String> = headers
        .get_all(reqwest::header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .collect();
    let body = resp.text().await.unwrap_or_default();

    // 3. Evaluate each signature
    let mut hits: Vec<Hit> = Vec::new();

    for sig in &sig_file.signatures {
        let mut evidence = Vec::new();

        // headers
        for (hname, pattern) in &sig.headers {
            let Ok(re) = Regex::new(pattern) else { continue };
            let Ok(hn) = reqwest::header::HeaderName::from_bytes(hname.as_bytes()) else {
                continue;
            };
            if let Some(val) = headers.get(&hn) {
                if let Ok(s) = val.to_str() {
                    if re.is_match(s) {
                        evidence.push(format!("header {hname}: {s}"));
                    }
                }
            }
        }

        // cookies (keys are themselves regexes, matched against Set-Cookie names)
        for (cname_pat, val_pat) in &sig.cookies {
            let Ok(name_re) = Regex::new(cname_pat) else { continue };
            let Ok(val_re) = Regex::new(val_pat) else { continue };
            for sc in &set_cookies {
                let name = sc.split('=').next().unwrap_or("").trim();
                let val = sc.splitn(2, '=').nth(1).unwrap_or("").trim();
                if name_re.is_match(name) && val_re.is_match(val) {
                    evidence.push(format!("cookie {name}"));
                }
            }
        }

        // body
        if let Some(bpat) = &sig.body {
            if let Ok(re) = Regex::new(bpat) {
                if re.is_match(&body) {
                    evidence.push("body match".to_string());
                }
            }
        }

        if !evidence.is_empty() {
            hits.push(Hit {
                name: sig.name.clone(),
                weight: sig.weight,
                evidence,
            });
        }
    }

    hits.sort_by(|a, b| b.weight.cmp(&a.weight));

    // 4. Report
    if as_json {
        let out = serde_json::json!({
            "url": url,
            "status": status.as_u16(),
            "hits": hits.iter().map(|h| serde_json::json!({
                "name": h.name,
                "weight": h.weight,
                "evidence": h.evidence,
            })).collect::<Vec<_>>(),
        });
        println!("{}", serde_json::to_string_pretty(&out)?);
    } else {
        println!("waf-probe scan: {url}");
        println!("HTTP status: {status}");
        if hits.is_empty() {
            println!("No known WAF signature matched.");
        } else {
            println!("\nMatched {} signature(s):\n", hits.len());
            for h in &hits {
                println!("  [HIT] {} (weight {})", h.name, h.weight);
                for e in &h.evidence {
                    println!("        - {e}");
                }
            }
        }
    }

    Ok(())
}
