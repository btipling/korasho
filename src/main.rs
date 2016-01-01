extern crate toml;

use std::fs::File;
use std::env;
use std::fmt;
use std::env::Args;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;
use toml::Value;

struct Server {
    host: String,
    port: u16,
}


impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.host, self.port)
    }
}

fn read_file_name(args: &mut Args) -> String {
    if args.len() > 1 {
        return args.nth(1).unwrap();
    } else {
        panic!("Need a config file path!");
    };
}

fn read_config(filename: &String) -> Vec<Server> {
    println!("reading config! {filename}", filename=filename);
    let mut input = String::new();
    let res = File::open(&filename).and_then(|mut f| {
        f.read_to_string(&mut input)
    });
    match res {
        Err(e) => panic!("Unable to open config file: {}", e),
        _ => {},
    };
    let new_input = input.clone();
    let mut parser = toml::Parser::new(&new_input);
    let toml = match parser.parse() {
        Some(toml) => toml,
        None => {
            for err in &parser.errors {
                let (loline, locol) = parser.to_linecol(err.lo);
                let (hiline, hicol) = parser.to_linecol(err.hi);
                println!("{}:{}:{}-{}:{} error: {}",
                filename, loline, locol, hiline, hicol, err.desc);
            }
            panic!("Unable to read config file. Is it proper toml?")
        }
    };
    let toml_servers = Value::Table(toml);
    let toml_servers = toml_servers.lookup("servers");
    let toml_servers = match toml_servers {
        None => panic!("Unable to find any servers in config!"),
        s => s.unwrap(),
    };
    let toml_servers = match *toml_servers {
        Value::Array(ref s) => s,
        _ => panic!("Config needs to be an array of servers!"),
    };
    let toml_servers = toml_servers.into_iter();
    let mut servers = Vec::new();
    for k in toml_servers {
        let toml_server = match *k {
            Value::Table(ref s) => s,
            _ => panic!("Servers need to be a table!"),
        };
        let host = match toml_server.get("host") {
            Some(h) => h,
            None => panic!("host needs to exist!"),
        };
        let host = match *host {
            Value::String(ref s) => s,
            _ => panic!("host needs to be a string!"),
        };
        let port = match toml_server.get("port") {
            Some(p) => p,
            None => panic!("port needs to exist!"),
        };
        let port = match *port {
            Value::Integer(ref s) => s,
            ref s => panic!("port needs to be an integer: {}", s),
        };
        let port: u16 = *port as u16;
        println!("found address: {host}:{port}", host=host, port=port);
        servers.push(Server { host: host.clone(), port: port })
    }
    servers
}

fn main() {
    println!("Starting");

    let filename = read_file_name(&mut env::args());
    println!("Using config in {filename}", filename=filename);
    let servers = read_config(&filename);
    if servers.len() < 1 {
        println!("Found no servers. :/");
        return;
    }
    let first_server = &servers[0];
    println!("Got server: {server}", server=first_server);
    let address = format!("{host}:{port}", host=first_server.host, port=first_server.port).to_string();

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
