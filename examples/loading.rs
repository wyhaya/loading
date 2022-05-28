use loading::Loading;
use std::thread;
use std::time::Duration;

fn main() {
    let loading = Loading::default();

    for i in 0..=100 {
        loading.text(format!("Loading {}", i));
        thread::sleep(Duration::from_millis(50));
    }

    loading.success("OK");

    loading.end();
}
