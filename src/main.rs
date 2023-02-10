use std::process::Command;

use fantoccini::{ClientBuilder, wd::Capabilities, Locator};
use serde_json::json;

const LOGIN_URL: &str = "https://ps.seattleschools.org/public/home.html";
const GRADES_URL: &str = "https://ps.seattleschools.org/guardian/home.html";

#[tokio::main]
async fn main() {
    Command::new("geckodriver").spawn().unwrap();

    get_grades().await.unwrap();
}

async fn get_grades() -> Result<(), Box<dyn std::error::Error>> {
    let arg = json!({
        "args": [
            "-headless",
        ],
    });

    let mut capabilities = Capabilities::new();
    capabilities.insert(String::from("moz:firefoxOptions"), arg);

    let client = ClientBuilder::native()
        .capabilities(capabilities)
        .connect("http://127.0.0.1:4444")
        .await?;

    client.goto(LOGIN_URL).await?;

    client
        .form(Locator::Id("LoginForm"))
        .await?
        .set(Locator::Id("fieldAccount"), "-")
        .await?
        .set(Locator::Id("fieldPassword"), "-")
        .await?
        .submit()
        .await?;

    let html = client.source().await?;

    println!("{html}");

    Ok(())
}
