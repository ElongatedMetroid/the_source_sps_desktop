use fantoccini::{wd::Capabilities, Client, ClientBuilder, Locator};
use serde_json::json;
use table_extract::Table;

use crate::{config::Config, class::Class};

const LOGIN_URL: &str = "https://ps.seattleschools.org/public/home.html";

pub struct GradesClient {
    client: Client,
    config: Config,

    classes: Vec<Class>,
}

impl GradesClient {
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

        let mut grades_client = Self { client, config, classes: Vec::new() };
        grades_client.refresh_grades().await?;

        Ok(grades_client)
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

        self.classes = self.parse_grades().await?;
        // parse html and put into structure
        // Period + Teacher Name -> S1(Q1 & Q2) & S2(Q3 & Q4) grades
        // Class -> 
        // |- Vec<Grade> // Represents Q1, Q2, S1, Q3, Q4, S2 grades (for a single class)
        //    |- Vec<Assignment> // Represents assignments
        // |- Attendance // Usize of amount of absenses(? maybe holds the day you were absent)

        Ok(())
    }

    pub async fn parse_grades(&self) -> Result<Vec<Class>, Box<dyn std::error::Error>> {
        let html = self.client.source().await?;
        let table = Table::find_by_id(&html, "tblgrades").unwrap();

        for row in &table {
            println!("{}", row.get("Course").unwrap_or("Exp"));
        }

        todo!()
    }
}
