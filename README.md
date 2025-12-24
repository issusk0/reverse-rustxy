# reverse-proxy
a reverse proxy entirely built in rust

# How to config
For now, Rustxy only works with one server. However, you can fully configure that server using the following example template:
```toml
[proxy]
listen = "0.0.0.0:8080"

[[upstreams]]
name = "rustxy"
address = "35.174.61.127"
port = 8000
```
Now, lets begin for proxy and how does it work with Rustxy. Rustxy uses that proxy to bind it to a socket, this all by default ipv4, so,mind that, the socket type will be in fact a sockaddrv4(L4 proxy), that means Rustxy doesnt care if is SSH HTTP or some database, it transfer bytes. If you want to change the port you will need to change the :8080 to the port you want, and rustxy will try to asign it to a socket, mind if, that port is being used by another socket, then Rustxy might not work.

Now for Upstreams. In reverse proxys you have this following template: ```[ Client[downstream] <-- Rustxy --> Server[upstream] ]``` as you see here, the data goes to server, so upstream will mean server(or target that rustxy is being configured to redirect the data), so if you want Rustxy to redirect all the data to a some specific server, you will need to change 2 core things: Address and Port. Name acts like a identificator for you to know which server is rustxy working with. Simple as that.

The address field is the public ipv4 of the server and the port is the port that your server recieves tcp connections, so be careful on assign it those fields.

for a better experience be sure to clone the proyect, so there wont be any issues about directory in the config.toml file and Rustxy.

# How to build
Since is a Rust program, it have a build.sh to have a binary, you have 2 options, running directly from the root file like cargo run or just execute the binary file that build.sh make for you.

Just run ./build.sh to generate the binary file if you wanna run from the binary, but, mind if everytime you change the config.toml file, you will need to re-compile again the binary.

¿Where is allocated?, the binary will be allocated in ```../target/release/reverse.rustxy``` for Linux, if you use Windows then the binary will be allocated in ```.../target/x86_64-pc-windows-gnu/release/reverse_rustxy.exe``` !!!important that i havent tested on Windows, so be careful if you find errors


