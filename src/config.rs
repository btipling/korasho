use std::fs::File;
use std::io::prelude::*;
use std::collections::{BTreeMap};
use std::fmt;
use toml::Value;

pub struct Server {
    pub host: String,
    pub port: u16,
    pub secure: bool,
}

pub struct Config {
    pub servers: Vec<Server>
}


impl fmt::Display for Server {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.host, self.port)
    }
}

impl Clone for Server {
    fn clone(&self) -> Server {
        return Server {
            host: self.host.clone(),
            port: self.port,
            secure: self.secure,
        }
    }
}

pub fn read_config(filename: &String) -> Config {
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
        let host = match read_string(toml_server, "host") {
            Ok(h) => h,
            Err(err) => panic!("host needs to exist: {err}!", err=err),
        };
        let port = match read_integer(toml_server, "port") {
            Ok(p) => p,
            Err(err) => panic!("port needs to exist: {err}!", err=err),
        };
        let secure = match read_bool(toml_server, "secure") {
            Ok(s) => s,
            _ => false,
        };
        let port: u16 = port as u16;
        println!("found address: {host}:{port} {secure}", host=host, port=port, secure=secure);
        servers.push(Server { host: host.clone(), port: port, secure: secure })
    }
    Config {servers: servers}
}

fn read_string(server: &BTreeMap<String, Value>, name: &str) -> Result<String, String> {
    let str = match server.get(name) {
        Some(s) => s.clone(),
        None => return Err(format!("{name} was not found.", name=name)),
    };
    match str {
        Value::String(s) => return Ok(s),
        _ => return Err(format!("{name} is not a string.", name=name)),
    };
}

fn read_integer(server: &BTreeMap<String, Value>, name: &str) -> Result<i64, String> {
    let int = match server.get(name) {
        Some(i) => i.clone(),
        None => return Err(format!("{name} was not found.", name=name)),
    };
    match int {
        Value::Integer(i) => return Ok(i),
        _ => return Err(format!("{name} is not a integer.", name=name)),
    };
}

fn read_bool(server: &BTreeMap<String, Value>, name: &str) -> Result<bool, String> {
    let boolean = match server.get(name) {
        Some(b) => b.clone(),
        None => return Err(format!("{name} was not found.", name=name)),
    };
    match boolean {
        Value::Boolean(b) => return Ok(b),
        _ => return Err(format!("{name} is not a boolean.", name=name)),
    };
}
