extern crate clap;
use clap::{App, Arg, ArgMatches};
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::{Duration, Instant};
use futures::executor::block_on;

struct Port {
    port_id: u16,
    port_status: bool,
}

impl Port {
    fn new(port_id: u16, port_status: bool) -> Port {
        return Port {
            port_id,
            port_status,
        };
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

    let port_len = 65535;
    let mut ports: Vec<u16> = Vec::new();
    for i in 0..port_len {
        ports.push((i + 1) as u16);
    }

    block_on(run_scanner(address, ports));

    let time = Instant::now() - bench_start - time;
    println!("Scanning is done: {} millis", time.as_millis());
}

async fn run_scanner(address: Ipv4Addr, ports: Vec<u16>) {
    let mut port_iter = ports.iter();
    loop {
        match port_iter.next() {
            Some(port) => {
                scanner(address.clone(), port.clone()).await;
            }
            None => break,
        }
    }
}

async fn scanner(address: Ipv4Addr, port: u16) {
    let socket = SocketAddr::new(IpAddr::V4(address), port);
    if let Ok(socket) = TcpStream::connect_timeout(&socket, Duration::new(1, 0)) {
        socket.shutdown(Shutdown::Both).unwrap();
        println!("{}:{} is open", address, port);
    }
}
