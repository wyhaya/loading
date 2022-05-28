
# loading

[![Crates.io](https://img.shields.io/crates/v/loading.svg?style=flat-square)](https://crates.io/crates/loading)
[![docs.rs](https://img.shields.io/badge/docs-rs-informational.svg?style=flat-square)](https://docs.rs/loading)
[![LICENSE](https://img.shields.io/crates/l/loading.svg?style=flat-square)](./LICENSE)

Used to display the loading style in the terminal, when the program needs to `wait`, or display the `progress`, it can improve the user experience.

![preview](https://user-images.githubusercontent.com/23690145/86200915-90f6ce80-bb90-11ea-8de7-37e83d124687.gif)


## Use

Add this in your `Cargo.toml`:

```toml
[dependencies]
loading = "*"
```

## Example
 
```rust
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
```

### Other example

```
cargo run --example loading
cargo run --example status
cargo run --example download
cargo run --example spinner
```

---

