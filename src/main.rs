use reqwest::Result;
use std::time::Duration;
use reqwest::{Response, ClientBuilder};
use reqwest::header::{ACCEPT};
use std::env;
use dotenv::dotenv;


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    login: String,
    name: String,
    url: String,
    public_repos: u16
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let user = "jayhere1";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("URL: {}", request_url);

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response: Response  = client.get(&request_url)
            .header("User-Agent", "rust-git-app")
            .header("user", user)           
            .header("x-api-key", &env::var("GIT_TOKEN").unwrap())
            .header("Content-type",  "application/json")
            .header(ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

    println!("{}", response.status());
    if response.status().is_success() {
        println!("{} is a user!", user);
        let user: User = response.json().await.unwrap();
        println!("{:?}", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}