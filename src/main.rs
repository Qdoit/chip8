use chip8::*;

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let filepath = args.next().unwrap();
    let rom = std::fs::read(filepath).unwrap();
    app::drive(&rom).unwrap();
    println!("Hello, world!");
}
