use base64::{engine::general_purpose::STANDARD, Engine as _};
use clap::Parser;
use std::process::exit;

#[derive(Debug)]
#[derive(Parser)]
struct Args {
  operation: String,
  text: String,
  key: String,
}

impl std::fmt::Display for Args {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "operation: '{}', text: '{}', key: '{}'", self.operation, self.text, self.key)
  }
}

fn xor_encrypt(key: &str, plaintext: &str) -> Vec<u8> {
    let key_bytes = key.as_bytes();
    let plaintext_bytes = plaintext.as_bytes();
    let key_len = key_bytes.len();

    plaintext_bytes
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_len])
        .collect()
}

fn xor_decrypt(key: &str, ciphertext: &[u8]) -> String {
    let key_bytes = key.as_bytes();
    let key_len = key_bytes.len();

    ciphertext
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ key_bytes[i % key_len])
        .map(|byte| byte as char)
        .collect()
}

fn encrypt(args: &Args) -> String {
  let xor_encrypted = xor_encrypt(args.key.as_str(), args.text.as_str());
  STANDARD.encode(xor_encrypted)
}
fn decrypt(args: &Args) -> String {
  let base64_decoded = STANDARD.decode(&args.text).expect("Error decoding base64 string");
  xor_decrypt(args.key.as_str(), &base64_decoded)
}

fn main() {
  let args = Args::parse();
  
  if args.operation.to_lowercase() == "encrypt" {
    let encrypted = encrypt(&args);
    println!("Encrypted: '{}'", encrypted);
  } else if args.operation.to_lowercase() == "decrypt" {
    let decrypted = decrypt(&args);
    println!("Decrypted: '{}'", decrypted);
  } else {
    eprintln!("Invalid operation: '{}'", args.operation);
    exit(1);
  }
}
