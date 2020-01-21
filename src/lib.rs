use std::io::{stdout, Write};
use std::result::Result::Ok;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Loading {
    sender: Sender<Signal>,
}

#[derive(Debug, Clone)]
enum Signal {
    Shape(&'static str),
    Text(String),
    End(String, Exit),
}

#[derive(Debug, Clone)]
enum Exit {
    Success,
    Fail,
}
impl Exit {
    fn as_str(&self) -> &'static str {
        match self {
            Exit::Success => "✔",
            Exit::Fail => "✖",
        }
    }
}

impl Loading {
    pub fn new() -> Self {
        let (se, re) = mpsc::channel();
        Self::update_stdout(re);
        Self::update_animation(
            se.clone(),
            Animation::new(vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            Duration::from_millis(80),
        );

        Self { sender: se }
    }

    pub fn builder(frames: Vec<&'static str>, interval: Duration) -> Self {
        let (se, re) = mpsc::channel();
        Self::update_stdout(re);
        Self::update_animation(se.clone(), Animation::new(frames), interval);

        Self { sender: se }
    }

    fn update_animation(sender: Sender<Signal>, mut animation: Animation, duration: Duration) {
        thread::spawn(move || loop {
            thread::sleep(duration);
            if sender.send(Signal::Shape(animation.next())).is_err() {
                break;
            }
        });
    }

    fn update_stdout(receiver: Receiver<Signal>) {
        let mut stdout = stdout();

        thread::spawn(move || {
            let mut shape = "";
            let mut text = String::new();

            while let Ok(signal) = receiver.recv() {
                match signal {
                    Signal::Shape(s) => {
                        shape = s;

                        stdout.write(b"\x1B[0E");
                        stdout.write(format!("{} {}", shape, text).as_bytes());
                        stdout.flush();
                    }
                    Signal::Text(s) => {
                        text = s;

                        stdout.write(b"\x1B[0E");
                        stdout.write(format!("{} {}", shape, text).as_bytes());
                        stdout.flush();
                    }
                    Signal::End(s, exit) => {
                        text = s;
                        shape = exit.as_str();

                        stdout.write(b"\x1B[0E");
                        stdout.write(format!("{} {}", shape, text).as_bytes());
                        stdout.flush();
                        break;
                    }
                }
            }
        });
    }

    pub fn text<T: ToString>(&self, text: T) {
        let _ = self.sender.send(Signal::Text(text.to_string()));
    }

    pub fn success<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::End(text.to_string(), Exit::Success));
    }

    pub fn fail<T: ToString>(&self, text: T) {
        let _ = self.sender.send(Signal::End(text.to_string(), Exit::Fail));
    }
}

struct Animation {
    index: usize,
    frames: Vec<&'static str>,
}

impl Animation {
    fn new(frames: Vec<&'static str>) -> Self {
        Self { index: 0, frames }
    }

    fn next(&mut self) -> &'static str {
        match self.frames.get(self.index) {
            Some(s) => {
                self.index += 1;
                s
            }
            None => {
                self.index = 0;
                self.frames[0]
            }
        }
    }
}
