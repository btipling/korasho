extern crate toml;


use std::fs::File;
use std::env;
use std::env::Args;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

struct Config<'a> {
    host: & 'a str,
    port: u16,
}

fn read_file_name(args: &mut Args) -> String {
    let filename = if args.len() > 1 {
        return args.nth(1).unwrap();
    } else {
        panic!("Need a config file path!");
    };
}

fn main() {
    println!("Starting");

    let filename = read_file_name(&mut env::args());
    println!("Using config in {filename}", filename=filename);
    //let config = readConfig(filename);
    //println!("Using config with values: {host} {port}", host=config.host, port=config.port);
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
