use anyhow::{bail, Result};
use chrono::NaiveDate;
use serde_json::Value;
use std::collections::HashMap;

async fn feature_dates(feature: String, host: String) -> Result<Vec<NaiveDate>> {
    let url =
        format!("https://rust-lang.github.io/rustup-components-history/{host}/{feature}.json");
    let response = reqwest::get(&url).await?;
    let data: HashMap<String, Value> = response.json().await?;
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
    println!(
        "{}",
        latest_common_nightly(
            vec!["miri".to_owned(), "clippy".to_owned(), "rls".to_owned()],
            "x86_64-unknown-linux-gnu".to_owned()
        )
        .await?
    );
    Ok(())
}
