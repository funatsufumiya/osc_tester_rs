use std::env;
use clap::{arg, command, ArgMatches, Command};
use std::net::UdpSocket;
use chrono::prelude::*;
use rosc::{OscPacket, OscType};
use std::net::ToSocketAddrs;

fn main() {
    let cmd = command!()
        .about("OSC Tester")
        .subcommand(
            Command::new("receive")
            .about("OSC Receiver (Server)")
            .arg(
                arg!(-i --ip <IP> "IP address to listen to")
                .default_value("0.0.0.0")
            )
            .arg(
                arg!(-p --port <PORT> "Port number to listen to")
                .value_parser(clap::value_parser!(u16).range(0..65535))
                .default_value("5005")
            )
            // prefer ipv6 resolution
            .arg(
                arg!(-v6 --ipv6 "Prefer IPv6")
                .default_value("false")
            )
        )
        .subcommand(
            Command::new("send")
            .about("OSC Sender (Client)")
            .arg(
                arg!(-i --ip <IP> "IP address to send to")
                .default_value("127.0.0.1")
            )
            .arg(
                arg!(-p --port <PORT> "Port number to send to")
                .value_parser(clap::value_parser!(u16).range(0..65535))
                .default_value("5005")
            )
            // prefer ipv6 resolution
            .arg(
                arg!(-v6 --ipv6 "Prefer IPv6")
                .default_value("false")
            )
            .arg(
                arg!(addr: <ADDR> "OSC address")
                .required(true)
            )
            .arg(
                arg!(args: [ARGS] "OSC arguments")
                .num_args(1..)
            )
        )
        .subcommand(
            Command::new("sample")
            .about("Sample sender")
            .arg(
                arg!(-i --ip <IP> "IP address to send to")
                .default_value("127.0.0.1")
            )
            .arg(
                arg!(-p --port <PORT> "Port number to send to")
                .value_parser(clap::value_parser!(u16).range(0..65535))
                .default_value("5005")
            )
            // prefer ipv6 resolution
            .arg(
                arg!(-v6 --ipv6 "Prefer IPv6")
                .default_value("false")
            )
            .arg(
                arg!(addr: <ADDR> "OSC address")
                .default_value("/filter")
                .required(false)
            )
        );

    let matches = cmd.get_matches();

    if let Some(matches) = matches.subcommand_matches("receive") {
        osc_server(matches);
    } else if let Some(matches) = matches.subcommand_matches("send") {
        osc_sender(matches);
    } else if let Some(matches) = matches.subcommand_matches("sample") {
        osc_sample(matches);
    }
}

fn osc_sample(matches: &ArgMatches) {
    let ip = matches.get_one::<String>("ip").unwrap();
    let port = matches.get_one::<u16>("port").unwrap();
    let addr = matches.get_one::<String>("addr").unwrap();
    let prefer_ipv6 = matches.get_one::<bool>("ipv6").unwrap();

    // if addr seems not to be IP address but hostname, resolve it
    let _ip = match format!("{}:80", ip).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(_addr) = addrs.next() {
                // check if it's ipv4, if not, use next
                if *prefer_ipv6 && _addr.ip().is_ipv4() {
                    addrs.next().unwrap().ip().to_string()
                } else if !*prefer_ipv6 && _addr.ip().is_ipv6() {
                    addrs.next().unwrap().ip().to_string()
                } else {
                    _addr.ip().to_string()
                }
            } else {
                ip.to_string()
            }
        }
        Err(_) => ip.to_string(),
    };

    println!("Sending to {}:{}... (Ctrl+C to quit)", _ip, port);

    // every 1sec
    loop {
        let val = rand::random::<f32>();
        let client = UdpSocket::bind(format!("0.0.0.0:0")).expect("Failed to bind to socket");
        let packet = rosc::OscPacket::Message(rosc::OscMessage {
            addr: addr.to_string(),
            args: vec![OscType::Float(val)],
        });
        let buf = rosc::encoder::encode(&packet).unwrap();
        client.send_to(&buf, format!("{}:{}", _ip, port)).expect("Failed to send packet");
        println!("[{}] {} {}", Local::now().format("%Y-%m-%d %H:%M:%S%.6f"), addr, val);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}


