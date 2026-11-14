use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_digest");

const HELLO_WORLD: &str = "hello world";
const MD5_HELLO_WORLD: &str = "5eb63bbbe01eeed093cb22bb8f5acdc3";
const SHA512_HELLO_WORLD: &str = "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f";
const SHA256_HELLO_WORLD: &str = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
const SHA1_HELLO_WORLD: &str = "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed";
const SHA256_EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";



fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .output()
        .expect("无法启动 digest.exe")
}

/// 去掉 `println!` 在 Windows 上可能带来的行尾差异。
fn stdout(out: &Output) -> String {
    let text = String::from_utf8(out.stdout.clone()).expect("stdout 不是合法 UTF-8");
    text.trim_end_matches(['\r', '\n']).to_owned()
}

fn stderr(out: &Output) -> String {
    let text = String::from_utf8_lossy(&out.stderr);
    text.trim_end_matches(['\r', '\n']).to_owned()
}

#[test]
fn implicit_sha256_text() {
    let out = run(&[HELLO_WORLD]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA256_HELLO_WORLD);
}

#[test]
fn short_flag_md5_text() {
    let out = run(&["-a", "md5", HELLO_WORLD]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), MD5_HELLO_WORLD);
}

#[test]
fn short_flag_sha256_text() {
    let out = run(&["-a", "sha256", HELLO_WORLD]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA256_HELLO_WORLD);
}

#[test]
fn long_flag_sha512_text() {
    let out = run(&["--algorithm", "sha512", HELLO_WORLD]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA512_HELLO_WORLD);
}

#[test]
fn long_flag_sha1_text() {
    let out = run(&["--algorithm", "sha1", HELLO_WORLD]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA1_HELLO_WORLD);
}

#[test]
fn short_flag_sha256_file() {
    let out = run(&["-a", "sha256", "-f", "./assets/winrar-setup.exe"]);
    assert!(!stdout(&out).is_empty())
}

#[test]
fn utf8_text() {
    let out = run(&["-a", "sha256", "中文"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(
        stdout(&out),
        "72726d8818f693066ceb69afa364218b692e62ea92b385782363780f47529c21"
    );
}

#[test]
fn empty_string_text() {
    let out = run(&["-a", "sha256", ""]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA256_EMPTY);
}
