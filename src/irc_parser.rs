use time::get_time;

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
    return parse_comm_line(line_bytes);
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
    let message = match make_message(&server_message_type, &message) {
        Some(m) => m,
        _ => return None,
    };
    let time = get_time();
    let server_message = ::irc::IRCServerMessage {
        server: server,
        message: message,
        time: time.sec,
    };
    let irc_message = ::irc::IRCMessage::IRCServerMessage(server_message);
    return Some(irc_message);
}

fn parse_comm_line(line_bytes: &[u8]) -> Option<::irc::IRCMessage> {
    return None;
}

fn make_message(message_type: &str, message: &str) -> Option<::irc::IRCMessageType> {
    let stored_message = message.to_string();
    match message_type {
        "NOTICE" => Some(::irc::IRCMessageType::NOTICE(stored_message)),
        _ => None,
    }
}
