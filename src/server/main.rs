use std::{
    net::Ipv4Addr,
    time::Duration,
    str::from_utf8,
    io::stdin
};

// TODO
// xor packets
// increment sequence
// replicate second delay between each ping for more consistant behavior

use icmp_socket::socket::IcmpSocket;
use icmp_socket::Icmpv4Message;
use icmp_socket::Icmpv4Packet;
use icmp_socket::IcmpSocket4;
use clap::Parser;

const TIMEOUT: Duration = Duration::from_secs(5);

const CLIENT_IDENTIFIER: u16 = 0;
const SERVER_IDENTIFIER: u16 = 1;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP address to bind to
    #[arg(short, long)]
    address: Option<String>
}

pub fn main() {
    let host_address = Args::parse().address.unwrap_or_else(|| {
        println!("No arguments provided. Setting default address as 127.0.0.1");

        "127.0.0.1".to_string()
    });

    let parsed_addr = host_address.parse::<Ipv4Addr>().unwrap();

    let socket = IcmpSocket4::new();
    let mut command = String::new();

    let Ok(mut socket) = socket else {
        println!("Error hooking socket: {:?}", socket.err().unwrap());
        return;
    };

    println!("Bound to {}!\nPlease type a command:", parsed_addr);

    while command.trim() != "exit" {
        command.clear();

        stdin().read_line(&mut command).unwrap();

        let packet = Icmpv4Packet {
            typ: 8,
            code: 0,
            checksum: 0,
            message: Icmpv4Message::EchoReply {
                identifier: SERVER_IDENTIFIER,
                sequence: 1,
                payload: command.as_bytes().to_vec()
            }
        };

        if let Err(err) = socket.send_to(parsed_addr, packet) {
            println!("Error sending packet: {:?}", err);
        }

        socket.set_timeout(Some(TIMEOUT));

        loop {
            let (packet, from_address) = match socket.rcv_from() {
                Ok(packet) => packet,
                Err(err) => {
                    println!("We got no response after {:?}. Most likely a timeout - {:?}", TIMEOUT, err);
                    break;
                }
            };

            let address = *from_address.as_socket_ipv4().unwrap().ip();

            if address == parsed_addr {
                if let Icmpv4Message::EchoReply { payload, identifier, .. } = packet.message {
                    if identifier != CLIENT_IDENTIFIER {
                        continue;
                    }

                    let to_utf8_from_payload = from_utf8(&payload);

                    let Ok(command_response) = to_utf8_from_payload else {
                        println!("Error parsing payload from {}: {:?}", address, to_utf8_from_payload.err().unwrap());
                        continue;
                    };

                    println!(
                        "ICMP response from client! {}\n{}",
                        address,
                        command_response
                    );

                    break;
                }
            }
        }
    }
}
