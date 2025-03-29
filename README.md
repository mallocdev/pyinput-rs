# A Python-like input library for Rust

## INSTALL
```toml
[dependencies]
pyinput = { git = "https://github.com/mallocdev/pyinput-rs.git" }
```

## USAGE
```rust
use pyinput::input;

fn main() {
    let input = input("Enter your name: ").unwrap();
    println!("Hello, {}!", input);
}
```
