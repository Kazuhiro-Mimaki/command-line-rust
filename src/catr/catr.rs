use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    path: PathBuf,

    #[arg(short = 'n', help = "Number lines")]
    number: bool,

    #[arg(short = 'b', help = "Number non-blank lines")]
    non_blank_number: bool,
}

fn main() {
    let args = Cli::parse();

    match fs::read_to_string(&args.path) {
        Ok(content) => {
            if args.number {
                for (i, line) in content.lines().enumerate() {
                    println!("{}: {}", i + 1, line);
                }
            } else if args.non_blank_number {
                let mut line_number = 1;
                for line in content.lines() {
                    if line.is_empty() {
                        println!("{}", line);
                    } else {
                        println!("{}: {}", line_number, line);
                        line_number += 1;
                    }
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
            .stdout(predicate::str::contains("catr in file!\n\ncatr in file!!"));
    }

    #[test]
    fn run_catr_number() {
        let mut cmd = Command::cargo_bin("catr").unwrap();
        cmd.args(["-n", "src/catr/test.txt"])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "1: catr in file!\n2: \n3: catr in file!!",
            ));
    }

    #[test]
    fn run_catr_non_blank_number() {
        let mut cmd = Command::cargo_bin("catr").unwrap();
        cmd.args(["-b", "src/catr/test.txt"])
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "1: catr in file!\n\n2: catr in file!!",
            ));
    }
}
