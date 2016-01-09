use std::str;
use std::slice::Split;

#[derive(Default)]
struct ConnectionState {
    nick: String,
    serverAddress: String,
    currentBuffer: String,
    identified: bool,
}

pub struct IRC {
    connection: ::connection::Connection,
    config: ::config::Config,
    connectionState: ConnectionState,
}

enum IRCMessageType {
    NOTICE(String),
}

struct IRCServerMessage {
    server: String,
    message: IRCMessageType,
    time: u64,
}

struct IRCCommMessage {
    time: u64,
}

enum IRCMessage {
    IRCServerMessage(IRCServerMessage),
    IRCCommMessage(IRCCommMessage),
}

impl IRC {
    pub fn run (&mut self) {
        let _ = self.connection.write(&[1]);
        let mut buf = [0; 128];
        loop {
            let result = self.connection.read(&mut buf).unwrap(); // ignore here too
            if result < 1 {
                continue;
            }
            let result_str = str::from_utf8(&buf).unwrap();
            self.process_data(&result_str);
        }
    }

    fn process_data(&mut self, result_str: &str) {
        self.connectionState.currentBuffer.push_str(result_str);
        loop {
            let i = match self.connectionState.currentBuffer.find('\n') {
                Some(i) => i,
                _ => break,
            };
            let newBuf = self.connectionState.currentBuffer.clone();
            let (line, rest) = newBuf.split_at(i + 1);
            self.connectionState.currentBuffer = rest.to_string();
            self.process_line(line.to_string());
        }
    }

    fn process_line(&mut self, line: String) {
        println!("Processing line: {line}", line=line);
        self.parse_line(line);
        if !self.connectionState.identified {
        }
    }

    fn parse_server_line(&mut self, mut line_bytes: &[u8]) -> Option<IRCMessage> {
        let mut new_line = line_bytes.clone();
        let (_, line_bytes) = match new_line.split_first() {
            Some(b) => b,
            None => return None,
        };
        println!("lol: {:?}", String::from_utf8_lossy(line_bytes));
        let mut line_iter = line_bytes.splitn(2, |x| *x == b':');
        let meta_parts = match line_iter.next() {
            Some(p) => p,
            None => return None,
        };
        let message = match line_iter.next() {
            Some(m) => String::from_utf8_lossy(m),
            None => return None,
        };
        let mut meta_iter = meta_parts.splitn(2, |x| *x == b' ');
        let server = match meta_iter.next() {
            Some(m) => String::from_utf8_lossy(m),
            None => return None,
        };
        let server_message_type = match meta_iter.next() {
            Some(m) => String::from_utf8_lossy(m),
            None => return None,
        };
        println!("\n\nServer: '{server}'\n message type: '{message_type}'\n message: {message}\n",
                 server=server, message_type=server_message_type, message=message);
        return None;
    }

    fn parse_comm_line(&mut self, mut line_bytes: &[u8]) -> Option<IRCMessage> {
        return None;
    }

    fn parse_line(&mut self, mut line: String) -> Option<IRCMessage> {
        let line_bytes: &[u8] = line.as_ref();
        let len = line_bytes.len();
        if len < 3 {
            return None;
        }
        let line_bytes = &line_bytes[0..len-2];
        if line_bytes[0] == b':' {
            return self.parse_server_line(line_bytes);
        }
        return self.parse_comm_line(line_bytes);
        /*
        line = rest.to_string();
        let i = match line.find(' ') {
            Some(i) => i,
            _ => return None,
        };
        new_line = line.clone();
        let (server_str, rest) = new_line.split_at(i);
        let server = server_str.to_string();
        line = rest.to_string();
        let i = match line.find(' ') {
            Some(i) => i,
            _ => return None,
        };
        new_line = line.clone();
        let (message_type_str, rest) = new_line.split_at(i);
        let message_type = message_type_str.clone();
        line = rest.to_string();
        let i = match line.find(':') {
            Some(i) => i,
            _ => return None,
        };
        new_line = line.clone();
        let (_, message_str) = new_line.split_at(i);
        let message = match self.make_message(&message_type, &message_str) {
            Some(m) => m,
            _ => return None,
        };
        Some(IRCMessage {
            server: server,
            message: message,
            time: 0,
        })
        */

    }

    fn make_message(&mut self, message_type: &str, message: &str) -> Option<IRCMessageType> {
        let stored_message = message.to_string();
        match message_type {
            "notice" => Some(IRCMessageType::NOTICE(stored_message)),
            _ => None,
        }
    }


}

pub fn new(mut connection: ::connection::Connection, config: ::config::Config) -> IRC {
    IRC {
        connection: connection,
        config: config,
        connectionState: ConnectionState {
            identified: false,
            ..Default::default()
        }
    }
}


