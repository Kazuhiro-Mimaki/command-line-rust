use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    file: PathBuf,
}

fn main() {
    let args = Cli::parse();

    match fs::read_to_string(&args.file) {
        Ok(content) => {
            let mut prev = String::new();

            for line in content.lines() {
                if line.trim_end() != prev.trim_end() {
                    print!("{}{}", line, "\n");
                }
                prev = line.to_string();
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
        let mut cmd = Command::cargo_bin("uniqr").unwrap();
        cmd.arg("src/uniqr/inputs/empty.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains(""));
    }
}
