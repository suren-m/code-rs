use async_std::fs;
use std::io::Error;
use tide::prelude::*;
use tide::Request;

const CONFIG_FILE: &str = "config.txt";

async fn get_config(path: &str) -> Result<String, Error> {
    fs::read_to_string(path).await
}

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let contents = get_config("~/data/british-english").await?;
    let mut app = tide::new();

    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {} Order for {} shoes", name, legs).into())
}
