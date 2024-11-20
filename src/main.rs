mod inet_utils;
mod scic;

use clap::Parser;

#[tokio::main]
async fn main() {
  let args = scic::Args::parse();

  if args.check {
    if inet_utils::is_connected().await {
      println!("Connection can be established!");
      return;
    } else {
      println!("Connection can't be established!");
      return;
    }
  }

  let mut download_url_string = String::new();

  match args.download {
    Some(ref url) => download_url_string = url.to_string(),
    None => {}
  }
  let download_url_len = download_url_string.len();

  let upload_url = "https://httpbin.org/post";

  let mut num_tests = 5;

  match args.tests {
    Some(tests) => num_tests = tests,
    None => print!(""),
  }

  let mut total_download_speed = 0.0;

  if download_url_len != 0 {
    for i in 1..=num_tests {
      let value = inet_utils::measure_download_speed(
        download_url_string.clone(),
      );
      let speed = value.await;

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

    let average_download_speed =
      total_download_speed / num_tests as f64;
    println!(
      "Avg download speed: {:.2} Mb/s",
      average_download_speed
    );
    println!();
  }

  let mut total_upload_speed = 0.0;

  println!("Uploading {} MB of data", args.size);
  for i in 1..=num_tests {
    let speed = inet_utils::measure_upload_speed(upload_url).await;

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
    _ if average_upload_speed <= 2.0
      && average_upload_speed > 1.0 =>
    {
      println!("\nNot Bad, But Can be Better")
    }
    _ if average_upload_speed > 2.0
      && average_upload_speed <= 3.0 =>
    {
      println!("\nIt Will Do")
    }
    _ if average_upload_speed > 3.0 => println!("\nGood"),
    () => todo!(),
  }
}
