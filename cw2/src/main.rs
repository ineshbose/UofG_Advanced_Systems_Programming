use std::env;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let _now = Instant::now();

    if args.len() < 2 {
        return println!("Usage: concon [domain-name] ...");
    }

    for arg in args[1..].iter() {
        let host = arg.clone();
        let host_ips = match (host.clone(), 80).to_socket_addrs() {
            Ok(ip_addrs) => ip_addrs,
            Err(e) => panic!("Could not resolve socket address(es) for {}: {}", host, e),
        };

        let (connected_tx, connected_rx) = channel::<TcpStream>();
        let connected_client =
            spawn(move || {
                let mut stream = connected_rx.recv().unwrap();
                let mut buf = String::new();

                match stream.write(format!("GET / HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n").as_bytes()) {
                    Ok(_) => {
                        stream.read_to_string(&mut buf).unwrap();
                        println!("Connected to {:?}\n", stream.peer_addr().unwrap());
                        println!("{buf}");
                    },
                    Err(e) => panic!("Could not connect to {:?}: {}", stream.peer_addr(), e),
                }
            });

        let mut senders = Vec::<Sender<SocketAddr>>::new();
        for _ in 0..host_ips.len() {
            let (ip_tx, ip_rx) = channel();
            senders.push(ip_tx);

            let tx = connected_tx.clone();
            spawn(move || {
                let t_ip = ip_rx.recv().unwrap().ip();

                match TcpStream::connect((t_ip, 80)) {
                    Ok(stream) => tx.send(stream).unwrap(),
                    Err(e) => panic!("Could not send data to {:?}: {}", t_ip, e),
                }
            });
        }

        for (ip, s) in host_ips.zip(senders) {
            s.send(ip).unwrap();
        }

        // match connected_client.join() {
        //     Ok(_) => println!("Finished in {:.2?} with {} threads", now.elapsed(), todo!()),
        //     Err(e) => panic!("Could not close the connected client thread: {:?}", e),
        // };
        connected_client.join().unwrap();
    }
}
