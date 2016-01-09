use std::fs::File;
use std::io::prelude::*;
use std::fmt;
use toml::Value;

const DEFAULT_BOT_NAME: &'static str = "korasho";
const DEFAULT_USERNAME: &'static str = "korasho";
const DEFAULT_REALNAME: &'static str = "korasho.bot";

#[derive(Debug)]
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

impl Clone for Server {
    fn clone(&self) -> Server {
        return Server {
            host: self.host.clone(),
            port: self.port,
            secure: self.secure,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub servers: Vec<Server>,
    pub nick: String,
    pub alt: String,
    pub username: String,
    pub realname: String,
}

impl Clone for Config {
    fn clone(&self) -> Config {
        Config {
            servers: self.servers.clone(),
            nick: self.nick.clone(),
            alt: self.alt.clone(),
            username: self.username.clone(),
            realname: self.realname.clone(),
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
            panic!("Unable to read config file. Is it proper toml?");
        }
    };
    let toml_config = Value::Table(rawtoml);
    let nick = match get_var(&toml_config, "nick").and_then(|v| as_string(v)) {
        Ok(n) => n,
        _ => DEFAULT_BOT_NAME.to_string(),
    };
    let alt = match get_var(&toml_config, "alt").and_then(|v| as_string(v)) {
        Ok(n) => n,
        _ => format!("{botname}`", botname=DEFAULT_BOT_NAME),
    };
    let username = match get_var(&toml_config, "username").and_then(|v| as_string(v)) {
        Ok(n) => n,
        _ => DEFAULT_USERNAME.to_string(),
    };
    let realname = match get_var(&toml_config, "realname").and_then(|v| as_string(v)) {
        Ok(n) => n,
        _ => DEFAULT_REALNAME.to_string(),
    };
    let toml_servers = match get_var(&toml_config, "servers").and_then(|v| as_array(v)) {
        Ok(n) => n,
        Err(err) => panic!("Config needs servers to connect to! {err}", err=err),
    };
    let mut servers = Vec::new();
    for toml_server in toml_servers.into_iter() {
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
    Config {
        nick: nick,
        alt: alt,
        servers: servers,
        username: username,
        realname: realname,
    }
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

fn as_array(value: &Value) -> Result<&Vec<Value>, String> {
    match value {
        &Value::Array(ref a) => return Ok(a),
        _ => return Err(format!("Not an array.")),
    };
}

fn get_var<'a>(map: &'a Value, name: &str) -> Result<&'a Value, String> {
    let map = match *map {
        Value::Table(ref s) => s,
        _ => return Err(format!("Not a valid map while looking up {name}", name=name)),
    };
    map.get(name).ok_or_else(|| format!("{name} not found", name=name))
}
