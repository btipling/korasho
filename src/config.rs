use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use toml::Value;

pub struct Server {
    pub host: String,
    pub port: u16,
    pub secure: bool,
}


impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.host, self.port)
    }
}

pub fn read_config(filename: &String) -> Vec<Server> {
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
    let mut parser = ::toml::Parser::new(&new_input);
    let rawtoml = match parser.parse() {
        Some(rawtoml) => rawtoml,
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
    let toml_servers = Value::Table(rawtoml);
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
            Some(h) => h.clone(),
            None => panic!("host needs to exist!"),
        };
        let host = match host {
            Value::String(s) => s,
            _ => panic!("host needs to be a string!"),
        };
        let port = match toml_server.get("port") {
            Some(p) => p.clone(),
            None => panic!("port needs to exist!"),
        };
        let port = match port {
            Value::Integer(s) => s,
            ref s => panic!("port needs to be an integer: {}", s),
        };
        let secure: Value = match toml_server.get("secure") {
            Some(s) => s.clone(),
            None => Value::Boolean(false),
        };
        let secure = match secure {
            Value::Boolean(s) => s,
            _ => false,
        };
        let port: u16 = port as u16;
        println!("found address: {host}:{port} {secure}", host=host, port=port, secure=secure);
        servers.push(Server { host: host.clone(), port: port, secure: secure })
    }
    servers
}

