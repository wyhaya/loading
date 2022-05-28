use loading::Loading;
use std::thread;
use std::time::Duration;

fn main() {
    let loading = Loading::default();

    for i in 0..100 {
        loading.text(format!("Download 'loading.rar' {}%", i));
        thread::sleep(Duration::from_millis(30));
    }

    loading.fail("Download 'loading.rar' failed");

    for i in 0..100 {
        loading.text(format!("Download 'loading.zip' {}%", i));
        thread::sleep(Duration::from_millis(30));
    }

    loading.success("Download 'loading.zip' successfully");

    loading.end();
}
