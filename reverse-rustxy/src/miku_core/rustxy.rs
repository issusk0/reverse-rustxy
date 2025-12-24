//reverse proxy
/*
#TODO: Start a server
#TODO: Implement Rustxy
#TODO: Rustxy must handle requests and redirect them to the servers specified in the Rustxy configuration.
#TODO: Rustxy must send responses to clients showing the IP address of the server Rustxy used as a reverse proxy.
#TODO: Rustxy must mark a server as "unreachable" if it fails health checks a total of 5 times consecutively, and therefore should not send requests to that server for the next 10 minutes.
*/
use std::{net::TcpListener, sync::Arc, thread};
use crate::miku_core::{config::{self, load_config}, server::{self, handle_client}};
pub fn rustxy(){
    let config_data = load_config();
    let config = Arc::new(config_data);
    let listener: TcpListener = server::server();
    for streams in listener.incoming(){
        let config_for_client = Arc::clone(&config);
        if let Ok(s) =streams {

                println!("New connection from: {:?}", s.peer_addr());
                thread::spawn(move || {
                    handle_client(s, config_for_client);
                });
        }
    }

}