
// MIT License
// Copyright (c) 2024 Pepa
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// provided to do so, subject to the following condition:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use std::fs;
use base64::Engine as _;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Operation {
  Encrypt,
  Decrypt,
  EncryptFile,
  DecryptFile,
}

#[derive(Parser, Debug)]
struct Args {
  #[clap(short, long)]
  operation: Operation,

  #[clap(short, long, required_if_eq("operation", "Encrypt"), required_if_eq("operation", "Decrypt"))]
  text: Option<String>,

  #[clap(short, long, required_if_eq("operation", "EncryptFile"), required_if_eq("operation", "DecryptFile"))]
  file: Option<String>,

  #[clap(short, long)]
  key: String,
}

impl Args {
  fn validate(&self) -> Result<(), String> {
    match self.operation {
      Operation::Encrypt | Operation::Decrypt => {
        if self.file.is_some() {
          return Err("The `file` argument cannot be used with `encrypt` or `decrypt` operations.".to_string());
        }
        if self.text.is_none() {
          return Err("The `text` argument is required for `encrypt` or `decrypt` operations.".to_string());
        }
      }
      Operation::EncryptFile | Operation::DecryptFile => {
        if self.text.is_some() {
          return Err("The `text` argument cannot be used with `encrypt-file` or `decrypt-file` operations.".to_string());
        }
        if self.file.is_none() {
          return Err("The `file` argument is required for `encrypt-file` or `decrypt-file` operations.".to_string());
        }
      }
    }
    Ok(())
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

fn operation_encrypt(args: &Args) -> Result<(), String> {
  let encrypted = xor_encrypt(args.key.as_str(), args.text.clone().unwrap().as_str());
  let base64_encrypted = base64::engine::general_purpose::STANDARD.encode(&encrypted);
  println!("{}", base64_encrypted);
  Ok(())
}
fn operation_decrypt(args: &Args) -> Result<(), String> {
  let base64_decoded;
  match base64::engine::general_purpose::STANDARD.decode(args.text.clone().unwrap().as_str()) {
    Ok(val) => base64_decoded = val,
    Err(e) => return Err(e.to_string()),
  }
  let decrypted = xor_decrypt(args.key.as_str(), &base64_decoded);
  println!("{}", decrypted);
  Ok(())
}
fn operation_encrypt_file(args: &Args) -> Result<(), String> {
  let file_contents: String;
  match fs::read_to_string(args.file.clone().unwrap()) {
    Ok(val) => file_contents = val,
    Err(e) => return Err(format!("File error: {}", e.to_string())),
  }
  let encrypted = xor_encrypt(args.key.as_str(), &file_contents);
  let base64_encoded = base64::engine::general_purpose::STANDARD.encode(&encrypted);
  println!("{}", base64_encoded);
  Ok(())
}
fn operation_decrypt_file(args: &Args) -> Result<(), String> {
  let mut file_contents: String;
  match fs::read_to_string(args.file.clone().unwrap().as_str()) {
    Ok(val) => file_contents = val,
    Err(e) => return Err(format!("File error: {}", e.to_string())),
  }
  file_contents.remove(file_contents.len() - 1); // Remove the '\n' at the end
  let base64_decoded: Vec<u8>;
  match base64::engine::general_purpose::STANDARD.decode(&file_contents) {
    Ok(val) => base64_decoded = val,
    Err(e) => return Err(format!("Base64 error: {}", e.to_string())),
  }
  let mut decrypted = xor_decrypt(args.key.as_str(), &base64_decoded);
  decrypted.remove(decrypted.len() - 1); // Remove the '\n' at the end
  println!("{}", decrypted);
  Ok(())
}

fn main() {
  let args = Args::parse();

  if let Err(e) = args.validate() {
    eprintln!("Error: {}", e);
    std::process::exit(1);
  }
  
  match args.operation {
    Operation::Encrypt => 
      match operation_encrypt(&args) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
      },
    Operation::Decrypt => 
      match operation_decrypt(&args) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
      },
    Operation::EncryptFile =>
      match operation_encrypt_file(&args) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
      },
    Operation::DecryptFile =>
      match operation_decrypt_file(&args) {
        Ok(_) => (),
        Err(e) => eprintln!("Error; {}", e),
      },
  }
}

