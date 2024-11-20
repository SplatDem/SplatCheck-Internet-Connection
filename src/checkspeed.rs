use crate::options::Args;

use clap::Parser;
use reqwest::{Client, Error};
use std::time::Instant;

pub async fn measure_download_speed(url: String) -> Result<f64, Error> {
    let client = Client::new();
    let start_time = Instant::now();
    let response = client.get(url).send().await?.error_for_status()?;

    let content_length = response.content_length().unwrap_or(0);
    let duration = start_time.elapsed();

    let speed_mbps = (content_length as f64 / 1024.0 / 1024.0) / duration.as_secs_f64();
    Ok(speed_mbps.clone())
}

pub async fn measure_upload_speed(url: &str) -> Result<f64, Error> {
    let args = Args::parse();

    let client = Client::new();
    let data = vec![0u8; args.size * 1024 * 1024];

    let start_time = Instant::now();
    let _response = client
        .post(url)
        .body(data.clone())
        .send()
        .await?
        .error_for_status()?;

    let duration = start_time.elapsed();
    let speed_mbps = (data.len() as f64 / 1024.0 / 1024.0) / duration.as_secs_f64();

    Ok(speed_mbps)
}
