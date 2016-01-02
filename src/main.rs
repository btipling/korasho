extern crate openssl;
extern crate toml;

use std::env;
use std::thread;
use std::io::prelude::*;
use std::env::Args;
use std::net::TcpStream;
use openssl::ssl::{Ssl, SslContext, SslMethod, SslStream};
use std::str;

mod config;
mod irc;
mod connect;


fn read_file_name(args: &mut Args) -> String {
    if args.len() > 1 {
        return args.nth(1).unwrap();
    } else {
        panic!("Need a config file path!");
    };
}

fn connect(server: config::Server) {
    println!("Connecting to: {server}", server=server);
    let address = format!("{host}:{port}", host=server.host, port=server.port).to_string();

    let mut stream = match TcpStream::connect(&*address) {
        Ok(ok) => ok,
        Err(err) => {
            println!("Could not connect to {address} due to {err}", address=address, err=err);
            return;
        }
    };
    if server.secure {
        let context = SslContext::new(SslMethod::Sslv23).unwrap();
        let ssl = Ssl::new(&context).unwrap();
        let mut ssl_stream = match SslStream::connect(ssl, stream) {
            Ok(s) => s,
            Err(err) => {
                println!("Could not connect to secure {address} due to {err}",
                         address=address, err=err);
                return;
            }
        };
        handle_connection(&mut ssl_stream);
        return;
    }
    handle_connection(&mut stream);
}

fn handle_connection<T: Read + Write>(stream: &mut T) {
    let _ = stream.write(&[1]);
    let mut buf = [0; 128];
    loop {
        let result = stream.read(&mut buf).unwrap(); // ignore here too
        let result_str = str::from_utf8(&buf).unwrap();
        process_data(result, &result_str);
    }
}

fn main() {
    println!("Starting bot");

    let filename = read_file_name(&mut env::args());
    println!("Using config in {filename}", filename=filename);
    let servers = config::read_config(&filename);
    if servers.len() < 1 {
        println!("Found no servers. :/");
        return;
    }

    let handles: Vec<_> = servers.into_iter().map(|server| {
        thread::spawn(move || {
            connect(server);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

fn process_data(result_size: usize, result_str: &str) {
    println!("got {size} bytes: {str}", size=result_size, str=result_str);
}
