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

const NICK: &'static str = "NICK";
const USER: &'static str = "USER";

impl IRC {
    pub fn run (&mut self) {
        let mut buf = [0; 128];
        loop {
            if let Ok(result) = self.connection.read(&mut buf) {
                if result < 1 {
                    continue;
                }
                if let Ok(result_str) = str::from_utf8(&buf) {
                    self.process_data(&result_str);
                }
            }
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
        println!("<- {line}", line=line);
        let message = match ::irc_parser::parse_line(line) {
            Some(m) => m,
            None => return,
        };
        match message {
            IRCMessage::IRCServerMessage(m) => self.process_server_message(m),
            IRCMessage::IRCCommMessage(m) => self.process_com_message(m),
        }
    }

    fn process_server_message(&mut self, message: IRCServerMessage) {
        if !self.conn_state.identified {
            self.conn_state.server_address = message.server;
            self.identify();
            self.conn_state.identified = true;
        }
    }

    fn process_com_message(&mut self, message: IRCCommMessage) {
        println!("Processing communication message {:?}", message);
    }

    fn identify(&mut self) {
        self.NICK();
        self.USER();
    }

    fn NICK(&mut self) {
        let nick = self.config.nick.clone();
        self.SEND_COMMAND(NICK, &nick);
        //self.SEND_RAW("NICK foomanchu8 \n");
    }

    fn USER(&mut self) {
        let user = self.config.username.clone();
        let realname = self.config.realname.clone();
        let message = format!("{user} 0 * :{realname}", user=user, realname=realname);
        //self.SEND_RAW("USER lol 0 * :LOL \n");
        self.SEND_COMMAND(USER, &message);
    }

    fn SEND_COMMAND(&mut self, cmd: &str, message: &str) {
        let cmd = format!("{cmd} {message} \n", cmd=cmd, message=message);
        self.SEND_RAW(cmd.as_ref());
    }

    fn SEND_RAW(&mut self, message: &str) {
        self.connection.write(message.as_ref());
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


