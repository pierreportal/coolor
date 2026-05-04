mod color_factory;
use clap::Parser;
use color_factory::{Color, ToString};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short)]
    p: Option<String>,
    #[arg(long)]
    hex: String,
}

fn main() {
    let cli = Cli::parse();
    let hex = cli.hex;
    let p = cli.p.unwrap_or("all".to_string());
    let color = Color::from_hex(&hex);

    match p {
        v if v == "rgb" => println!("{}", color.rgb.to_string()),
        v if v == "hsl" => println!("{}", color.hsl.to_string()),
        v if v == "lab" => println!("{}", color.lab.to_string()),
        v if v == "lum" => println!("{}", color.luminance_wcag),
        v if v == "all" => println!("{:#?}", color),
        _ => println!("Invalid print format: {}", p),
    };
}
