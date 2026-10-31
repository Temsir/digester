use std::fmt;
use clap::{Parser, ValueEnum};
use sha2::{Digest, Sha256, Sha512};

#[derive(Parser, Debug)]
#[command(version, about = "计算文本的哈希摘要")]
struct MyArgs {
    /// Choose Hash Algorithm
    #[arg(short='v', long="ver", default_value_t = Ver::Sha256)]
    ver: Ver,

    #[arg(name="source_text")]
    data: String,
}

impl fmt::Display for Ver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ver::Sha256 => write!(f, "sha256"),
            Ver::Sha512 => write!(f, "sha512"),
        }
    }
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum Ver {
    Sha256,
    Sha512,
}

fn hexify(bytes: impl AsRef<[u8]>) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

fn main() {
    let args = MyArgs::parse();

    let hash = match args.ver {
        Ver::Sha256 => hexify(Sha256::digest(args.data.as_bytes())),
        Ver::Sha512 => hexify(Sha512::digest(args.data.as_bytes())),
    };

    println!("{hash}");
}
