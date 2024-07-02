// use std::io::Read;

use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main()->Result<()> {
    // get request without async await
    // let mut resp=reqwest::blocking::get("http://httpbin.org/get")?;
    // let mut body=String::new();
    // resp.read_to_string(&mut body)?;

    // println!("Status code = {}",resp.status());

    // println!("Headers = \n{:?}",resp.headers());

    // println!("Body = \n{}",body);


    // get request using async await

    let resp=reqwest::get("http://httpbin.org/get").await.unwrap();

    println!("Status code = {}",resp.status());

    println!("Headers = \n{:?}",resp.headers());

    println!("Body = \n{}",resp.text().await.unwrap());

    Ok(())
}
