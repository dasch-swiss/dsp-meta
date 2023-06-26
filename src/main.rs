mod cli;
mod cmd;

fn main() {
    println!("Hello, world!");
    cmd::validate::read_metadata();
    cli::parse();
}
