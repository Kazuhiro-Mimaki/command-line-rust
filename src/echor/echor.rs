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

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::predicate;

    #[test]
    fn run_echor() {
        let mut cmd = Command::cargo_bin("echor").unwrap();
        cmd.arg("hello")
            .assert()
            .success()
            .stdout(predicate::str::contains("hello"));
    }
}
