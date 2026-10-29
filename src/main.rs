use std::env;

fn main() {
    match env::var("USERNAME") {
        Ok(value) => println!("Hello Mundo desde Platzi + Rust! hecho por: {}", value),
        _ => println!("Hello Mundo desde Platzi + Rust!"),
    };
}
