use loading::Loading;
use std::thread;
use std::time::Duration;

fn main() {
    let loading = Loading::new();
    for i in 0..5 {
        loading.text(format!("A piece of text {}", i));
        thread::sleep(Duration::from_millis(500));
    }
    loading.fail("Loaded successfully");

    thread::sleep(Duration::from_millis(1000));

    let loading = Loading::builder(vec!["◐", "◓", "◑", "◒"], Duration::from_millis(50));
    for i in 0..5 {
        loading.text(format!("A piece of text {}", i));
        thread::sleep(Duration::from_millis(500));
    }
    loading.success("Loaded successfully");
}