fn osc_sender(matches: &ArgMatches) {
    let ip = matches.get_one::<String>("ip").unwrap();
    let port = matches.get_one::<u16>("port").unwrap();
    let addr = matches.get_one::<String>("addr").unwrap();
    let args = matches.get_many::<String>("args").map(|vals| vals.collect::<Vec<_>>()).unwrap_or_default();
    let prefer_ipv6 = matches.get_one::<bool>("ipv6").unwrap();

    let client = UdpSocket::bind(format!("0.0.0.0:0")).expect("Failed to bind to socket");
    
    let osc_args: Vec<OscType> = args.iter().map(|arg| {
        let s = arg.to_string();
        if let Ok(int) = arg.parse::<i32>() {
            OscType::Int(int)
        } else if let Ok(float) = arg.parse::<f32>() {
            OscType::Float(float)
        } else if let Ok(double) = arg.parse::<f64>() {
            OscType::Double(double)
        } else if s == "true" || s == "True" || s == "t" || s == "T" {
            OscType::Int(1)
        } else if s == "false" || s == "False" || s == "f" || s == "F" {
            OscType::Int(0)
        } else {
            OscType::String(arg.to_string())
        }
    }).collect();

    // if ip seems not to be IP address but hostname, resolve it
    let _ip = match format!("{}:80", ip).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(_addr) = addrs.next() {
                // check if it's ipv4, if not, use next
                if *prefer_ipv6 && _addr.ip().is_ipv4() {
                    addrs.next().unwrap().ip().to_string()
                } else if !*prefer_ipv6 && _addr.ip().is_ipv6() {
                    addrs.next().unwrap().ip().to_string()
                } else {
                    _addr.ip().to_string()
                }
            } else {
                ip.to_string()
            }
        }
        Err(_) => ip.to_string(),
    };

    println!("Sending to {}:{}", _ip, port);

    let packet = rosc::OscPacket::Message(rosc::OscMessage {
        addr: addr.to_string(),
        args: osc_args.clone(),
    });

    let buf = rosc::encoder::encode(&packet).unwrap();
    client.send_to(&buf, format!("{}:{}", _ip, port)).expect("Failed to send packet");

    println!("[{}] {} {} (type tags: {})",
        Local::now().format("%Y-%m-%d %H:%M:%S%.6f"),
        addr,
        args.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(" "),
        get_type_tags(&osc_args.iter().collect::<Vec<_>>()));
}

fn osc_server(matches: &ArgMatches) {
    let ip = matches.get_one::<String>("ip").unwrap();
    let port = matches.get_one::<u16>("port").unwrap();

    // if addr seems not to be IP address but hostname, resolve it
    let _ip = match format!("{}:80", ip).to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(_addr) = addrs.next() {
                _addr.ip().to_string()
            } else {
                ip.to_string()
            }
        }
        Err(_) => ip.to_string(),
    };

    println!("Listening on {}:{}...", _ip, port);

    let socket = UdpSocket::bind(format!("{}:{}", _ip, port)).expect("Failed to bind to socket");
    
    let mut buf = [0u8; rosc::decoder::MTU];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _addr)) => {
                // println!("Received packet with size {} from: {}", size, addr);
                let packet = rosc::decoder::decode_udp(&buf[..size]).unwrap();
                handle_packet(packet.1);
            }
            Err(e) => {
                println!("Error receiving from socket: {}", e);
                break;
            }
        }
    }
}

fn get_type_string(osc_type: &OscType) -> String {
    match osc_type {
        OscType::Int(_) => "i".to_string(),
        OscType::Float(_) => "f".to_string(),
        OscType::Double(_) => "f".to_string(),
        OscType::String(_) => "s".to_string(),
        OscType::Blob(_) => "b".to_string(),
        OscType::Bool(_) => "i".to_string(),
        default => panic!("Unsupported type: {:?}", default),
    }
}

fn get_string(osc_type: &OscType) -> String {
    match osc_type {
        OscType::Int(i) => i.to_string(),
        OscType::Float(f) => f.to_string(),
        OscType::Double(d) => d.to_string(),
        OscType::String(s) => s.to_string(),
        OscType::Blob(b) => format!("{:?}", b),
        OscType::Bool(b) => b.to_string(),
        default => panic!("Unsupported type: {:?}", default),
    }
}


fn get_type_tags(args: &Vec<&OscType>) -> String {
    args.iter().map(|arg| get_type_string(arg)).collect::<Vec<String>>().join("")
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            let time_str = Local::now().format("%Y-%m-%d %H:%M:%S%.6f").to_string();
            if msg.args.len() == 0 {
                println!("[{}] {}", time_str, msg.addr);
                return;
            }
            println!("[{}] {} {} (type tags: {})", time_str, msg.addr,
                &msg.args.iter().map(|arg| get_string(arg)).collect::<Vec<String>>().join(" "),
                get_type_tags(&msg.args.iter().collect::<Vec<_>>()));
        }
        OscPacket::Bundle(bundle) => {
            println!("Received a bundle: {:?}", bundle);
        }
    }
}
