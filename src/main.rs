use reqwest::{self, Error};

// reqwest - https://blog.logrocket.com/making-http-requests-rust-reqwest/
// tokio main - https://en.wikipedia.org/wiki/Tokio_(software)
// tokio tutorial - https://tokio.rs/tokio/tutorial


#[tokio::main]
async fn main() -> Result<(), Error>{

    println!("Hello, world!");

    let resp = reqwest::get("https://www.rust-lang.org").await?;

    println!("url: {}", resp.url());
    println!("status: {}", resp.status());
    println!("{}", resp.text().await?);
    Ok(())
}
