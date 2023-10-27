#[cfg(test)]
pub mod tests;
pub mod interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 { eprintln!("Usage: cargo run <file.c>"); return; }
    let content = std::fs::read_to_string(&args[1]).expect("Could not read file");
    let mut interpreter = interpreter::Interpreter::new(&content);
    interpreter.interpret();
    println!("Hello, world!");
}
