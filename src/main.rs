use anyhow::{bail, Result};
use chrono::NaiveDate;
use clap::Parser;
use rayon::prelude::*;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(bin_name = "cargo which-nightly", author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, default_value = current_platform::CURRENT_PLATFORM, help = "The target triple to check")]
    target: String,

    #[arg(required = true, num_args = 1.., help = "The feature(s) to find available versions for")]
    features: Vec<String>,
}

fn feature_dates(feat: &str, target: &str) -> Result<Vec<NaiveDate>> {
    let url = format!("https://rust-lang.github.io/rustup-components-history/{target}/{feat}.json");
    let res = ureq::get(&url).call()?;
    let data: HashMap<String, Value> = res.into_json()?;
    Ok(data
        .into_iter()
        .filter(|(_, value)| *value == serde_json::Value::Bool(true))
        .filter_map(|(key, _)| NaiveDate::parse_from_str(&key, "%Y-%m-%d").ok())
        .collect())
}

fn latest_common_nightly(features: &[String], host: &str) -> Result<NaiveDate> {
    if features.is_empty() {
        bail!("No features provided");
    }
    let mut feat_dates: Vec<_> = features
        .into_par_iter() // for the number of items we have, much faster than async
        .map(|f| feature_dates(f, host))
        .collect::<Result<_>>()?;
    feat_dates[0].sort_by(|a, b| b.cmp(a)); // Latest first
    for date in &feat_dates[0] {
        if feat_dates[1..].iter().all(|dates| dates.contains(date)) {
            return Ok(*date);
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
    println!("{}", latest_common_nightly(&cfg.features, &cfg.target)?);
    Ok(())
}
