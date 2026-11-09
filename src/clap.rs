use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser, Debug)]
#[command(version, about = "计算哈希摘要")]
pub struct MyArgs {
    /// Choose Hash Algorithm
    #[arg(short='a', long="algorithm", default_value_t = Algo::Sha256)]
    pub algo: Algo,

    /// The a file path you want to digest
    #[arg(short = 'i', long = "input")]
    pub input: Option<String>,

    #[arg(name = "source_text")]
    pub text: Option<String>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Algo {
    #[value(alias = "m")]
    Md5,
    #[value(alias = "2")]
    Sha256,
    #[value(alias = "5")]
    Sha512,
    #[value(alias = "1")]
    Sha1
}

impl fmt::Display for Algo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algo::Md5 => write!(f, "md5"),
            Algo::Sha256 => write!(f, "sha256"),
            Algo::Sha512 => write!(f, "sha512"),
            Algo::Sha1 => write!(f, "sha1"),
        }
    }
}
