use anyhow::Result;
use chrono::NaiveDate;
use serde_json::Value;
use std::collections::HashMap;

async fn feature_dates(feature: &str, host: &str) -> Result<Vec<NaiveDate>> {
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

#[tokio::main]
async fn main() -> Result<()> {
    println!(
        "{:?}",
        feature_dates("miri", "x86_64-unknown-linux-gnu").await
    );
    Ok(())
}
