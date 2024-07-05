//! Example:
//! ```
//! use loading::Loading;
//! use std::thread;
//! use std::time::Duration;
//!
//! let loading = Loading::default();
//!
//! for i in 0..=100 {
//!     loading.text(format!("Loading {}", i));
//!     thread::sleep(Duration::from_millis(50));
//! }
//!
//! loading.success("OK");
//!
//! loading.end();
//! ```

use std::io::{stderr, stdout, Result, Stderr, Stdout, Write};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Loading {
    sender: Sender<Signal>,
}

impl Default for Loading {
    fn default() -> Self {
        Self::with_stdout(Spinner::default())
    }
}

impl Loading {
    /// Create a stdout loading
    pub fn with_stdout(spinner: Spinner) -> Self {
        Self::create(spinner, Output::Stdout(stdout()))
    }

    /// Create a stderr loading
    pub fn with_stderr(spinner: Spinner) -> Self {
        Self::create(spinner, Output::Stderr(stderr()))
    }

    fn create(spinner: Spinner, output: Output) -> Self {
        let (sender, receiver) = mpsc::channel();

        Self::update_output(receiver, output);
        Self::update_animation(sender.clone(), spinner);

        Self { sender }
    }

    /// End loading
    pub fn end(self) {
        let (sender, receiver) = mpsc::channel();
        let _ = self.sender.send(Signal::Exit(sender));
        // Waiting for the sub -thread to exit
        let _ = receiver.recv();
    }

    /// Modify the currently displayed text
    pub fn text<T: ToString>(&self, text: T) {
        let _ = self.sender.send(Signal::Text(text.to_string()));
    }

    /// Save the current line as 'success' and continue to load on the next line
    pub fn success<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Success, text.to_string()));
    }

    /// Save the current line as 'fail' and continue to load on the next line
    pub fn fail<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Fail, text.to_string()));
    }

    /// Save the current line as 'warn' and continue to load on the next line
    pub fn warn<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Warn, text.to_string()));
    }

    /// Save the current line as 'info' and continue to load on the next line
    pub fn info<T: ToString>(&self, text: T) {
        let _ = self
            .sender
            .send(Signal::Next(Status::Info, text.to_string()));
    }

    fn update_animation(sender: Sender<Signal>, mut spinner: Spinner) {
        thread::spawn(move || {
            while sender.send(Signal::Frame(spinner.next())).is_ok() {
                thread::sleep(spinner.interval);
            }
        });
    }

    fn update_output(receiver: Receiver<Signal>, mut output: Output) {
        thread::spawn(move || {
            let mut frame = "";
            let mut text = String::new();

            macro_rules! write_content {
                () => {
                    let _ = output.write(b"\x1B[2K\x1B[0G");
                    let _ = output.flush();
                };
                ($($arg:tt)*) => {
                    let _ = output.write(b"\x1B[2K\x1B[0G");
                    let _ = output.write(format!($($arg)*).as_bytes());
                    let _ = output.flush();
                };
            }

            while let Ok(signal) = receiver.recv() {
                match signal {
                    Signal::Frame(s) => {
                        frame = s;
                        write_content!("{} {}", frame, text);
                    }
                    Signal::Text(s) => {
                        write_content!("{} {}", frame, s);
                        text = s;
                    }
                    Signal::Next(status, s) => {
                        write_content!("{} {}\n", status.as_str(), s);
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
enum Output {
    Stdout(Stdout),
    Stderr(Stderr),
}

impl Write for Output {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self {
            Self::Stdout(out) => out.write(buf),
            Self::Stderr(out) => out.write(buf),
        }
    }
    #[inline]
    fn flush(&mut self) -> Result<()> {
        match self {
            Self::Stdout(out) => out.flush(),
            Self::Stderr(out) => out.flush(),
        }
    }
}

#[derive(Debug)]
enum Signal {
    Frame(&'static str),
    Text(String),
    Next(Status, String),
    Exit(Sender<()>),
}

#[derive(Debug, Clone)]
pub struct Spinner {
    index: usize,
    frames: Vec<&'static str>,
    interval: Duration,
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new(vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
    }
}

impl Spinner {
    /// Create a Spinner
    ///
    /// ```
    /// let spin = Spinner::new(vec!["∙∙∙", "●∙∙", "∙●∙", "∙∙●"])
    /// ```
    ///
    /// ```
    /// let spin = Spinner::new(vec!["+", "-", "*", "/"])
    /// ```
    pub fn new(frames: Vec<&'static str>) -> Self {
        Self {
            index: 0,
            frames,
            interval: Duration::from_millis(80),
        }
    }

    /// Change the interval between two frames
    pub fn interval(&mut self, interval: Duration) {
        self.interval = interval
    }

    fn next(&mut self) -> &'static str {
        match self.frames.get(self.index) {
            Some(s) => {
                self.index += 1;
                s
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
