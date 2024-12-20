# XOR Encrypt - A Simple XOR-Based Encryption Tool

**XOR Encrypt** is an open-source command-line application written in Rust for simple XOR-based encryption and decryption. It supports encrypting and decrypting text strings as well as files. 

## Features

- **Text Encryption and Decryption**: Encrypt or decrypt text directly from the command line.
- **File Encryption and Decryption**: Process files securely using a given key.
- **Cross-Platform**: Built with Rust, making it highly portable.
- **Lightweight**: A simple tool with no unnecessary dependencies.

## Installation

### Requirements

- Rust and Cargo installed on your system.
- A Linux-based operating system for the installation script.

### Install on Linux

Run the provided installation script:

```bash
./install_linux.sh
```

This will build the project and place the executable in `$HOME/.local/bin`.

### Adding to PATH

To make the `xor-encrypt` command accessible from anywhere in the terminal, add `$HOME/.local/bin` to your `PATH`. You can do this by adding the following line to your shell configuration file (e.g., `.bashrc`, `.zshrc`):

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Then, reload the shell configuration:

```bash
source ~/.bashrc  # or source ~/.zshrc for Zsh users
```

### Uninstall on Linux

To remove the application, run the uninstall script:

```bash
./uninstall_linux.sh
```

This will remove the installed binary from `$HOME/.local/bin`.

### Manual Installation

Alternatively, you can build the project manually using Cargo:

```bash
cargo build --release
```

The resulting binary will be located in `./target/release/xor-encrypt`.

## Usage

### Basic Syntax

```bash
xor-encrypt [OPTIONS] --operation <OPERATION> --key <KEY>
```

### Options

| Option                 | Description                                                   |
|------------------------|---------------------------------------------------------------|
| `-o, --operation`      | The operation to perform. Possible values: `encrypt`, `decrypt`, `encrypt-file`, `decrypt-file`. |
| `-t, --text`           | The text to encrypt or decrypt (for text operations).         |
| `-f, --file`           | The file to encrypt or decrypt (for file operations).         |
| `-k, --key`            | The encryption key (required).                                |
| `-h, --help`           | Prints help information.                                      |

### Examples

#### Encrypting a Text String

```bash
xor-encrypt -o encrypt -t "Hello, World!" -k mysecretkey
```

#### Decrypting a Text String

```bash
xor-encrypt -o decrypt -t "EncryptedString" -k mysecretkey
```

#### Encrypting a File

```bash
xor-encrypt -o encrypt-file -f myfile.txt -k mysecretkey
```

#### Decrypting a File

```bash
xor-encrypt -o decrypt-file -f myfile_encrypted.txt -k mysecretkey
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.

## License

This project is licensed under the [MIT License](LICENSE). 

---

If you encounter any issues or have questions, please check the repository's [issues section](https://github.com/DucktectiveCZ/xor-encrypt/issues) or contact the maintainers.

