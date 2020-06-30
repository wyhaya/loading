use loading::Loading;
use std::thread;
use std::time::Duration;

fn main() {
    let mut loading = Loading::new();
    // vec!["◐", "◓", "◑", "◒"]

    loading.start();

    for status in 0..4 {
        for i in 0..5 {
            loading.text(format!("Loading {}", i));
            thread::sleep(Duration::from_millis(200));
        }
        match status {
            0 => loading.success("Successs ..."),
            1 => loading.fail("Fail ..."),
            2 => loading.warn("Warn ..."),
            3 => loading.info("Info ..."),
            _ => {}
        };
    }

    loading.end();
}
