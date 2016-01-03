use openssl::ssl::{Ssl, SslContext, SslMethod, SslStream};
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::{Result};

pub enum IRCStream {
    PlainText(TcpStream),
    Secure(SslStream<TcpStream>),
}

pub struct Connection {
    pub server: ::config::Server,
    pub stream: IRCStream,
}

impl Connection {
    pub fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.stream {
            IRCStream::PlainText(ref mut s) => s.write(buf),
            IRCStream::Secure(ref mut s) => s.write(buf),
        }
    }
    pub fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.stream {
            IRCStream::PlainText(ref mut s) => s.read(buf),
            IRCStream::Secure(ref mut s) => s.read(buf),
        }
    }
}

pub fn connect(server: ::config::Server) -> Connection {
    println!("Connecting to: {server}", server=server);
    let address = format!("{host}:{port}", host=server.host, port=server.port).to_string();

    let stream = TcpStream::connect(&*address).unwrap();
    if server.secure {
        let context = SslContext::new(SslMethod::Sslv23).unwrap();
        let ssl = Ssl::new(&context).unwrap();
        return Connection {
            server: server,
            stream: IRCStream::Secure(SslStream::connect(ssl, stream).unwrap())
        }
    }
    Connection {
        server: server,
        stream: IRCStream::PlainText(stream),
    }
}
