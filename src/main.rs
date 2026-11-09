use clap::Parser;
use digest::clap::MyArgs;
use digest::hasher;

fn main() {
    let args = MyArgs::parse();

    let hash = if args.input == None {
        hasher::digest_text(&args.text.unwrap(), args.algo)
    } else {
        hasher::digest_file(&args.input.unwrap(), args.algo)
    };

    println!("{hash}");
}
