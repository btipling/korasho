use std::str;
use time::get_time;

#[derive(Default)]
#[derive(Debug)]
struct ConnectionState {
    nick: String,
    server_address: String,
    current_buf: String,
    identified: bool,
}

#[derive(Debug)]
pub struct IRC {
    connection: ::connection::Connection,
    config: ::config::Config,
    conn_state: ConnectionState,
}

#[derive(Debug)]
enum IRCMessageType {
    NOTICE(String),
}

#[derive(Debug)]
struct IRCServerMessage {
    server: String,
    message: IRCMessageType,
    time: i64,
}

#[derive(Debug)]
struct IRCCommMessage {
    time: i64,
}

#[derive(Debug)]
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
        self.conn_state.current_buf.push_str(result_str);
        loop {
            let i = match self.conn_state.current_buf.find('\n') {
                Some(i) => i,
                _ => break,
            };
            let new_buf = self.conn_state.current_buf.clone();
            let (line, rest) = new_buf.split_at(i + 1);
            self.conn_state.current_buf = rest.to_string();
            self.process_line(line.to_string());
        }
    }

    fn process_line(&mut self, line: String) {
        println!("-> {line}", line=line);
        let message = self.parse_line(line);
        println!("Received message: {:?}", message);
        if !self.conn_state.identified {
        }
    }

    fn parse_server_line(&mut self, line_bytes: &[u8]) -> Option<IRCMessage> {
        let new_line = line_bytes.clone();
        let (_, line_bytes) = match new_line.split_first() {
            Some(b) => b,
            None => return None,
        };
        let mut line_iter = line_bytes.splitn(2, |x| *x == b':');
        let meta_parts = match line_iter.next() {
            Some(p) => p,
            None => return None,
        };
        let message = match line_iter.next() {
            Some(m) => String::from_utf8_lossy(m),
            None => return None,
        };
        let message = message.into_owned();
        let mut meta_iter = meta_parts.split(|x| *x == b' ');
        let server = match meta_iter.next() {
            Some(m) => String::from_utf8_lossy(m),
            None => return None,
        };
        let server = server.into_owned();
        let server_message_type = match meta_iter.next() {
            Some(m) => String::from_utf8_lossy(m),
            None => return None,
        };
        let server_message_type = server_message_type.into_owned();
        println!("server_message_type {:?}", server_message_type);
        let message = match self.make_message(&server_message_type, &message) {
            Some(m) => m,
            _ => return None,
        };
        let time = get_time();
        let server_message = IRCServerMessage {
            server: server,
            message: message,
            time: time.sec,
        };
        let irc_message = IRCMessage::IRCServerMessage(server_message);
        return Some(irc_message);
    }

    fn parse_comm_line(&mut self, line_bytes: &[u8]) -> Option<IRCMessage> {
        return None;
    }

    fn parse_line(&mut self, line: String) -> Option<IRCMessage> {
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
    }

    fn make_message(&mut self, message_type: &str, message: &str) -> Option<IRCMessageType> {
        let stored_message = message.to_string();
        match message_type {
            "NOTICE" => Some(IRCMessageType::NOTICE(stored_message)),
            _ => None,
        }
    }


}

pub fn new(connection: ::connection::Connection, config: ::config::Config) -> IRC {
    IRC {
        connection: connection,
        config: config,
        conn_state: ConnectionState {
            identified: false,
            ..Default::default()
        }
    }
}


