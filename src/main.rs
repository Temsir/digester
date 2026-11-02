use std::fmt::{self, write};
use clap::{Parser, ValueEnum};
use md5;
use sha2::{Digest, Sha256, Sha512};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about = "计算哈希摘要")]
struct MyArgs {
    /// Choose Hash Algorithm
    #[arg(short='v', long="ver", default_value_t = Ver::Sha256)]
    ver: Ver,

    /// The a file path you want to digest
    #[arg(short='i', long="input")]
    input: Option<String>,
    
    #[arg(name="source_text")]
    text: String,
}

impl fmt::Display for Ver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ver::Md5 => write!(f, "md5"),
            Ver::Sha256 => write!(f, "sha256"),
            Ver::Sha512 => write!(f, "sha512"),
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Ver {
    Md5,
    Sha256,
    Sha512,
}

fn hexify(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    
    let args = MyArgs::parse();

    let hash = match args.ver {
        Ver::Sha256 => hexify(Sha256::digest(args.text.as_bytes())),
        Ver::Sha512 => hexify(Sha512::digest(args.text.as_bytes())),
        Ver::Md5 => format!("{:02x}", md5::compute(args.text)),
    };

    println!("{hash}");

}
