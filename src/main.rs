use clap::Parser;
use reqwest::{Client, Error};

use std::time::Instant;

#[derive(Parser, Debug)]
#[command(long_about = None)]
struct Args {
    /// Number of tests
    #[arg(short, long)]
    tests: Option<u8>,

    /// Enable/Disable download test
    #[arg(short, long)]
    download: bool,

    /// Size of data to upload
    #[arg(short, long, default_value_t = 10)]
    size: usize,
}

#[tokio::main]
async fn main() {
    let download_url = "https://www.speedtest.net/";
    let upload_url = "https://httpbin.org/post";

    let args = Args::parse();
    let mut num_tests = 5;

    match args.tests {
        Some(tests) => num_tests = tests,
        None => print!(""),
    }

    if args.download {
        let mut total_download_speed = 0.0;

        for i in 1..=num_tests {
            let speed = measure_download_speed(download_url).await;

            match speed {
                Ok(s) => {
                    total_download_speed += s;
                    println!("Test {}: Download speed: {:.2} MB/s", i, s);
                    if s > 0.0 {
                        println!("----                   ----------");
                    }
                }
                Err(e) => {
                    eprintln!("Error in measuring load: {}", e);
                }
            }
        }

        let average_download_speed = total_download_speed / num_tests as f64;
        println!("Avg download speed: {:.2} Mb/s", average_download_speed);
        println!();
    }

    let mut total_upload_speed = 0.0;

    println!("Uploading {} MB of data", args.size);
    for i in 1..=num_tests {
        let speed = measure_upload_speed(upload_url).await;

        match speed {
            Ok(s) => {
                total_upload_speed += s;
                println!("Test {}: Upload speed: {:.2} MB/s", i, s);
                if s < 2.0 {
                    println!("------               ----------");
                }
            }
            Err(e) => {
                eprintln!("error in measuring upload {}", e);
            }
        }
    }

    let average_upload_speed = total_upload_speed / num_tests as f64;
    println!("Avg upload speed: {:.2} MB/s", average_upload_speed);

    match () {
        _ if average_upload_speed <= 1.0 => {
            println!("\nHolly Shit, Bro, Your Connection is Shitful")
        }
        _ if average_upload_speed <= 2.0 && average_upload_speed > 1.0 => {
            println!("\nNot Bad, But Can be Better")
        }
        _ if average_upload_speed > 2.0 && average_upload_speed <= 3.0 => println!("\nIt Will Do"),
        _ if average_upload_speed > 3.0 => println!("\nGood"),
        () => todo!(),
    }
}

async fn measure_download_speed(url: &str) -> Result<f64, Error> {
    let client = Client::new();
    let start_time = Instant::now();
    let response = client.get(url).send().await?.error_for_status()?;

    let content_length = response.content_length().unwrap_or(0);
    let duration = start_time.elapsed();

    let speed_mbps = (content_length as f64 / 1024.0 / 1024.0) / duration.as_secs_f64();
    Ok(speed_mbps.clone())
}

async fn measure_upload_speed(url: &str) -> Result<f64, Error> {
    let args = Args::parse();

    let client = Client::new();
    let data = vec![0u8; args.size * 1024 * 1024]; // 10 МБ данных для загрузки

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
