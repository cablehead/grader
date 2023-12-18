use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use std::thread;

use clap::Parser;

fn main() -> io::Result<()> {
    #[derive(Parser, Debug, Clone)]
    #[clap(author, version, about = "Binary sorter for text files. Lines are sorted into two bins based on child process response", long_about = None)]
    struct Args {
        #[clap(value_parser, help = "Path for output bin 1 (for echoed lines)")]
        bin1: PathBuf,

        #[clap(value_parser, help = "Path for output bin 2 (for non-echoed lines)")]
        bin2: PathBuf,

        #[clap(value_parser, help = "Command to execute for processing lines")]
        command: String,

        #[clap(value_parser, help = "Arguments for the command")]
        args: Vec<String>,
    }

    let args = Args::parse();

    let mut bin1 = BufWriter::new(File::create(&args.bin1)?);
    let mut bin2 = BufWriter::new(File::create(&args.bin2)?);

    let (sender, receiver) = mpsc::channel();

    let mut child = Command::new(&args.command)
        .args(&args.args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let mut child_stdin = child.stdin.take().expect("Failed to open stdin");
    let child_stdout = child.stdout.take().expect("Failed to open stdout");

    let writer_thread = thread::spawn(move || {
        let stdin = io::stdin();
        let handle = stdin.lock();

        for line in handle.lines() {
            let line = line.expect("Failed to read line");
            writeln!(child_stdin, "{}", line).expect("Failed to write to stdin");
            sender.send(line).expect("Failed to send line");
        }
    });

    let reader_thread = thread::spawn(move || {
        let reader = io::BufReader::new(child_stdout);

        for line in reader.lines() {
            let line = line.expect("Failed to read line from child stdout");

            while let Ok(sent_line) = receiver.try_recv() {
                if sent_line == line {
                    writeln!(bin1, "{}", sent_line).expect("Failed to write to bin1");
                    break;
                } else {
                    writeln!(bin2, "{}", sent_line).expect("Failed to write to bin2");
                }
            }
        }
    });

    writer_thread.join().expect("Writer thread panicked");
    reader_thread.join().expect("Reader thread panicked");

    Ok(())
}
