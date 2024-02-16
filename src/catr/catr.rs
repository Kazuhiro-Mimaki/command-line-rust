use clap::Parser;
use std::{fs, path::PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(value_name = "PATH", help = "filepath to read from")]
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = fs::read_to_string(&args.path);

    match content {
        Ok(content) => println!("{:?}", content),
        Err(e) => panic!("There was a problem opening the file: {:?}", e),
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn run_catr() {
        let mut cmd = Command::cargo_bin("catr").unwrap();
        cmd.arg("src/catr/test.txt")
            .assert()
            .success()
            .stdout("\"catr in file!\\ncatr in file!!\"\n");
    }
}
