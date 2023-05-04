use anyhow::{bail, Context, Result};
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

fn feature_dates(feat: &str, target: &str) -> Result<Vec<String>> {
    let url = format!("https://rust-lang.github.io/rustup-components-history/{target}/{feat}.json");
    let res = ureq::get(&url).call()?;
    let data: HashMap<String, Value> = res.into_json()?;
    Ok(data
        .into_iter()
        .filter_map(|(key, value)| {
            if value == serde_json::Value::Bool(true) {
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
    println!("{}", latest_common_nightly(&cfg.features, &cfg.target)?);
    Ok(())
}
