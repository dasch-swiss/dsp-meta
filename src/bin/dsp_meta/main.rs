mod cli;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    cli::parse()?;
    anyhow::Ok(())
}
