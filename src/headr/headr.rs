use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    path: PathBuf,

    #[arg(short = 'n', help = "Count lines", default_value = "10", value_parser = clap::value_parser!(u32))]
    count: u32,
}

fn main() {
    let args = Cli::parse();

    match fs::read_to_string(&args.path) {
        Ok(content) => {
            for (i, line) in content.lines().enumerate() {
                if i < args.count as usize {
                    println!("{}", line);
                }
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
}
