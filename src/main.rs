use reqwest::header::USER_AGENT;
// use std::io::Read;
use serde::Deserialize;

use error_chain::error_chain;

error_chain! {
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    // ! get request without async await
    // let mut resp=reqwest::blocking::get("http://httpbin.org/get")?;
    // let mut body=String::new();
    // resp.read_to_string(&mut body)?;

    // println!("Status code = {}",resp.status());

    // println!("Headers = \n{:?}",resp.headers());

    // println!("Body = \n{}",body);

    // ! get request using async await

    // let resp=reqwest::get("http://httpbin.org/get").await.unwrap();

    // println!("Status code = {}",resp.status());

    // println!("Headers = \n{:?}",resp.headers());

    // println!("Body = \n{}",resp.text().await.unwrap());

    // ! api requests

    let req_url = format!("https://api.github.com/repos/{owner}/{repos}/stargazers",
        owner="rust-lang-nursery",
        repos="rust-cookbook",
    );

    let client=reqwest::Client::new();

    let resp=client.get(&req_url).header(USER_AGENT, "rust web-api-client demo").send().await?;

    let users:Vec<User>=resp.json().await?;

    println!("{:?}",users);

    Ok(())
}
