//reverse proxy
/*
#TODO: Start a server
#TODO: Implement Rustxy
#TODO: Rustxy must handle requests and redirect them to the servers specified in the Rustxy configuration.
#TODO: Rustxy must send responses to clients showing the IP address of the server Rustxy used as a reverse proxy.
#TODO: Rustxy must mark a server as "unreachable" if it fails health checks a total of 5 times consecutively, and therefore should not send requests to that server for the next 10 minutes.
*/
use crate::miku_core::server::{server, handle_client};
use std::{net::TcpListener, thread};
pub fn rustxy(){
    let connection = thread::spawn(||{
        let listener: TcpListener = server();
        for streams in listener.incoming() {
            match streams {
                Ok(streams) => {
                    let conn = thread::spawn(||{
                        let _state  = handle_client(streams);
                    });
                    conn.join().unwrap();
                }
                Err(_e) => {
                    println!("Error while handle client");
                }
            }
        }
    });
    connection.join().unwrap();
}