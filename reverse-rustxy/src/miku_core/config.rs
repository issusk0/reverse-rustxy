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
    let mut config_path = std::env::current_exe().expect("No se pudo obtener la ruta del ejecutable");
    config_path.pop(); // Quita el nombre del binario
    config_path.push("config.toml"); // Apunta al archivo

    let content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| {
            // Si no está junto al binario, intenta en el root (para desarrollo con cargo run)
            fs::read_to_string("config.toml").expect("CONFIG NOT FOUND IN ANY LOCATION")
        });

    toml::from_str(&content).expect("FAILED TO PARSE CONFIG")
}