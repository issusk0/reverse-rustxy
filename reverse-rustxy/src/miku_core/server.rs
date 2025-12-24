//server
/*
#TODO: an starting server for rustxy to use as reverse proxy server for now will only accept HTTP/80
 */



use std::arch::x86_64::_mm512_cvt_roundepu32_ps;
use std::io::{copy};
use std::net::{TcpListener, TcpStream};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;
use std::thread;

use crate::miku_core::config::{self, Config, Proxy, load_config};

//listener and stream for proxy
pub fn server() -> TcpListener{
    let config: Config = load_config();
    let proxy: &Proxy = &config.proxy(); //-> 0x102f2 -> struct allocated
    let parts: Vec<String> = proxy.get_proxy().split(':').map(str::to_string).collect();
    let ip_addr: Ipv4Addr = parts[0].parse().expect("NO compatible ipv4");
    let port: u16 = parts[1].parse::<u16>().expect("No compatible port");
    //listener ready to bind to a socket
    let socket = SocketAddrV4::new(ip_addr,port);
    let listener: TcpListener = TcpListener::bind(socket).expect("No valid socket addr");    
    //returns the listener for matching incoming connections
    listener
}


pub fn handle_client(client_stream: TcpStream, config: Arc<Config>){

    let upstream_info = match config.upstreams().first(){
        Some(u) => u,
        None => {
                    println!("No hay upstreams configurados");
                    return;
        }
    };
    match TcpStream::connect((upstream_info.address().as_str(), upstream_info.port())) {
            Ok(upstream_stream) => {
                let mut client_read = client_stream.try_clone().expect("ERROR: cant copy.");
                let mut client_write = client_stream.try_clone().expect("ERROR: cant copy.");
                let mut upstream_read = upstream_stream.try_clone().expect("ERROR: cant copy upstream data");
                let mut upstream_write = upstream_stream;
                thread::spawn(move || {
                    let _ = copy(&mut client_read, &mut upstream_write);
                    
                });

                let _ = copy(&mut upstream_read, &mut client_write);
            }
            Err(e_) => {
                println!("cant do connections");
            }
        }

}

