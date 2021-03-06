use time::get_time;
use std::string::String;

pub fn parse_line(line: String) -> Option<::irc::IRCMessage> {
    let line_bytes: &[u8] = line.as_ref();
    let len = line_bytes.len();
    if len < 3 {
        return None;
    }
    let line_bytes = &line_bytes[0..len-2];
    if line_bytes[0] == b':' {
        return parse_server_line(line_bytes);
    }
    if line_bytes[0] == b'P' && line_bytes.len() > 5 {
        let check = String::from_utf8_lossy(line_bytes);
        let check = &check[0..4];
        if check == "PING" {
            let time = get_time();
            let ping = ::irc::IRCPing { time: time.sec };
            return Some(::irc::IRCMessage::IRCPing(ping));
        }
    }
    println!("Unhandled line? {}", String::from_utf8_lossy(line_bytes));
    return None
}

fn parse_server_line(line_bytes: &[u8]) -> Option<::irc::IRCMessage> {
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
        Some(m) => String::from_utf8_lossy(m).into_owned(),
        None => "".to_string(),
    };
    let mut meta_iter = meta_parts.split(|x| *x == b' ');
    let from = match meta_iter.next() {
        Some(m) => m,
        None => return None,
    };
    let from = match parse_from(from) {
        Some(f) => f,
        None => return None,
    };
    let server_message_type = match meta_iter.next() {
        Some(m) => String::from_utf8_lossy(m),
        None => return None,
    };
    let server_message_type = server_message_type.into_owned();
    let target = match meta_iter.next() {
        Some(m) => String::from_utf8_lossy(m),
        None => return None,
    };
    let mut additional_data = "".to_string();
    for next in meta_iter {
        additional_data.push_str(&String::from_utf8_lossy(next));
    }
    let message = match make_message(&server_message_type, &message, &additional_data) {
        Some(m) => m,
        _ => return None,
    };
    let time = get_time();
    let server_message = ::irc::IRCServerMessage {
        from: from,
        message: message,
        time: time.sec,
        target: target.into_owned(),
        raw: String::from_utf8_lossy(line_bytes).into_owned(),
    };
    let irc_message = ::irc::IRCMessage::IRCServerMessage(server_message);
    return Some(irc_message);
}

fn parse_from(from_bytes: &[u8]) -> Option<::irc::Entity> {
    let mut from_iter = from_bytes.splitn(2, |x| *x == b'!');
    let nick = match from_iter.next() {
        Some(n) => n,
        None => return None,
    };
    let nick = String::from_utf8_lossy(nick);
    let address_bytes = match from_iter.next() {
        Some(r) => r,
        None => return Some(::irc::Entity::Server(nick.into_owned())),
    };
    let mut addresss_iter = address_bytes.splitn(2, |x| *x == b'@');
    let username = match addresss_iter.next() {
        Some(u) => u,
        None => return None,
    };
    let address = match addresss_iter.next() {
        Some(a) => a,
        None => return None,
    };
    let nick = nick.into_owned();
    let username = String::from_utf8_lossy(username).into_owned();
    let address = String::from_utf8_lossy(address).into_owned();
    let client = ::irc::Client {
        nick: nick,
        username: username,
        address: address,
    };
    Some(::irc::Entity::Client(client))
}

fn make_message(message_type: &str, message: &str, meta: &str) -> Option<::irc::IRCMessageType> {
    let stored_message = message.to_string();
    match message_type {
        "NOTICE" => Some(::irc::IRCMessageType::NOTICE(stored_message)),
        "MODE" => Some(::irc::IRCMessageType::MODE(meta.to_string())),
        "PRIVMSG" => Some(::irc::IRCMessageType::PRIVMSG(message.as_bytes().to_vec())),
        _ => {
            match message_type.parse::<u64>() {
                Ok(i) => Some(::irc::IRCMessageType::INFO(i)),
                Err(_) => None,
            }
        },
    }
}
