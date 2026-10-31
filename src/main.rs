use std::{fmt};

use clap::{Parser,ValueEnum};
use sha2::Digest;

#[derive(Parser, Debug)]
#[command(version)]
struct MyArgs {
    #[arg(short, long, default_value_t=Ver::Sha256)]
    ver: Ver,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
#[value(rename_all="kebab")]
enum Ver {
    #[value(alias="2")]
    Sha256,
    #[value(alias="5")]
    Sha512
}

impl fmt::Display for Ver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ver::Sha256 => write!(f, "sha256"),
            Ver::Sha512 => write!(f, "sha512"),
        }
    }
}

fn main() {
    let args = MyArgs::parse();
    
    let hex256 = sha2::Sha256::digest("data");
    let hex512 = sha2::Sha512::digest("data");
        
    let hash: String = match args.ver {
        Ver::Sha256 => hex256.iter().map(|b| format!("{:02x}", b)).collect(),
        Ver::Sha512 => hex512.into_iter().map(|b| format!("{:02x}", b)).collect(),
    };
    
    println!("{hash}");
}
