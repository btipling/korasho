#[derive(Debug)]
#[derive(Clone)]
pub enum BotJob {
    Join(String),
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
}


impl<'a> Bot<'a> {
    pub fn handle_message(&mut self, message: ::irc::IRCMessage) {
        match message {
            ::irc::IRCMessage::IRCServerMessage(m) => self.process_server_message(m),
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

    pub fn process_server_message(&mut self, message: ::irc::IRCServerMessage) {
        let message_data = message.message.clone();
        match message_data {
            ::irc::IRCMessageType::PRIVMSG(m) => self.handle_privmsg(&m, message),
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

    pub fn handle_privmsg(&mut self, privmsg: &str, message: ::irc::IRCServerMessage) {
        println!("Got a private message {:?} {:?}", privmsg, message);
    }
}

pub fn new<'a>(config: &'a ::config::Config, server: ::config::Server) -> Bot {
    Bot {
        config: config,
        server: server,
        job_queue: Vec::new(),
        bot_state: BotState {
            connected: false,
        },
    }
}


