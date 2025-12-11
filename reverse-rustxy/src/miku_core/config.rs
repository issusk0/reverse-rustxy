use std::fs;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
struct Proxy{
    listen: String,
}

#[derive(Deserialize, Debug)]
pub struct Upstreams {
    name: String,
    address: String,
    port: u16,
}

impl Upstreams {
    pub fn address(&self) -> &String{
        &self.address
    }
    pub fn port(&self) -> u16{
        self.port
    }
}
#[derive(Deserialize, Debug)]
pub struct Config{
    proxy: Proxy,
    upstreams: Vec<Upstreams>,
}
impl Config {
    pub fn upstreams(&self) -> &[Upstreams]{
        &self.upstreams
    }
}

pub fn load_config() -> Config {
    let content = fs::read_to_string("config.toml")
        .expect("FAILED TO READ CONFIG");

    toml::from_str(&content)
        .expect("FAILED TO PARSE CONFIG")
}