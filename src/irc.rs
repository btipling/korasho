use std::str;

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
pub enum IRCMessageType {
    NOTICE(String),
}

#[derive(Debug)]
pub struct IRCServerMessage {
    pub server: String,
    pub message: IRCMessageType,
    pub time: i64,
}

#[derive(Debug)]
pub struct IRCCommMessage {
    pub time: i64,
}

#[derive(Debug)]
pub enum IRCMessage {
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
        let message = match ::irc_parser::parse_line(line) {
            Some(m) => m,
            None => return,
        };
        println!("Received message: {:?}", message);
        match message {
            IRCMessage::IRCServerMessage(m) => self.process_server_message(m),
            IRCMessage::IRCCommMessage(m) => self.process_com_message(m),
        }
    }

    fn process_server_message(&mut self, message: IRCServerMessage) {
        if !self.conn_state.identified {
            self.conn_state.server_address = message.server;
            self.identify()
        }
    }

    fn process_com_message(&mut self, message: IRCCommMessage) {
        println!("Processing communication message {:?}", message);
    }

    fn identify(&mut self) {
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


