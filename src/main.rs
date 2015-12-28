use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() {
    println!("Starting");

    let host = "chat.freenode.net";
    let port = "6667";
    let address = format!("{host}:{port}", host=host, port=port).to_string();

    let mut stream = TcpStream::connect(&*address).unwrap();

    let _ = stream.write(&[1]);
    let mut buf = [0; 128];
    loop {
        let result = stream.read(&mut buf).unwrap(); // ignore here too
        let result_str = str::from_utf8(&buf).unwrap();
        process_data(result, &result_str);
    }
}

fn process_data(result_size: usize, result_str: &str) {
    println!("got {size} bytes: {str}", size=result_size, str=result_str);
}
