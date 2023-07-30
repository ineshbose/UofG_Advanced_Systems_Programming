use std::env;
use std::io::prelude::*;
use std::io::Read;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::str;
use std::sync::mpsc;
use std::thread;

const BUFFER_SIZE: usize = 10;

fn connect_to_address(address: SocketAddr) -> Result<SocketAddr, String> {
    /*
     * Attempts to connect to a given SocketAddr, works for both IPv4 and IPv6 addresses.
     */
    match TcpStream::connect(address) {
        Ok(_) => {
            println! {"Successfully connected to {}", address.ip()};
            return Ok(address);
        }
        Err(_e) => {
            return Err(format!("Unable to connect to {}. {}", address.ip(), _e));
        }
    }
}

fn send_get_to_address(address: SocketAddr) -> Result<String, String> {
    /*
     * Connects to the SocketAddr given as param.
     */
    let mut stream = match TcpStream::connect(address) {
        Ok(_stream) => _stream,
        Err(_e) => return Err(format!("Unable to connect to {}. {}", address.ip(), _e)),
    };

    let output = format!("Host: {}\r\n", address);
    stream
        .write(b"GET / HTTP/1.1\r\n")
        .expect("Could not write message in its entirety to the stream.");
    stream
        .write(output.as_bytes())
        .expect("Could not write message in its entirety to the stream.");
    stream
        .write(b"\r\n")
        .expect("Could not write message in its entirety to the stream.");

    /*
     * Buffered reading from stream.
     * Other methods were considered, such as read_to_end - but some responses didn't have EOF char.
     */
    let mut response = String::new();
    let mut buffer = [0; BUFFER_SIZE];
    let mut done: bool = false;
    while !done {
        match stream.read(&mut buffer) {
            Ok(_n) => {
                if _n < BUFFER_SIZE {
                    done = true;
                }
                response.push_str(str::from_utf8(&buffer).unwrap_or(""));
                buffer = [0; BUFFER_SIZE];
                continue;
            }
            Err(_e) => done = true,
        }
    }
    Ok(response)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("concon: no input files");
        std::process::exit(1);
    }

    /*
     * Performs DNS lookup for domain name specified on the command line.
     * A port is required to create socket address.
     * As opposed to a dummy port to retrieve IP, port 80 used which allows for the SocketAddr to be used in later connections.
     */
    let host: &str = &String::from(&args[1]);
    println!("{}", host);
    let host_port = (host, 80);
    let socket_addresses = host_port.to_socket_addrs().unwrap();

    /*
     * Creates a single "connected client" thread
     */
    let (connected_client_tx, connected_client_rx) = mpsc::channel();
    let connected_client = thread::spawn(move || {
        let mut done: bool = false;
        while !done {
            // Listens on input channel for IP address.
            let address = connected_client_rx.recv().unwrap();
            println!("First address received: {:?}", address);
            // Attempts to send GET request to given address.
            match send_get_to_address(address) {
                Ok(_resp) => {
                    println!("{}", _resp);
                    done = true;
                    // Notifies main thread that completion successful.
                }
                // On Err, nothing. Waits for next input.
                Err(_) => (),
            }
        }
    });

    /*
     * Creates "connection attempt" thread for each IP address returned by the lookup.
     * Vector of all thread transmitters initialised.
     */
    let mut transmitters = vec![];
    for _ in 0..socket_addresses.len() {
        let (tx, rx) = mpsc::channel();
        let connected_client_tclone = mpsc::Sender::clone(&connected_client_tx);
        thread::spawn(move || {
            let mut done: bool = false;
            while !done {
                // Listens on input channel for IP address.
                let address = rx.recv().unwrap();
                // Attempts to connect to channel.
                match connect_to_address(address) {
                    // If succeeds, sends to connected client thread.
                    Ok(_addr) => {
                        connected_client_tclone.send(_addr).unwrap();
                        // Exits on send.
                        done = true;
                    }
                    Err(_) => done = true,
                }
            }
        });
        transmitters.push(tx);
    }

    /*
     * After all the threads are created, a message is sent to each thread instructing it to connect to an IP address.
     */
    let mut i = 0;
    for addr in socket_addresses {
        transmitters[i].send(addr).unwrap();
        i = i + 1;
    }

    /*
     * Waits for the connected client to exit.
     */
    connected_client.join().unwrap();
}