use std::str;

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


