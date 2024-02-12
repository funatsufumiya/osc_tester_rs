// # OSC Tester (Rust)

// This is a simple tool to test the OSC communication between a client and a server.

// ## Build

// ```bash
// $ cargo build --release
// ```

// ## Usage

// ### OSC Receiver

// ```bash
// $ osc-tester server
// # Listening on 127.0.0.1:5005...
// # [2024-02-12 10:37:42.448582] /hoge 1 2 hoge (type tags: iis)
// # [2024-02-12 10:38:41.971990] /hoge 1 2 hoge (type tags: iis)
// # [2024-02-12 10:39:00.811072] /hoge 1 2 hoge (type tags: iis)
// # [2024-02-12 10:39:05.522840] /hoge 1 2.0 hoge (type tags: ifs)
// ```

// ### OSC Sender

// ```bash
// $ osc-tester send /hoge 1 2.0 hoge
// # Sending to 127.0.0.1:5005
// # [2024-02-12 10:39:05.522620] /hoge 1 2.0 hoge (type tags: ifs)
// ```

// ### Sample sender

// ```bash
// $ osc-tester sample
// # Sending to 127.0.0.1:5005... (Ctrl+C to quit)
// # [2024-02-12 10:45:16.000462] /filter 0.6610950773002804
// # [2024-02-12 10:45:17.002817] /filter 0.8154223208829204
// # [2024-02-12 10:45:18.004950] /filter 0.37209750414016063
// # [2024-02-12 10:45:19.010492] /filter 0.46979363082349024
// ```

use std::env;
// use argparse_rs::{ArgParser, ArgType};
use clap::{Command, arg};
use std::net::UdpSocket;
use chrono::prelude::*;
use rosc::{OscPacket, OscType};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || (args[1] != "server" && args[1] != "send" && args[1] != "sample") {
        println!("Usage: osc-tester [server|send|sample]");
        return;
    }

    if args[1] == "server" {
        // osc_server(&args[1..]);
        osc_server(&[&["osc-tester server".to_owned()], &args[2..]].concat());
    } else if args[1] == "send" {
        // osc_sender(&args[2..]);
        panic!("Not implemented yet");
    } else if args[1] == "sample" {
        // osc_sample_sender(&args[2..]);
        panic!("Not implemented yet");
    }
}

// fn osc_sender(args: &[String]) {
//     // show help with clap
//     let mut parser = ArgParser::new("osc-tester send".into());
//     parser.add_opt("ip", Some("127.0.0.1"), 'i', false, "IP address to send to", ArgType::Option);
//     parser.add_opt("port", Some("5005"), 'p', false, "Port number to send to", ArgType::Option);
//     parser.

fn osc_server(args: &[String]) {
    let cmd = Command::new("osc-tester server")
        .about("OSC server")
        .arg(
            arg!(-i --ip <IP> "IP address to listen to")
            .value_parser(clap::value_parser!(String))
            .default_value("127.0.0.1")
        )
        .arg(
            arg!(-p --port <PORT> "Port number to listen to")
            .value_parser(clap::value_parser!(u16).range(0..65535))
            .default_value("5005")
        );
    
    let matches = cmd.get_matches_from(args.iter());

    let ip = matches.get_one::<String>("ip").unwrap();
    let port = matches.get_one::<u16>("port").unwrap();

    println!("Listening on {}:{}...", ip, port);

    let socket = UdpSocket::bind(format!("{}:{}", ip, port)).unwrap();
    
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


fn get_type_tags(args: &[OscType]) -> String {
    args.iter().map(|arg| get_type_string(arg)).collect::<Vec<String>>().join("")
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            // let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros();
            let time_str = Local::now().format("%Y-%m-%d %H:%M:%S%.6f").to_string();
            println!("[{}] {} (type tags: {})", time_str, msg.addr, get_type_tags(&msg.args));
        }
        OscPacket::Bundle(bundle) => {
            println!("Received a bundle: {:?}", bundle);
        }
    }
}
