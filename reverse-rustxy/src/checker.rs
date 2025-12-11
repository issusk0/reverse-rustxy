//The checker will be responsible for checking the status of a server.

//#TODO: Create the function to estimate whether a server responds in a timely manner or not
use crate::miku_core::config;
use reqwest;

pub fn is_healthy() -> Result<(), Box<dyn std::error::Error>>  {
    let config = config::load_config();
    let upstream = &config.upstreams()[0];


    let url = format!("http://{}:{}", upstream.address(), upstream.port());
    println!("Checking url: {}", url);
    let resp = reqwest::blocking::get(&url)?;
    println!("Status: {:?}", resp.status());

    let body = resp.text()?;
    println!("Body: {}", &body);
    Ok(())
}