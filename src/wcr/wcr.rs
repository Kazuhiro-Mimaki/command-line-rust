use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    path: PathBuf,

    #[arg(short = 'l', help = "Number of lines")]
    show_line_count: bool,

    #[arg(short = 'w', help = "Number of words")]
    show_word_count: bool,

    #[arg(short = 'c', help = "Number of bytes")]
    show_byte_count: bool,
}

fn main() {
    let args = Cli::parse();

    match fs::read_to_string(&args.path) {
        Ok(content) => {
            let (line_count, word_count, byte_count) = get_file_counts(content);
            let mut count_vec = Vec::<String>::new();
            if args.show_line_count {
                count_vec.push(line_count.to_string());
            }
            if args.show_word_count {
                count_vec.push(word_count.to_string());
            }
            if args.show_byte_count {
                count_vec.push(byte_count.to_string());
            }
            if count_vec.len() == 0 {
                count_vec.extend(vec![
                    line_count.to_string(),
                    word_count.to_string(),
                    byte_count.to_string(),
                ]);
            }

            let joined = count_vec.join(" ");
            println!("{} {:?}", joined, args.path);
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
