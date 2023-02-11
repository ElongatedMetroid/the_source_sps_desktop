use fantoccini::{wd::Capabilities, Client, ClientBuilder, Locator};
use serde_json::json;

use crate::config::Config;

const LOGIN_URL: &str = "https://ps.seattleschools.org/public/home.html";

pub struct Grades {
    client: Client,
    config: Config,
}

impl Grades {
    pub async fn start_client(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
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

        Ok(Self { client, config })
    }
    /// TODO: this will waste memory currently since it opens a new tab
    /// everytime
    pub async fn refresh_grades(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.client.goto(LOGIN_URL).await?;

        self.client
            .form(Locator::Id("LoginForm"))
            .await?
            .set(Locator::Id("fieldAccount"), &self.config.username)
            .await?
            .set(Locator::Id("fieldPassword"), &self.config.password)
            .await?
            .submit()
            .await?;

        let html = self.client.source().await?;

        // parse html and put into table

        Ok(())
    }
}
