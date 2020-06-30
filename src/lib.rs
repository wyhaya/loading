use std::io::{stdout, Write};
use std::result::Result::Ok;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Loading {
    sender: Option<Sender<Signal>>,
    frames: Frames,
    interval: Duration,
}

impl Loading {
    pub fn new() -> Self {
        Self {
            sender: None,
            frames: Frames::new(vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
            interval: Duration::from_millis(80),
        }
    }

    pub fn frame(mut self, frames: Vec<&'static str>) -> Self {
        self.frames = Frames::new(frames);
        self
    }

    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    pub fn start(&mut self) {
        let (se, re) = mpsc::channel();
        self.sender = Some(se.clone());
        Self::update_stdout(re);
        Self::update_animation(se.clone(), self.frames.clone(), self.interval);
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

    pub fn warn<T: ToString>(&self, text: T) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Signal::Next(Status::Warn, text.to_string()));
        }
    }

    pub fn info<T: ToString>(&self, text: T) {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Signal::Next(Status::Info, text.to_string()));
        }
    }

    fn update_animation(sender: Sender<Signal>, mut frames: Frames, duration: Duration) {
        thread::spawn(move || {
            sender.send(Signal::Frame(frames.next()));
            loop {
                thread::sleep(duration);
                if sender.send(Signal::Frame(frames.next())).is_err() {
                    break;
                }
            }
        });
    }

    fn update_stdout(receiver: Receiver<Signal>) {
        thread::spawn(move || {
            let mut stdout = stdout();
            let mut frame = "";
            let mut text = String::new();

            macro_rules! write_content {
                () => {
                    let _ = stdout.write(b"\x1B[2K\x1B[0G");
                    let _ = stdout.flush();
                };
                ($($arg:tt)*) => {
                    let _ = stdout.write(b"\x1B[2K\x1B[0G");
                    let _ = stdout.write(format!($($arg)*).as_bytes());
                    let _ = stdout.flush();
                };
            }

            while let Ok(signal) = receiver.recv() {
                match signal {
                    Signal::Frame(s) => {
                        frame = s;
                        write_content!("{} {}", frame, text);
                    }
                    Signal::Text(s) => {
                        text = s;
                        write_content!("{} {}", frame, text);
                    }
                    Signal::Next(status, s) => {
                        write_content!("{} {}\n", status.as_str(), s);
                    }
                    Signal::Exit => {
                        write_content!();
                        break;
                    }
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
struct Frames {
    index: usize,
    frames: Vec<&'static str>,
}

impl Frames {
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
    Warn,
    Info,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Success => "\x1B[32m✔\x1B[0m",
            Status::Fail => "\x1B[31m✖\x1B[0m",
            Status::Warn => "\x1B[33m⚠\x1B[0m",
            Status::Info => "\x1B[34mℹ\x1B[0m",
        }
    }
}
