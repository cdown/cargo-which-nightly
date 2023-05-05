use anyhow::{bail, Context, Result};
use clap::Parser;
use rayon::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::process::{Command, Stdio};

#[derive(Parser, Debug)]
#[command(bin_name = "cargo which-nightly", author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, default_value = current_platform::CURRENT_PLATFORM, help = "The target triple to check")]
    target: String,

    #[arg(required = true, num_args = 1.., help = "The feature(s) to find available versions for")]
    features: Vec<String>,

    #[arg(long, help = "Set the found nightly as the default with rustup")]
    set_default: bool,
}

fn set_default(date: &str) -> Result<()> {
    let status = Command::new("rustup")
        .arg("default")
        .arg(format!("nightly-{date}"))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if !status.success() {
        bail!("Failed to set default toolchain to nightly-{}", date);
    }
    Ok(())
}

/// A naive date check. No need to depend on `chrono::NaiveDate` -- worst case we just get provided
/// something bogus.
fn is_date(date: &str) -> bool {
    date.len() == "0000-00-00".len() && date.chars().all(|c| c.is_ascii_digit() || c == '-')
}

fn feature_dates(feat: &str, target: &str) -> Result<Vec<String>> {
    let url = format!("https://rust-lang.github.io/rustup-components-history/{target}/{feat}.json");
    let res = ureq::get(&url).call()?;
    let data: HashMap<String, Value> = res.into_json()?;
    Ok(data
        .into_iter()
        .filter_map(|(key, value)| {
            if is_date(&key) && value == serde_json::Value::Bool(true) {
                Some(key)
            } else {
                None
            }
        })
        .collect())
}

fn latest_common_nightly(features: &[String], host: &str) -> Result<String> {
    let mut feat_dates: Vec<_> = features
        .into_par_iter() // for the number of items we have, much faster than async
        .map(|f| feature_dates(f, host))
        .collect::<Result<_>>()?;

    let mut tail = feat_dates.pop().context("No features provided")?;
    tail.sort_by(|a, b| b.cmp(a)); // Latest first
    for date in tail {
        if feat_dates.iter().all(|dates| dates.contains(&date)) {
            return Ok(date);
        }
    }
    bail!("No common dates found")
}

fn main() -> Result<()> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.get(1) == Some(&"which-nightly".to_string()) {
        args.remove(1);
    }
    let cfg = Config::parse_from(args);
    let date = latest_common_nightly(&cfg.features, &cfg.target)?;
    if cfg.set_default {
        set_default(&date)?;
    } else {
        println!("{date}");
    }
    Ok(())
}
