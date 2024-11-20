pub async fn is_connected() -> bool {
    let response = reqwest::get("https://www.google.com").await;

    match response {
        Ok(res) => res.status().is_success(),
        Err(_) => false,
    }
}
