use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use openssl::ssl::{SslConnector, SslMethod, SslStream};
use trapdoor::packet;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_stream(stream);
    }
}
fn handle_stream(mut stream: TcpStream) {
    let mut state: i32 = 0;
    read_packet(&stream, &mut state); // Handshake
    read_packet(&stream, &mut state); // Login start
}
fn read_packet(stream: &TcpStream, state: &mut i32) {
    let packet_len = packet::read_var_int(&stream);
    let packet_id = packet::read_var_int(&stream);
    // println!("Packet ID: {:?}", packet_id);
    if *state == 0 { // Handshaking
        if packet_id == 0 {
            // println!(" (Handshake)");
            *state = read_handshake(&stream);
        }
    } else if *state == 1 { // Status

    } else if *state == 2 { // Login
        if packet_id == 0 {
            let mut username = packet::read_string(&stream);
            println!("Username: {}", username);
            send_post(
                // String::from("localhost:80"),
                String::from("api.mojang.com"),
                String::from("/profiles/minecraft"),
                String::from("application/json"),
                String::from(format!("[\"{}\"]", username)
            ));
        }
    }
}
fn read_handshake(stream: &TcpStream) -> i32 {
    let protocol_version = packet::read_var_int(&stream);
    // println!("Protocol Version: {:?}", protocol_version);
    let server_address = packet::read_string(&stream);
    // println!("Server Address: {:?}", server_address);
    let server_port = packet::read_unsigned_short(&stream);
    // println!("Server Port: {:?}", server_port);
    let next_state = packet::read_var_int(&stream);
    // println!("Next State: {:?}", next_state);
    println!("Read handshake");
    next_state
}
fn send_post(server_address: String, server_path: String, content_type: String, content: String) {
    let mut stream = get_ssl_stream(&server_address);
    // let mut req = String::new();
    // req.push_str(&String::from(format!(
    //     "POST {} HTTP/1.1\n",
    //     server_path
    // )));
    // req.push_str(&String::from(format!(
    //     "Content-Type: {}\n",
    //     content_type
    // )));
    // req.push_str(&String::from(format!(
    //     "Host: {}\n",
    //     server_address
    // )));
    // req.push_str(&String::from(format!(
    //     "Content-Length: {}\n",
    //     content.len()
    // )));
    // req.push_str(&String::from("Accept: */*\nConnection: keep-alive\nCache-Control: no-cache\n"));
    // req.push_str(&String::from("\n"));
    // req.push_str(&content);
    // println!("Making request:\n{}\n", req);
    // stream.write(req.as_bytes()).unwrap();
    // let mut buf = [0; 1024];
    // stream.read(&mut buf).unwrap();
    // println!("Got response:\n{}\n", String::from_utf8_lossy(&buf));
}
fn get_ssl_stream(server_address: &String) -> SslStream<TcpStream> {
    let mut server_address = String::from(format!("{}:443", server_address));
    println!("Setting up a https connection to {}", server_address);
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();
    let stream = TcpStream::connect(&server_address).unwrap();
    connector.connect(&server_address, stream).unwrap()
}
