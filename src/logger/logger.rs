use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

pub type Messager = Sender<Message>;
pub type Reader = Receiver<Message>;
pub struct Logger {
    rx: Reader,
    lst: Vec<String>,
}

pub enum Message {
    Flush,
    Log(String),
}

impl Logger {
    pub fn new(rx: Reader) -> Self {
        Logger {
            rx,
            lst: Vec::new(),
        }
    }
    pub fn start(mut self) {
        thread::sleep(Duration::from_millis(2000));
        thread::spawn(move || {
            for msg in self.rx {
                match msg {
                    Message::Flush => {
                        for element in self.lst {
                            println!("[LOG] {}", element);
                        }
                        self.lst = Vec::new();
                    }
                    Message::Log(x) => self.lst.push(x),
                }
            }
        });
    }
}
