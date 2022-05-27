//! Example:
//! ```
//! use loading::Loading;
//! use std::thread;
//! use std::time::Duration;
//!
//!
//! let mut loading = Loading::new();
//!
//! loading.start();
//!
//! for i in 0..100 {
//!     loading.text(format!("Loading {}", i));
//!     thread::sleep(Duration::from_millis(50));
//! }
//!
//! loading.success("OK");
//!
//! loading.end();
//! ```

use std::io::{stdout, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Loading {
    sender: Option<Sender<Signal>>,
    frame: Frame,
    interval: Duration,
}

impl Default for Loading {
    fn default() -> Self {
        Self::new()
    }
}

impl Loading {
    /// Create a Loading
    pub fn new() -> Self {
        Self {
            sender: None,
            frame: Frame::default(),
            interval: Duration::from_millis(80),
        }
    }

    /// Modify the style of frame
    pub fn frame(mut self, frames: Vec<char>) -> Self {
        self.frame = Frame::new(frames);
        self
    }

    /// Modify the frame refresh time. Default: 80ms
    pub fn interval(mut self, interval: Duration) -> Self {
        self.interval = interval;
        self
    }

    fn sender(&self) -> &Sender<Signal> {
        self.sender
            .as_ref()
            .expect("Please call the `self.start()` first")
    }

    /// Start in the terminal
    pub fn start(&mut self) {
        if self.sender.is_some() {
            panic!("The `self.start()` can only be called once")
        }

        let (sender, receiver) = mpsc::channel();
        self.sender = Some(sender.clone());

        Self::update_stdout(receiver);
        Self::update_animation(sender, self.frame.clone(), self.interval);
    }

    /// End in terminal
    pub fn end(&self) {
        let (sender, receiver) = mpsc::channel();
        let _ = self.sender().send(Signal::Exit(sender));
        let _ = receiver.recv();
    }

    /// Modify the currently displayed text
    pub fn text<T: ToString>(&self, text: T) {
        let _ = self.sender().send(Signal::Text(text.to_string()));
    }

    /// Save the current line as 'success' and continue to load on the next line
    pub fn success<T: ToString>(&self, text: T) {
        let _ = self
            .sender()
            .send(Signal::Next(Status::Success, text.to_string()));
    }

    /// Save the current line as 'fail' and continue to load on the next line
    pub fn fail<T: ToString>(&self, text: T) {
        let _ = self
            .sender()
            .send(Signal::Next(Status::Fail, text.to_string()));
    }

    /// Save the current line as 'warn' and continue to load on the next line
    pub fn warn<T: ToString>(&self, text: T) {
        let _ = self
            .sender()
            .send(Signal::Next(Status::Warn, text.to_string()));
    }

    /// Save the current line as 'info' and continue to load on the next line
    pub fn info<T: ToString>(&self, text: T) {
        let _ = self
            .sender()
            .send(Signal::Next(Status::Info, text.to_string()));
    }

    fn update_animation(sender: Sender<Signal>, mut frame: Frame, duration: Duration) {
        thread::spawn(move || {
            while sender.send(Signal::Frame(frame.next())).is_ok() {
                thread::sleep(duration);
            }
        });
    }

    fn update_stdout(receiver: Receiver<Signal>) {
        thread::spawn(move || {
            let mut stdout = stdout();
            let mut frame = char::default();
            let mut text: Option<String> = None;

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
                        if let Some(text) = &text {
                            write_content!("{} {}", frame, text);
                        }
                    }
                    Signal::Text(s) => {
                        write_content!("{} {}", frame, s);
                        text = Some(s);
                    }
                    Signal::Next(status, s) => {
                        write_content!("{} {}\n", status.as_str(), s);
                        text = None;
                    }
                    Signal::Exit(sender) => {
                        write_content!();
                        let _ = sender.send(());
                        break;
                    }
                }
            }
        });
    }
}

#[derive(Debug)]
enum Signal {
    Frame(char),
    Text(String),
    Next(Status, String),
    Exit(Sender<()>),
}

#[derive(Debug, Clone)]
struct Frame {
    index: usize,
    frames: Vec<char>,
}

impl Default for Frame {
    fn default() -> Self {
        Self::new(vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'])
    }
}

impl Frame {
    fn new(frames: Vec<char>) -> Self {
        Self { index: 0, frames }
    }

    fn next(&mut self) -> char {
        match self.frames.get(self.index) {
            Some(s) => {
                self.index += 1;
                *s
            }
            None => {
                self.index = 1;
                self.frames[0]
            }
        }
    }
}

#[derive(Debug)]
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
