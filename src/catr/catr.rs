use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    path: PathBuf,

    #[arg(short = 'n', help = "Number lines")]
    number: bool,
}

fn main() {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path);

    match content {
        Ok(content) => {
            if args.number {
                for (i, line) in content.lines().enumerate() {
                    println!("{}: {}", i + 1, line);
                }
            } else {
                println!("{}", content);
            }
        }
        Err(e) => panic!("There was a problem opening the file: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::predicate;

    #[test]
    fn run_catr() {
        let mut cmd = Command::cargo_bin("catr").unwrap();
        cmd.arg("src/catr/test.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains("catr in file!\ncatr in file!!"));
    }

    #[test]
    fn run_catr_number() {
        let mut cmd = Command::cargo_bin("catr").unwrap();
        cmd.args(["-n", "src/catr/test.txt"])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "1: catr in file!\n2: catr in file!!",
            ));
    }
}
