use clap::builder::PossibleValue;
use clap::{Parser, ValueEnum};
use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Dir => Some("d".into()),
            Self::File => Some("f".into()),
            Self::Link => Some("l".into()),
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    files: Vec<PathBuf>,

    #[arg(short = 'n', long = "name", value_name = "NAME")]
    names: Vec<String>,

    #[arg(short = 't', long = "type", value_name = "TYPE")]
    entry_types: Vec<EntryType>,
}

fn main() {
    let args = Cli::parse();

    let names = args
        .names
        .iter()
        .map(|name| Regex::new(name).unwrap())
        .collect::<Vec<_>>();

    for file in args.files {
        for entry in WalkDir::new(file) {
            match entry {
                Ok(entry) => {
                    if (args.entry_types.is_empty()
                        || args.entry_types.iter().any(|entry_type| match entry_type {
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::File => entry.file_type().is_file(),
                            EntryType::Link => entry.file_type().is_symlink(),
                        }))
                        && (names.is_empty()
                            || names
                                .iter()
                                .any(|re| re.is_match(&entry.file_name().to_string_lossy())))
                    {
                        println!("{}", entry.path().display());
                    }
                }
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use assert_cmd::Command;

    #[test]
    fn run() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/default.txt")?;
        Command::cargo_bin("findr")?
            .arg("src/findr/inputs")
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn run_dir() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/dir.txt")?;
        Command::cargo_bin("findr")?
            .args(["src/findr/inputs", "-t", "d"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn run_file() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/file.txt")?;
        Command::cargo_bin("findr")?
            .args(["src/findr/inputs", "-t", "f"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn run_link() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/link.txt")?;
        Command::cargo_bin("findr")?
            .args(["src/findr/inputs", "-t", "l"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn run_multiple_type() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/multiple_type.txt")?;
        Command::cargo_bin("findr")?
            .args(["src/findr/inputs", "-t", "d", "-t", "f"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn run_names() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/name.txt")?;
        Command::cargo_bin("findr")?
            .args(["src/findr/inputs", "-n", ".*\\.csv"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn run_multiple_name() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/multiple_names.txt")?;
        Command::cargo_bin("findr")?
            .args(["src/findr/inputs", "-n", ".*\\.csv", "-n", ".*\\.mp3"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn run_multiple_input() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/multiple_inputs.txt")?;
        Command::cargo_bin("findr")?
            .args(["src/findr/inputs/a", "src/findr/inputs/d"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }
}
