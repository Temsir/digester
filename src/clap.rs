use clap::{Parser, ValueEnum};
use std::fmt;

#[derive(Parser, Debug)]
#[command(version, about = "计算哈希摘要")]
pub struct MyArgs {
    /// Choose Hash Algorithm
    #[arg(short='v', long="ver", default_value_t = Ver::Sha256)]
    pub ver: Ver,

    /// The a file path you want to digest
    #[arg(short = 'i', long = "input")]
    pub input: Option<String>,

    #[arg(name = "source_text")]
    pub text: Option<String>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum Ver {
    #[value(alias = "m")]
    Md5,
    #[value(alias = "2")]
    Sha256,
    #[value(alias = "5")]
    Sha512,
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
