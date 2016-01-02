use openssl::ssl::{Ssl, SslContext, SslMethod, SslStream};
use std::net::TcpStream;
use std::io::prelude::*;
use std::str;
use std::fmt;
use std::error;
use std::result;

pub trait Connection: Read + Write {}
impl<T: Read + Write> Connection for T {}
type Result<Connection> = result::Result<Connection, String>;

fn get_error(server: ::config::Server, err: error::Error) -> String {
    format!("Could not connect to {server} due to {err}", server=server, err=err);
}

pub fn connect(server: ::config::Server) -> Result<Connection> {
    println!("Connecting to: {server}", server=server);
    let address = format!("{host}:{port}", host=server.host, port=server.port).to_string();

    let mut stream = match TcpStream::connect(&*address) {
        Ok(ok) => ok,
        Err(err) => {
            return Err(get_error(server, err));
        }
    };
    if server.secure {
        let context = SslContext::new(SslMethod::Sslv23).unwrap();
        let ssl = Ssl::new(&context).unwrap();
        match SslStream::connect(ssl, stream) {
            Ok(ok) => return Ok(ok),
            Err(err) => {
                return Err(get_error(server, err));
            }
        }
    }
    return Ok(stream);
}
