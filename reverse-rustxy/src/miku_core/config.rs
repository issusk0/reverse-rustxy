use std::fs;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct Proxy{
    listen: String,
}

impl Proxy {
    pub fn get_proxy (&self) -> &String {
        &self.listen
    }

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

    pub fn proxy(&self) -> &Proxy {
        &self.proxy
    }
}

pub fn load_config() -> Config {
    // Esto lee el archivo durante la compilacion y lo mete en el binario como texto
    let content = include_str!("../../config.toml"); 


    toml::from_str(content)
        .expect("FAILED TO PARSE CONFIG EMBEDDED")
}