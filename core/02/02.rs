use clap::{Parser, Subcommand};

use crate::bypass;
use crate::config::config0;
use crate::error::Result0;
use crate::fingerprint;
use crate::probe;
use crate::report;
use crate::signature;

#[derive(Parser)]
#[command(name = "waf-probe", version)]
pub struct Cli0 {
    #[command(subcommand)]
    pub cmd: Cmd0,
}

#[derive(Subcommand)]
pub enum Cmd0 {
    Scan {
        target: String,
        #[arg(long, default_value_t = 10.0)]
        timeout: f64,
        #[arg(long)]
        json: bool,
    },
    TestBypass {
        url: String,
        #[arg(short, long, default_value = "q")]
        param: String,
        #[arg(long, default_value_t = 10.0)]
        timeout: f64,
        #[arg(long)]
        json: bool,
        #[arg(long = "yes-i-own-this")]
        i_own_this: bool,
    },
}

pub fn main() -> i32 {
    let cli = Cli0::parse();
    let rt = match tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("runtime: {e}");
            return 1;
        }
    };
    match rt.block_on(run0(cli)) {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

async fn run0(cli: Cli0) -> Result0<()> {
    match cli.cmd {
        Cmd0::Scan { target, timeout, json } => {
            let cfg = config0(timeout);
            let db = signature::load::load0()?;
            let p = probe::run::run0(&target, &cfg).await?;
            let m = fingerprint::evaluate::eval0(&db, &p);
            let r = report::scan::scan0(&target, &p, &m);
            if json {
                println!("{}", report::json::dump0(&r)?);
            } else {
                report::scan::print0(&r);
            }
            Ok(())
        }
        Cmd0::TestBypass { url, param, timeout, json, i_own_this } => {
            if !i_own_this {
                eprintln!("Refusing to run. Only test targets you own.");
                std::process::exit(2);
            }
            let cfg = config0(timeout);
            let b = bypass::run::run0(&url, &param, &cfg).await?;
            let r = report::bypass::bypass0(&url, b);
            if json {
                println!("{}", report::json::dump0(&r)?);
            } else {
                report::bypass::print0(&r);
            }
            Ok(())
        }
    }
}