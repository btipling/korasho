use openssl::ssl::{Ssl, SslContext, SslMethod, SslStream};
use std::net::TcpStream;
use std::io::prelude::*;
use std::io;

#[derive(Debug)]
pub enum IRCStream {
    PlainText(TcpStream),
    Secure(SslStream<TcpStream>),
}

#[derive(Debug)]
pub struct Connection {
    pub server: ::config::Server,
    pub stream: IRCStream,
}

impl Connection {
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.stream {
            IRCStream::PlainText(ref mut s) => s.write(buf),
            IRCStream::Secure(ref mut s) => s.write(buf),
        }
    }
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.stream {
            IRCStream::PlainText(ref mut s) => s.read(buf),
            IRCStream::Secure(ref mut s) => s.read(buf),
        }
    }
}

pub fn connect(server: ::config::Server) -> Result<Connection, String> {
    println!("Connecting to: {server}", server=server);
    let address = format!("{host}:{port}", host=server.host, port=server.port).to_string();

    let stream = match TcpStream::connect(&*address) {
        Ok(c) => c,
        Err(err) => return Err(format!("Unable to create a TCP connection: {err}", err=err)),
    };
    if server.secure {
        let context = match SslContext::new(SslMethod::Sslv23) {
            Ok(c) => c,
            Err(err) => return Err(format!("Unable to make an SSL context: {err}", err=err)),
        };
        let ssl = match Ssl::new(&context) {
            Ok(c) => c,
            Err(err) => return Err(format!("Unable to make an SSL object: {err}", err=err)),
        };
        let stream = match SslStream::connect(ssl, stream) {
            Ok(c) => c,
            Err(err) => return Err(format!("To create SSL connection: {err}", err=err)),
        };
        return Ok(Connection {
            server: server,
            stream: IRCStream::Secure(stream),
        })
    }
    Ok(Connection {
        server: server,
        stream: IRCStream::PlainText(stream),
    })
}
