use std::fs::File;
use std::io::prelude::*;

pub struct LogInState {
    token: String,
}

impl LogInState {
    pub fn new() -> LogInState {
        let file = File::open("login.txt");
        match file {
            Ok(mut file) => {
                let mut token = String::new();
                file.read_to_string(&mut token).expect("reading saved token error");
                LogInState {
                    token,
                }
            },
            Err(_) => {
                LogInState {
                    token: String::new(),
                }
            },
        }
    }

    pub fn get_token(&mut self) -> String {
        self.token.clone()
    }

    pub fn set_token(&mut self, token: String) {
        self.token = token;
        self.save();
    }

    fn save(&mut self) {
        let file = File::create("login.txt");
        let mut file = match file {
            Ok(file) => file,
            Err(err) => {
                panic!("error writing login token {}", err);
            },
        };
        file.write_all(self.token.as_bytes()).expect("saving login token error");
    }
}