use loading::Loading;
use std::thread;
use std::time::Duration;

fn main() {
    let loading = Loading::default();

    for status in 0..4 {
        for i in 0..5 {
            loading.text(format!("Loading {}", i));
            thread::sleep(Duration::from_millis(200));
        }
        match status {
            0 => loading.fail("Fail ..."),
            1 => loading.warn("Warn ..."),
            2 => loading.info("Info ..."),
            3 => loading.success("Successs ..."),
            _ => {}
        };
    }

    loading.end();
}
