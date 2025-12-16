//reverse proxy
/*
#TODO: Start a server
#TODO: Implement Rustxy
#TODO: Rustxy must handle requests and redirect them to the servers specified in the Rustxy configuration.
#TODO: Rustxy must send responses to clients showing the IP address of the server Rustxy used as a reverse proxy.
#TODO: Rustxy must mark a server as "unreachable" if it fails health checks a total of 5 times consecutively, and therefore should not send requests to that server for the next 10 minutes.
*/

use crate::miku_core::checker;
use std::thread;
pub fn rustxy(){
    let checker_status = thread::spawn(|| {
        checker::main_checker();
    });

    checker_status.join().unwrap();
}