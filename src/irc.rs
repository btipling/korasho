use std::str;
use time;

#[derive(Default)]
#[derive(Debug)]
struct ConnectionState {
    nick: String,
    server_address: String,
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
    MODE(String),
    INFO(u64),
}

#[derive(Debug)]
pub struct IRCServerMessage {
    pub server: String,
    pub message: IRCMessageType,
    pub time: i64,
    pub raw: String,
    pub target: String,
}

#[derive(Debug)]
pub struct IRCPing {
    pub time: i64,
}

#[derive(Debug)]
pub struct IRCCommMessage {
    pub time: i64,
    pub raw: String,
}

#[derive(Debug)]
pub enum IRCMessage {
    IRCServerMessage(IRCServerMessage),
    IRCCommMessage(IRCCommMessage),
    IRCPing(IRCPing),
}

const NICK: &'static str = "NICK";
const USER: &'static str = "USER";
const PONG: &'static str = "PONG";

impl IRC {
    pub fn run (&mut self) {
        loop {
            let mut buf: Vec<u8> = Vec::new();
            if let Ok(result) = self.connection.read(&mut buf) {
                if let Ok(result_str) = str::from_utf8(&buf) {
                    self.process_line(&result_str);
                }
            }
        }
    }

    fn process_line(&mut self, line: &str) {
        if line.len() < 1 {
            return;
        }
        let line = line.to_string();
        let debug = line.clone();
        let message = match ::irc_parser::parse_line(line) {
            Some(m) => m,
            None => {
                println!("Err parsing: {}", debug);
                return;
            },
        };
        match message {
            IRCMessage::IRCServerMessage(m) => self.process_server_message(m),
            IRCMessage::IRCCommMessage(m) => self.process_com_message(m),
            IRCMessage::IRCPing(p) => self.handle_ping(p),
        }
    }

    fn format_time(&mut self, seconds: i64) -> String {
        let timespec = time::Timespec::new(seconds, 0);
        let at = time::at_utc(timespec);
        match time::strftime("[%F  %T]", &at) {
            Ok(s) => s,
            Err(_) => "[????]".to_string(),
        }
    }

    fn handle_ping(&mut self, ping: IRCPing) {
        let time = self.format_time(ping.time);
        println!("<- {time} PING {server}", time=time, server=self.conn_state.server_address);
        let server_address = self.conn_state.server_address.clone();
        self.send_command(PONG, &server_address);
    }

    fn process_server_message(&mut self, message: IRCServerMessage) {
        println!("<- {time} {line}", time=self.format_time(message.time), line=message.raw);
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
        self.nick();
        self.user();
    }

    fn nick(&mut self) {
        let nick = self.config.nick.clone();
        self.send_command(NICK, &nick);
    }

    fn user(&mut self) {
        let user = self.config.username.clone();
        let realname = self.config.realname.clone();
        let message = format!("{user} 0 * :{realname}", user=user, realname=realname);
        self.send_command(USER, &message);
    }

    fn send_command(&mut self, cmd: &str, message: &str) {
        let cmd = format!("{cmd} {message} \r\n", cmd=cmd, message=message);
        self.send_raw(cmd.as_ref());
    }

    fn send_raw(&mut self, message: &str) {
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


