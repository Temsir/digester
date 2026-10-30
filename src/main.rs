use clap::Parser;
use sha2::{Digest, Sha256, Sha512};

#[derive(Parser, Debug)]
#[command(version)]
struct MyArgs {
    #[arg(short, long)]
    ver: String,
}

fn main() {
    let args = MyArgs::parse();
    
    let hash: String = if args.ver == "sha256" || args.ver == "2" {
        let temp_hex = Sha256::digest("data");
        temp_hex.iter().map(|b| format!("{:02x}", b)).collect()
    } else if args.ver == "sha512" || args.ver == "5" {
        let temp_hex = Sha512::digest("data");
        temp_hex.iter().map(|b| format!("{:02x}", b)).collect()
    } else {
        "None".to_string()
    };
    
    println!("{hash}");
}
