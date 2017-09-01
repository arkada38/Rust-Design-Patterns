# Config

Sample project with *Builder*

## Structure

- Config
  - width
  - height
  - title
  - is_minimized

- ConfigBuilder

## Usage

### main

```rust
fn main() {
    let config = ConfigBuilder::new()
        .with_width(150)
        .with_title("Rust is awesome".to_string())
        .finalize();
    println!("{}", config);

    let config = ConfigBuilder::new()
        .with_width(1000)
        .with_height(750)
        .with_minimized(true)
        .finalize();
    println!("{}", config);
}
```

### Output

```bash
Width: 150
Height: 50
Title: Rust is awesome
Is minimized: false

Width: 1000
Height: 750
Title: Builder
Is minimized: true
```
