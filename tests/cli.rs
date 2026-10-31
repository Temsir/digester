//! `digest` 二进制的集成测试：通过真实进程验证命令行行为和输出。

use std::process::{Command, Output};

/// Cargo 只在集成测试里注入这个环境变量，指向编译好的二进制。
const BIN: &str = env!("CARGO_BIN_EXE_digest");

const SHA256_STRING: &str = "473287f8298dba7163a897908958f7c0eae733e25d2e027992ea2edc9bed2fa8";
const SHA512_STRING: &str = "2757cb3cafc39af451abb2697be79b4ab61d63d74d85b0418629de8c26811b529f3f3780d0150063ff55a2beee74c4ec102a2a2731a1f1f7f10d473ad18a6a87";
const SHA256_HELLO_WORLD: &str = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
const SHA256_EMPTY: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

fn run(args: &[&str]) -> Output {
    Command::new(BIN)
        .args(args)
        .output()
        .expect("无法启动 digest 二进制")
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
fn explicit_sha256() {
    let out = run(&["-v", "sha256", "string"]);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA256_STRING);
}

#[test]
fn default_algorithm_is_sha256() {
    let out = run(&["string"]);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA256_STRING);
}

#[test]
fn explicit_sha512_uses_long_flag() {
    let out = run(&["--ver", "sha512", "string"]);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA512_STRING);
}

#[test]
fn argument_with_spaces_is_one_value() {
    // 传进来的 "hello world" 必须整体参与哈希，而不是被拆成两个参数。
    let out = run(&["-v", "sha256", "hello world"]);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA256_HELLO_WORLD);
}

#[test]
fn utf8_argument_is_hashed_as_utf8_bytes() {
    let out = run(&["-v", "sha256", "中文"]);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(
        stdout(&out),
        "72726d8818f693066ceb69afa364218b692e62ea92b385782363780f47529c21"
    );
}

#[test]
fn empty_string_is_valid_input() {
    let out = run(&["-v", "sha256", ""]);

    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_eq!(stdout(&out), SHA256_EMPTY);
}


#[test]
fn unknown_algorithm_is_rejected() {
    let out = run(&["-v", "md5", "string"]);

    assert_eq!(out.status.code(), Some(2));
    let err = stderr(&out);
    assert!(
        err.contains("invalid value 'md5'"),
        "stderr 应指出取值非法，实际: {err}"
    );
}
