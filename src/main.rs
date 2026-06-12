use reqwest::{self, Error};

#[tokio::main]
async fn main() {
    println!("Hello in the program\n");
    let _ = sioniste()
    .await;
}

async fn sioniste() -> Result<String, Error> {
    let body = reqwest::get("https://www.rust-lang.org")
    .await?
    .text()
    .await?;
    println!("body = {body:?}");
    return Ok(body)
}