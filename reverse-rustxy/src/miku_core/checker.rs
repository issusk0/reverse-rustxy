//The checker will be responsible for checking the status of a server.

//#TODO: Create the function to estimate whether a server responds in a timely manner or not
use crate::miku_core::config;
use reqwest;

pub fn is_healthy() -> Result<i32, &'static str> {
    let config = config::load_config();
    let upstream = &config.upstreams()[0];


    let url = format!("http://{}:{}", upstream.address(), upstream.port());

    let resp =  reqwest::blocking::get(&url)
        .map_err(|_| "NO VALID CODE FOR HEALTHY")?;



    let status_code = resp.status();

    if status_code != reqwest::StatusCode::OK{
        Err("Not healthy")
    }else{
         Ok(status_code.as_u16() as i32)

    }






}


pub fn show_status(status_code:i32) -> &'static str{
    match status_code {
        200 => "OK",
        _ => "Error"
    }
}


pub fn main_checker(){
    let ip_stream = config::load_config();
    let code = is_healthy().unwrap();
    let status = show_status(code);
    println!("Status: {} for: {}", status, ip_stream.upstreams()[0].address());
}