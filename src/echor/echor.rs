use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "TEXT", help = "Input text")]
    text: Vec<String>,

    #[arg(short = 'n', long, help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    println!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    );
}
