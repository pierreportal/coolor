mod color_factory;
use clap::Parser;
use color_factory::Color;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(long)]
    hex: String,
}

fn main() {
    let cli = Cli::parse();
    let hex = cli.hex;
    let color = Color::from_hex(&hex);

    println!("{:#?}", color);
}
