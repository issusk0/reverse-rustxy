//server
/*
#TODO: an starting server for rustxy to use as reverse proxy server for now will only accept HTTP/80
 */


use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::StripPrefixError;

use crate::miku_core::config::{Config, Proxy,load_config};

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


//Start the redirection of package recieved by the listener
pub fn handle_client(streams: TcpStream){
    let mut buffer= [0;512];
    let config = load_config();
    for el in config.upstreams(){
        let mut streams = TcpStream::connect((el.address().as_str(), el.port())).expect("Failed to connect");
        loop {
            match streams.read(&mut buffer){
                Ok(0) => {print!("Closed by client")}
                Ok(n) => {
                    let data = &buffer[n..];
                    streams.write_all(data);
                    streams.flush();
                }
                Err(e_) => {println!("not able to handle client")}
            }
        }

    }
}
