use clap::{Parser, ValueEnum};
use md5;
use sha2::{Digest, Sha256, Sha512};
use std::{
    fmt::{self},
    fs::File,
    io::{ErrorKind::NotFound, Read},
    vec,
};

#[derive(Parser, Debug)]
#[command(version, about = "计算哈希摘要")]
struct MyArgs {
    /// Choose Hash Algorithm
    #[arg(short='v', long="ver", default_value_t = Ver::Sha256)]
    ver: Ver,

    /// The a file path you want to digest
    #[arg(short = 'i', long = "input")]
    input: Option<String>,

    #[arg(name = "source_text")]
    text: Option<String>,
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
    #[value(alias = "m")]
    Md5,
    #[value(alias = "2")]
    Sha256,
    #[value(alias = "5")]
    Sha512,
}

enum Hasher {
    Md5(md5::Context),
    Sha256(sha2::Sha256),
    Sha512(sha2::Sha512),
}

fn main() {
    let args = MyArgs::parse();

    let hash = if args.input == None {
        match args.ver {
            Ver::Sha256 => hexify(&Sha256::digest(args.text.unwrap().as_bytes())),
            Ver::Sha512 => hexify(&Sha512::digest(args.text.unwrap().as_bytes())),
            Ver::Md5 => format!("{:02x}", md5::compute(args.text.unwrap())),
        }
    } else {
        let file_path = args.input.unwrap();
        digest_file(&file_path, args.ver)
    };

    println!("{hash}");
}

fn hexify(bytes: &[u8]) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

fn digest_file(file_path: &str, ver: Ver) -> String {
    let mut buf = vec![0u8; 16 * 1024];
    let mut handle = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                NotFound => eprintln!("File Not Found: \"{}\"", &file_path),
                _ => eprintln!("Unable to open file \"{}\", due to: {}", &file_path, e),
            }
            std::process::exit(1)
        }
    };

    let mut hasher = match ver {
        Ver::Sha256 => Hasher::Sha256(sha2::Sha256::new()),
        Ver::Sha512 => Hasher::Sha512(sha2::Sha512::new()),
        Ver::Md5 => Hasher::Md5(md5::Context::new()),
    };

    println!("Digesting file. May take a while...");

    loop {
        let n = handle.read(&mut buf).expect("fail to read");

        if n == 0 {
            break;
        }

        match &mut hasher {
            Hasher::Sha256(h) => h.update(&buf[0..n]),
            Hasher::Sha512(h) => h.update(&buf[0..n]),
            Hasher::Md5(ctx) => ctx.consume(&buf[0..n]),
        }
    }

    match hasher {
        Hasher::Sha256(h) => hexify(&h.finalize()),
        Hasher::Sha512(h) => hexify(&h.finalize()),
        Hasher::Md5(ctx) => hexify(&ctx.finalize().0),
    }
}
