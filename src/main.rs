use anyhow::{bail, Result};
use chrono::NaiveDate;
use clap::Parser;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(long, default_value=current_platform::CURRENT_PLATFORM)]
    target: String,

    #[arg(required = true, num_args = 1..)]
    features: Vec<String>,
}

async fn feature_dates(feature: String, target: String) -> Result<Vec<NaiveDate>> {
    let url =
        format!("https://rust-lang.github.io/rustup-components-history/{target}/{feature}.json");
    let res = reqwest::get(&url).await?;
    let status = res.status();
    let data: HashMap<String, Value> = if !res.status().is_success() {
        bail!("{url}: Got status {status:?}");
    } else {
        res.json().await?
    };
    let avail_dates = data
        .into_iter()
        .filter(|(_, value)| *value == serde_json::Value::Bool(true))
        .filter_map(|(key, _)| NaiveDate::parse_from_str(&key, "%Y-%m-%d").ok())
        .collect();
    Ok(avail_dates)
}

async fn latest_common_nightly(features: Vec<String>, host: String) -> Result<NaiveDate> {
    let mut handles = Vec::with_capacity(features.len());

    if features.is_empty() {
        bail!("No features provided");
    }

    for feature in &features {
        handles.push(tokio::spawn(feature_dates(feature.clone(), host.clone())));
    }

    let mut feat_dates = Vec::with_capacity(features.len());
    for handle in handles {
        feat_dates.push(handle.await??);
    }
    feat_dates[0].sort_by(|a, b| b.cmp(a)); // Latest first

    for date in &feat_dates[0] {
        if feat_dates[1..].iter().all(|dates| dates.contains(date)) {
            return Ok(*date);
        }
    }

    bail!("No common dates found")
}

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = Config::parse();
    println!(
        "{}",
        latest_common_nightly(cfg.features, cfg.target,).await?
    );
    Ok(())
}
