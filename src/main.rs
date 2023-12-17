use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use clap::Parser;

fn main() -> io::Result<()> {
    #[derive(Parser, Debug, Clone)]
    #[clap(author, version, about, long_about = None)]
    struct Args {
        #[clap(value_parser)]
        bin1: PathBuf,

        #[clap(value_parser)]
        bin2: PathBuf,

        #[clap(value_parser)]
        command: String,
        #[clap(value_parser)]
        args: Vec<String>,
    }

    let args = Args::parse();

    let mut bin1 = File::create(&args.bin1)?;
    let mut bin2 = File::create(&args.bin2)?;

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = line?;

        let mut child = Command::new(&args.command)
            .args(&args.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            writeln!(stdin, "{}", line)?;
        }

        let exit_status = child.wait()?;

        // Write to the appropriate file descriptor based on exit status
        if exit_status.success() {
            writeln!(bin1, "{}", line)?;
        } else {
            writeln!(bin2, "{}", line)?;
        }
    }

    Ok(())
}
