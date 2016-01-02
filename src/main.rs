extern crate openssl;
extern crate toml;

use std::env;
use std::thread;
use std::env::Args;

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
            let mut connection = match connect::connect(server) {
                Ok(s) => s,
                Err(err) => {
                    println!("Could not connect to secure {server} due to {err}",
                             server=server, err=err);
                    return;
                }
            };
            handle_connection(&mut connection);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}

fn handle_connection<T: connect::Connection>(stream: &mut T) {
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

