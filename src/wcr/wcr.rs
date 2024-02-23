use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    match fs::read_to_string(&args.path) {
        Ok(content) => {
            let (line_count, word_count, byte_count) = get_file_counts(content);
            println!(
                "{} {} {} {:?}",
                line_count, word_count, byte_count, args.path
            );
        }
        Err(e) => panic!("There was a problem opening the file: {:?}", e),
    }
}

fn get_file_counts(content: String) -> (usize, usize, usize) {
    if content.len() == 0 {
        return (0, 0, 0);
    }
    let line_count = content.lines().count();
    let word_count = content.split_whitespace().count();
    let byte_count = content.len();
    return (line_count, word_count, byte_count);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::predicate;

    #[test]
    fn run_empty() {
        let mut cmd = Command::cargo_bin("wcr").unwrap();
        cmd.arg("src/wcr/inputs/empty.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "0 0 0 \"src/wcr/inputs/empty.txt\"\n",
            ));
    }

    #[test]
    fn run_single_file() {
        let mut cmd = Command::cargo_bin("wcr").unwrap();
        cmd.arg("src/wcr/inputs/single.txt")
            .assert()
            .success()
            .stdout(predicate::str::contains(
                "10 10 49 \"src/wcr/inputs/single.txt\"\n",
            ));
    }
}
