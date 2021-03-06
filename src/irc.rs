use std::str;
use time;

#[derive(Default)]
#[derive(Debug)]
pub struct ConnectionState {
    pub nick: String,
    pub server_address: String,
    identified: bool,
}

#[derive(Debug)]
pub struct IRC<'a> {
    connection: ::connection::Connection,
    config: &'a ::config::Config,
    conn_state: ConnectionState,
    bot: ::bot::Bot<'a>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Client {
    pub nick: String,
    pub username: String,
    pub address: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Entity {
    Server(String),
    Client(Client),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum IRCMessageType {
    NOTICE(String),
    MODE(String),
    PRIVMSG(Vec<u8>),
    INFO(u64),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct IRCServerMessage {
    pub from: Entity,
    pub message: IRCMessageType,
    pub time: i64,
    pub raw: String,
    pub target: String,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct IRCPing {
    pub time: i64,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum IRCMessage {
    IRCServerMessage(IRCServerMessage),
    IRCPing(IRCPing),
}

const NICK: &'static str = "NICK";
const JOIN: &'static str = "JOIN";
const USER: &'static str = "USER";
const PONG: &'static str = "PONG";
const PRIVMSG: &'static str = "PRIVMSG";

impl<'a> IRC<'a> {
    pub fn run (&mut self) {
        loop {
            let mut buf: Vec<u8> = Vec::new();
            if let Ok(_) = self.connection.read(&mut buf) {
                if let Ok(result_str) = str::from_utf8(&buf) {
                    self.process_line(&result_str);
                }
            }
            let job = self.bot.get_job();
            self.handle_bot_job(job);
        }
    }

    fn handle_bot_job(&mut self, bot_job: Option<::bot::BotJob>) {
        let bot_job = match bot_job {
            Some(j) => j,
            None => return,
        };
        match bot_job {
            ::bot::BotJob::Join(channel) => self.join(&channel),
            ::bot::BotJob::PrivMsg((nick, message)) => {
                self.priv_msg(&nick, &message);
            },
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
        let bot_message = message.clone();
        match message {
            IRCMessage::IRCServerMessage(m) => self.process_server_message(m),
            IRCMessage::IRCPing(p) => self.handle_ping(p),
        }
        self.bot.handle_message(bot_message, &self.conn_state);
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
            match message.from {
                Entity::Server(s) => {
                    self.conn_state.server_address = s;
                    self.identify();
                    self.conn_state.identified = true;
                },
                _ => {},
            }
        }
    }

    fn identify(&mut self) {
        self.nick();
        self.user();
    }

    fn priv_msg(&mut self, target: &str, message: &str) {
        let message = format!("{target} :{message}", target=target, message=message);
        self.send_command(PRIVMSG, &message);
    }

    fn join(&mut self, channel: &str) {
        self.send_command(JOIN, channel);
    }

    fn nick(&mut self) {
        let nick = self.config.nick.clone();
        self.send_command(NICK, &nick);
        self.conn_state.nick = nick;
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

pub fn new<'a>(
                connection: ::connection::Connection,
                config: &'a ::config::Config,
                bot: ::bot::Bot<'a>
        ) -> IRC<'a> {
    IRC {
        connection: connection,
        config: config,
        bot: bot,
        conn_state: ConnectionState {
            identified: false,
            ..Default::default()
        }
    }
}


