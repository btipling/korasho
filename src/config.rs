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
            _ => continue,
        };
        let host = match get_var(toml_server, "host").and_then(|v| as_string(v)) {
            Ok(h) => h,
            Err(err) => {
                println!("Skipping a server: {err}", err=err);
                continue;
            }
        };
        let port = match get_var(toml_server, "port").and_then(|v| as_integer(v)) {
            Ok(p) => p,
            Err(err) => {
                println!("Skipping a server: {err}", err=err);
                continue;
            }
        };
        let secure = match get_var(toml_server, "secure").and_then(|v| as_bool(v)) {
            Ok(s) => s,
            _ => false,
        };
        let port: u16 = port as u16;
        println!("found address: {host}:{port} {secure}", host=host, port=port, secure=secure);
        servers.push(Server { host: host.clone(), port: port, secure: secure })
    }
    Config {servers: servers}
}

fn as_string(value: &Value) -> Result<String, String> {
    match value {
        &Value::String(ref s) => return Ok(s.clone()),
        _ => return Err(format!("Not a string.")),
    };
}

fn as_integer(value: &Value) -> Result<i64, String> {
    match value {
        &Value::Integer(i) => return Ok(i),
        _ => return Err(format!("Not an integer.")),
    };
}

fn as_bool(value: &Value) -> Result<bool, String> {
    match value {
        &Value::Boolean(b) => return Ok(b),
        _ => return Err(format!("Not a boolean.")),
    };
}

fn get_var<'a>(server: &'a BTreeMap<String, Value>, name: &str) -> Result<&'a Value, String> {
    server.get(name).ok_or_else(|| format!("{name} not found", name=name))
}
