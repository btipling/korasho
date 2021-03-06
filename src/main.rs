extern crate openssl;
extern crate time;
extern crate toml;

use std::env;
use std::thread;
use std::env::Args;

mod bot;
mod config;
mod connection;
mod irc;
mod irc_parser;


fn read_file_name(args: &mut Args) -> String {
    if let Some(file_name) = args.nth(1) {
        return file_name;
    }
    panic!("Need a config file path!");
}

fn main() {
    println!("Starting bot");

    let filename = read_file_name(&mut env::args());
    println!("Using config in {filename}", filename=filename);
    let config = config::read_config(&filename);
    if config.servers.len() < 1 {
        println!("Found no servers. :/");
        return;
    }
    let handles: Vec<_> = config.servers.clone().into_iter().map(|server| {
        let captured_config = config.clone();
        thread::spawn(move || {
            let connection = match connection::connect(server.clone()) {
                Ok(s) => s,
                Err(err) => {
                    println!("Could not connect to {server}: {err}", server=server, err=err);
                    return;
                },
            };
            let bot = bot::new(&captured_config, server.clone());
            let mut irc = irc::new(connection, &captured_config, bot);
            irc.run();
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
