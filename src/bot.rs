use std::rc;
use std::cell;

#[derive(Debug)]
pub struct Bot<'a> {
    config: &'a ::config::Config
}


impl<'a> Bot<'a> {
    pub fn handle_message(&mut self) {
    }

    pub fn get_data(&mut self) {
    }
}

pub fn new<'a>(config: &'a ::config::Config) -> Bot {
    Bot {
        config: config
    }
}


