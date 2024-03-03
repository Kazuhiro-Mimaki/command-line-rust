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
    file: PathBuf,

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

    for entry in WalkDir::new(args.file) {
        match entry {
            Ok(entry) => {
                if args.entry_types.is_empty() {
                    println!("{}", entry.path().display());
                } else {
                    for entry_type in &args.entry_types {
                        if match entry_type {
                            EntryType::Dir => entry.file_type().is_dir(),
                            EntryType::File => entry.file_type().is_file(),
                            EntryType::Link => entry.file_type().is_symlink(),
                        } && (names.is_empty()
                            || names
                                .iter()
                                .any(|re| re.is_match(&entry.file_name().to_string_lossy())))
                        {
                            println!("{}", entry.path().display());
                        }
                    }
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use assert_cmd::Command;

    #[test]
    fn run_dir() -> Result<(), Box<dyn Error>> {
        let expected = fs::read_to_string("src/findr/expects/default.txt")?;
        Command::cargo_bin("findr")?
            .arg("src/findr/inputs")
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }
}
