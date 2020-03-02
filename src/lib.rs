use std::io::{stdout, Write};
use std::result::Result::Ok;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Loading {
    sender: Option<Sender<Signal>>,
    animation: Animation,
    duration: Duration,
}

impl Loading {
    pub fn new() -> Self {
        Self {
            sender: None,
            animation: Animation::new(vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            duration: Duration::from_millis(80),
        }
    }

    pub fn frame(mut self, frames: Vec<&'static str>) -> Self {
        self.animation = Animation::new(frames);
        self
    }

    pub fn update_time(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn start(&mut self) {
        let (se, re) = mpsc::channel();
        self.sender = Some(se.clone());
        Self::update_stdout(re);
        Self::update_animation(se.clone(), self.animation.clone(), self.duration);
    }

    pub fn end(&mut self) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Signal::Exit);
        }
    }

    pub fn text<T: ToString>(&self, text: T) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Signal::Text(text.to_string()));
        }
    }

    pub fn success<T: ToString>(&self, text: T) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Signal::Next(Status::Success, text.to_string()));
        }
    }

    pub fn fail<T: ToString>(&self, text: T) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Signal::Next(Status::Fail, text.to_string()));
        }
    }

    pub fn warn(&self) {

    }

    fn update_animation(sender: Sender<Signal>, mut animation: Animation, duration: Duration) {
        thread::spawn(move || loop {
            thread::sleep(duration);
            if sender.send(Signal::Frame(animation.next())).is_err() {
                break;
            }
        });
    }

    fn update_stdout(receiver: Receiver<Signal>) {
        let mut stdout = stdout();

        thread::spawn(move || {
            let mut frame = "";
            let mut text = String::new();

            while let Ok(signal) = receiver.recv() {
                match signal {
                    Signal::Frame(s) => {
                        frame = s;

                        stdout.write(b"\x1B[0E");
                        stdout.write(format!("{} {}", frame, text).as_bytes());
                        stdout.flush();
                    }
                    Signal::Text(s) => {
                        text = s;

                        stdout.write(b"\x1B[0E");
                        stdout.write(format!("{} {}", frame, text).as_bytes());
                        stdout.flush();
                    }
                    Signal::Next(status, s) => {
                        text = String::new();
                        frame = status.as_str();

                        stdout.write(b"\x1B[0E");
                        stdout.write(format!("{} {}", frame, s).as_bytes());
                        stdout.write(b"\n");
                        stdout.flush();
                    }
                    Signal::Exit => {
                        break;
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
enum Signal {
    Frame(&'static str),
    Text(String),
    Next(Status, String),
    Exit,
}

#[derive(Debug, Clone)]
enum Status {
    Success,
    Fail,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Success => "✔",
            Status::Fail => "✖",
        }
    }
}
