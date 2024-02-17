use std::{error::Error, fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    path: PathBuf,

    #[arg(short = 'n', help = "Count lines", default_value = "10")]
    count: String,

    #[arg(short = 'c', help = "Bytes")]
    bytes: Option<String>,
}

fn main() {
    let args = Cli::parse();

    match fs::read_to_string(&args.path) {
        Ok(content) => match args.bytes {
            Some(bytes) => match parse_positive_int(bytes) {
                Ok(count) => {
                    for (i, byte) in content.bytes().enumerate() {
                        if i < count as usize {
                            println!("{}", byte);
                        }
                    }
                }
                Err(err) => {
                    println!("headr: illegal line count -- {}", err);
                }
            },
            None => match parse_positive_int(args.count) {
                Ok(count) => {
                    for (i, line) in content.lines().enumerate() {
                        if i < count as usize {
                            println!("{}", line);
                        }
                    }
                }
                Err(err) => {
                    println!("headr: illegal line count -- {}", err);
                }
            },
        },
        Err(e) => panic!("There was a problem opening the file: {:?}", e),
    }
}

fn parse_positive_int(val: String) -> Result<i32, Box<dyn Error>> {
    match val.parse::<i32>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::predicate;

    #[test]
    fn run_empty() {
        let mut cmd = Command::cargo_bin("headr").unwrap();
        cmd.arg("src/headr/inputs/empty.txt")
            .assert()
            .success()
            .stdout("");
    }

    #[test]
    fn run_default_count() {
        let mut cmd = Command::cargo_bin("headr").unwrap();
        cmd.arg("src/headr/inputs/default.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten",
            ));
    }

    #[test]
    fn run_less_than_default() {
        let mut cmd = Command::cargo_bin("headr").unwrap();
        cmd.arg("src/headr/inputs/less_than_default.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains("one\ntwo\nthree"));
    }

    #[test]
    fn run_more_than_default() {
        let mut cmd = Command::cargo_bin("headr").unwrap();
        cmd.arg("src/headr/inputs/more_than_default.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "one\ntwo\nthree\nfour\nfive\nsix\nseven\neight\nnine\nten",
            ));
    }

    #[test]
    fn run_with_count() {
        let mut cmd = Command::cargo_bin("headr").unwrap();
        cmd.args(["-n", "5", "src/headr/inputs/default.txt"])
            .assert()
            .success()
            .stdout("one\ntwo\nthree\nfour\nfive\n");
    }

    #[test]
    fn run_with_bytes() {
        let mut cmd = Command::cargo_bin("headr").unwrap();
        cmd.args(["-c", "10", "src/headr/inputs/default.txt"])
            .assert()
            .success()
            .stdout("one\ntwo\nth");
    }
}
