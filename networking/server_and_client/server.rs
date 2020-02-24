use std::net::TcpStream;
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        handle_stream(stream);
    }
}
fn handle_stream(mut stream: TcpStream) {
    read_packet(&stream);
}
fn read_packet(stream: &TcpStream) {
    let packet_len = read_var_int(&stream);
    let packet_id = read_var_int(&stream);
    // println!("Packet ID: {:?}", packet_id);
    if packet_id == 0 { // Handshake
        read_handshake(&stream);
    }
}
fn read_handshake(stream: &TcpStream) {
    let protocol_version = read_var_int(&stream);
    // println!("Protocol Version: {:?}", protocol_version);
    let server_address = read_string(&stream);
    // println!("Server Address: {:?}", server_address);
    let server_port = read_unsigned_short(&stream);
    // println!("Server Port: {:?}", server_port);
    let next_state = read_var_int(&stream);
    // println!("Next State: {:?}", next_state);
}
fn read_string(stream: &TcpStream) -> String {
    let string_length = read_var_int(&stream);
    let mut string_bytes = Vec::new();
    for _i in 0..string_length {
        string_bytes.push(read_byte(&stream));
    }
    String::from_utf8(string_bytes).unwrap()
}
fn read_var_int(stream: &TcpStream) -> i32 {
    let mut num_read: i32 = 0;
    let mut result: i32 = 0;
    let mut read: u8 = 0;
    let mut run_once = false;
    while !run_once || ((read & 0b10000000) != 0) {
        run_once = true;
        read = read_byte(&stream);
        let value: i32 = (read & 0b01111111) as i32;
        result |= value << (7 * num_read);

        num_read += 1;
        if num_read > 5 {
            panic!("VarInt is too big");
        }
    }
    result
}
fn read_unsigned_short(stream: &TcpStream) -> i32 {
    let byte1 = read_byte(&stream);
    let byte2 = read_byte(&stream);
    let together = format!("{:X?}{:X?}", byte1, byte2);
    let port = i64::from_str_radix(&together, 16).unwrap().try_into().unwrap();
    port
}
fn read_byte(mut stream: &TcpStream) -> u8 {
    let mut buffer = [0; 1];
    stream.read(&mut buffer).unwrap();
    buffer[0]
}
