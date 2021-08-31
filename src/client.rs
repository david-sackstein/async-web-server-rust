use std::collections::HashMap;
use reqwest::Url;
use tokio::runtime::Runtime;
use serde_json::Value;

use super::Error;

pub(crate) fn run_blocking(port: u16) -> Result<(), Error> {
    let rt  = Runtime::new().unwrap();

    Ok(rt.block_on(start(port)).map_err(Error::Client)?)
}

pub async fn start(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut map: HashMap<&str, Value> = HashMap::new();

    map.insert("name", "tortoise".into());
    map.insert("legs", 5.into() );

    println!("Sending {:#?}", map);

    let client: reqwest::Client = reqwest::Client::new();

    let mut url = Url::parse("http://localhost/orders/shoes")?;

    let _res = url.set_port(Some(port));

    let _res = client.post(url)
        .json(&map)
        .send()
        .await?;
    

    println!("Response {:#?}", _res);

    Ok(())
}