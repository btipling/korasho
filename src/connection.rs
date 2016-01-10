use openssl::ssl::{Ssl, SslContext, SslMethod, SslStream};
use std::net::TcpStream;
use std::io::prelude::*;
use std::io;

#[derive(Debug)]
pub enum IRCStream {
    PlainText(TcpStream),
    Secure(SslStream<TcpStream>),
}

impl Write for IRCStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            &mut IRCStream::PlainText(ref mut writer) => writer.write(buf),
            &mut IRCStream::Secure(ref mut writer) => writer.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            &mut IRCStream::PlainText(ref mut writer) => writer.flush(),
            &mut IRCStream::Secure(ref mut writer) => writer.flush(),
        }
    }
}


impl Read for IRCStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            &mut IRCStream::PlainText(ref mut reader) => reader.read(buf),
            &mut IRCStream::Secure(ref mut reader) => reader.read(buf),
        }
    }
}


#[derive(Debug)]
pub struct Connection {
    pub server: ::config::Server,
    pub reader: io::BufReader<IRCStream>,
    pub writer: io::BufWriter<IRCStream>,
}

impl Connection {
    pub fn write(&mut self, buf: &[u8]) {
        println!(" -> {:?}", String::from_utf8_lossy(buf));
        self.writer.write_all(buf).unwrap();
        self.writer.flush().unwrap();
    }
    pub fn read(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        self.reader.read_until(b'\n', buf)
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
        let reader = match stream.try_clone() {
            Ok(s) => io::BufReader::new(IRCStream::Secure(s)),
            Err(err) => return Err(format!("Could not clone stream: {err}", err=err)),
        };
        let writer = io::BufWriter::new(IRCStream::Secure(stream));
        return Ok(Connection {
            server: server,
            writer: writer,
            reader: reader,
        })
    }
    let reader = match stream.try_clone() {
        Ok(s) => io::BufReader::new(IRCStream::PlainText(s)),
        Err(err) => return Err(format!("Could not clone stream: {err}", err=err)),
    };
    let writer = io::BufWriter::new(IRCStream::PlainText(stream));
    Ok(Connection {
        server: server,
        writer: writer,
        reader: reader,
    })
}
