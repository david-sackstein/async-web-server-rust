use std::thread;
use std::time::Duration;

use async_std::task;
use tide::Request;
use tide::prelude::*;

use super::Error;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

pub(crate) fn run_blocking(port: u16) -> Result<(), Error> {
    Ok(task::block_on(start(port)).map_err(Error::Server)?)
}

async fn start(port: u16) -> tide::Result<()> {

    let mut app = tide::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen(format!("127.0.0.1:{}", port)).await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    println!("Thread before json = {:?}", thread::current().id());

    task::sleep(Duration::from_secs(5)).await;

    let Animal { name, legs } = req.body_json().await?;

    println!("Thread sfter json = {:?}", thread::current().id());
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}