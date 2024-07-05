use loading::{Loading, Spinner};
use std::thread;
use std::time::Duration;

fn main() {
    let loading = Loading::with_stdout(Spinner::new(vec!["◐", "◓", "◑", "◒"]));
    for i in 0..10 {
        loading.text(format!("Loading {}", i));
        thread::sleep(Duration::from_millis(200));
    }
    loading.success("Successs ...");
    loading.end();

    let loading = Loading::with_stderr(Spinner::new(vec!["∙∙∙", "●∙∙", "∙●∙", "∙∙●"]));
    for i in 0..10 {
        loading.text(format!("Loading {}", i));
        thread::sleep(Duration::from_millis(200));
    }
    loading.fail("Error ...");
    loading.end();
}
