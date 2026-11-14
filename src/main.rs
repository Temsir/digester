use std::process::ExitCode;
use clap::Parser;
use digest::clap::{MyArgs, print_help};
use digest::hasher;

fn main() -> ExitCode {
    let args = MyArgs::parse();

    let hash = if args.file_path == None {
        let text = match args.text {
            Some(t) => t,
            None => {
                print_help();
                return ExitCode::FAILURE;
            }
        };

        hasher::digest_text(&text, args.algo)
    } else {
        let file_path = args.file_path.unwrap();
        // Clap will do the error handling and print corresponding help message. So `args.file_path.unwrap()` is fine.
        match hasher::digest_file(&file_path, args.algo) {
            Ok(hash) => hash,
            Err(e) => {
                eprintln!("Unable to open or read file: \"{}\", due to: {}", file_path, e);
                return ExitCode::FAILURE;
            }
        }
    };

    println!("{hash}");
    ExitCode::SUCCESS
}
