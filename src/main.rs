use anyhow::Result;
use clap::Parser;
use maintenance::maintenance;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    auto: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = maintenance::Config { auto_start: cli.auto, ..Default::default() };
    let mut component = maintenance::new(config);
    
    component.initialize()?;
    component.run()?;
    
    println!("SigmaOS-maintenance operational");
    Ok(())
}
