use crate::clap::Ver;
use sha2::{Digest, Sha256, Sha512};
use sha1::Sha1;
use std::fs::File;
use std::io::ErrorKind::NotFound;
use std::io::Read;

enum Hasher {
    Md5(md5::Context),
    Sha256(sha2::Sha256),
    Sha512(sha2::Sha512),
    Sha1(sha1::Sha1),
}

pub fn hexify(bytes: &[u8]) -> String {
    bytes.as_ref().iter().map(|b| format!("{b:02x}")).collect()
}

pub fn digest_file(file_path: &str, ver: Ver) -> String {
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
        Ver::Sha1 => Hasher::Sha1(sha1::Sha1::new()),
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
            Hasher::Sha1(h) => h.update(&buf[0..n]),
        }
    }

    match hasher {
        Hasher::Sha256(h) => hexify(&h.finalize()),
        Hasher::Sha512(h) => hexify(&h.finalize()),
        Hasher::Md5(ctx) => hexify(&ctx.finalize().0),
        Hasher::Sha1(h) => hexify(&h.finalize()),
    }
}

pub fn digest_text(source_text: &str, ver: Ver) -> String {
    match ver {
        Ver::Sha256 => hexify(&Sha256::digest(source_text.as_bytes())),
        Ver::Sha512 => hexify(&Sha512::digest(source_text.as_bytes())),
        Ver::Md5 => format!("{:02x}", md5::compute(source_text)),
        Ver::Sha1 => hexify(&Sha1::digest(source_text.as_bytes())),
    }
}
