use std::str;

#[derive(Debug)]
#[derive(Clone)]
pub enum BotJob {
    Join(String),
    PrivMsg((String, String)),
}

#[derive(Debug)]
pub struct Bot<'a> {
    config: &'a ::config::Config,
    server: ::config::Server,
    job_queue: Vec<BotJob>,
    bot_state: BotState,
}


#[derive(Default)]
#[derive(Debug)]
struct BotState {
    connected: bool,
    admin: Option<::irc::Client>,
}

impl<'a> Bot<'a> {
    pub fn handle_message(&mut self, message: ::irc::IRCMessage,
                          conn_state: &::irc::ConnectionState) {
        match message {
            ::irc::IRCMessage::IRCServerMessage(m) => self.process_server_message(m, conn_state),
            _ => {},
        }
    }

    pub fn get_job(&mut self) -> Option<BotJob> {
        if self.job_queue.len() < 1 {
            return None;
        }
        let job = self.job_queue.remove(0);
        Some(job)

    }

    pub fn process_server_message(&mut self, message: ::irc::IRCServerMessage,
                                 conn_state: &::irc::ConnectionState) {
        let message_data = message.message.clone();
        match message_data {
            ::irc::IRCMessageType::PRIVMSG(m) => {
                self.handle_privmsg(&m[..], message, conn_state);
            },
            ::irc::IRCMessageType::INFO(i) => {
                if i > 10 && !self.bot_state.connected {
                    self.bot_state.connected = true;
                    for channel in self.server.channels.clone() {
                        self.job_queue.push(BotJob::Join(channel))
                    }
                }
            },
            _ => {},
        }
    }

    pub fn msg(&mut self, target: String, from: ::irc::Entity, message: &str,
               conn_state: &::irc::ConnectionState) {
        let client = match from {
            ::irc::Entity::Client(c) => c,
            _ => return,
        };
        let mut nick = target;
        if &conn_state.nick == &nick {
            nick = client.nick.clone();
        }
        let message = message.to_string();
        self.job_queue.push(BotJob::PrivMsg((nick, message)));
    }

    pub fn handle_privmsg(&mut self, privmsg: &[u8], message: ::irc::IRCServerMessage,
                          conn_state: &::irc::ConnectionState) {
        if privmsg[0] != self.config.command_byte {
            return;
        }
        let command_bytes = &privmsg[1..privmsg.len()];
        let mut command_iter = command_bytes.splitn(2, |x| *x == b' ');
        let command = match command_iter.next() {
            Some(c) => c,
            None => return,
        };
        let command = match str::from_utf8(command) {
            Ok(c) => c,
            _ => return,
        };
        let rest = command_iter.next();
        match command {
            "auth" => self.auth(message, rest, conn_state),
            "botsnack" => self.botsnack(message, conn_state),
            _ => println!("Unhandled command: {}", command),
        }
    }

    pub fn auth(&mut self, message: ::irc::IRCServerMessage, args: Option<&[u8]>,
                conn_state: &::irc::ConnectionState) {
        match args {
            Some(p) => {
                match str::from_utf8(p) {
                    Ok(p) => {
                        if p == self.config.admin_password {
                            let from = message.from.clone();
                            match from {
                                ::irc::Entity::Client(c) => {
                                    self.bot_state.admin = Some(c);
                                    self.msg(message.target, message.from, "Authed!", conn_state);
                                },
                                _ => {},
                            }
                            return;
                        }
                    },
                    _ => {},
                };
            },
            _ => {},
        }
        self.msg(message.target, message.from, "Not authed. :(", conn_state);
        return;
    }

    pub fn botsnack(&mut self, message: ::irc::IRCServerMessage,
                    conn_state: &::irc::ConnectionState) {
        if !self.authed(&message.from) {
            return;
        }
        self.msg(message.target, message.from, ":)", conn_state);
    }

    pub fn authed(&mut self, from: &::irc::Entity) -> bool {
        let from = match from {
            &::irc::Entity::Client(ref c) => c,
            _ => return false,
        };
        let admin = &self.bot_state.admin;
        let admin = match admin {
            &Some(ref a) => a,
            _ => return false,
        };
        return *admin == *from;
    }
}

pub fn new<'a>(config: &'a ::config::Config, server: ::config::Server) -> Bot {
    Bot {
        config: config,
        server: server,
        job_queue: Vec::new(),
        bot_state: BotState {
            connected: false,
            admin: None,
        },
    }
}


