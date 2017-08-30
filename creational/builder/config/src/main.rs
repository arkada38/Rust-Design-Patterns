extern crate config;

use config::ConfigBuilder;

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
