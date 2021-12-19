extern crate clap;
use clap::{App, Arg, ArgMatches};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::{Duration, Instant};
use threadpool::ThreadPool;
use std::sync::{Arc, Barrier, mpsc::channel};
use std::collections::HashMap;

struct Port {
    port_id:u16,
    port_status:bool,
}

impl Port {
    fn new(port_id:u16, port_status:bool) -> Port {
        return Port {
            port_id,
            port_status,
        }
    }
}

fn get_matches<'s>() -> ArgMatches<'s> {
    return App::new("Scanner Air")
        .version("0.1.0")
        .about("This is a simple open port scanner")
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("Ip address")
                .help("The ip address of target")
                .takes_value(true)
                .empty_values(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("target port")
                .help("The port of target")
                .takes_value(true)
                .empty_values(true)
                .default_value("80"),
        )
        .get_matches();
}

fn main() {
    let bench_start = Instant::now();
    let matches = get_matches();

    let address = match matches.value_of("address") {
        Some(value) => value,
        None => "",
    };

    if address == "" {
        return;
    }
    let address = Ipv4Addr::from_str(address).unwrap();
    let time = Instant::now() - bench_start;
    println!("get address:{}", time.as_secs());

    let n_workers = 1000;
    let n_jobs = 65535;
    let pool = ThreadPool::new(100);
    let barrier = Arc::new(Barrier::new(n_jobs));

    let (sender, receiver) = channel();

    for port in 0..n_jobs {
        let barrier = barrier.clone();
        let sender = sender.clone();
        let port:u16 = (port + 1) as u16;
        pool.execute(move || {
            let socket = SocketAddr::new(IpAddr::V4(address), port);
            if let Ok(socket) = TcpStream::connect_timeout(&socket, Duration::new(1, 0)) {
                socket.shutdown(Shutdown::Both).unwrap();
                sender.send(Port::new(port, true)).unwrap();
            } else {
                sender.send(Port::new(port, false)).unwrap();
            }
            barrier.wait();
        });
    }

    let mut ports:HashMap<u16, bool> = HashMap::new();

    let mut count = 0;
    for port in receiver.into_iter() {
        count += 1;
        if port.port_status {
            dbg!(port.port_id, port.port_status);
            ports.insert(port.port_id, true);
        } else {
            dbg!(port.port_id, port.port_status);
            ports.insert(port.port_id, false);
        }
        if count == n_jobs {
            break;
        }
    }

    // println!("{:?}", ports);

    let time = Instant::now() - bench_start - time;
    println!("scanner done: {} millis", time.as_millis());
}
