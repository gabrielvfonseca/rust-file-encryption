# File Encryption and Decryption Tool

This is a simple Rust command-line tool for encrypting and decrypting files using a custom encryption algorithm. The program allows you to encrypt files within a directory and later decrypt them without needing to provide the encryption key manually, as the key is stored within the encrypted files themselves.

## Features

- Encrypt files within a directory.
- Decrypt previously encrypted files using the stored encryption key.

## Prerequisites

Before running this tool, ensure you have the following installed on your system:

- Rust (including Cargo)

## How It Works

The program operates in two modes: **encryption** and **decryption**.

### Encryption

1. When you run the program in encryption mode, it first generates a random encryption key of 32 bytes in length.

2. It then reads each file within the specified directory and encrypts the file's content using a simple XOR operation with the generated key. The encrypted data is saved with the same filename but with the ".encrypted" extension.

3. The program also stores the encryption key at the beginning of each encrypted file.

### Decryption

1. When you run the program in decryption mode, it searches for files within the specified directory that have the ".encrypted" extension.

2. For each encrypted file found, the program reads the encryption key from the beginning of the file.

3. Using the extracted encryption key, the program decrypts the file's content, reversing the XOR operation performed during encryption.

4. The decrypted data is saved with the same filename but with the ".decrypted" extension.

## Usage

1. Clone or download this repository to your local machine.

2. Open your terminal and navigate to the project directory.

3. Compile the Rust code using Cargo:

   ```bash
   cargo build --release
Run the program with the following command:
bash
Copy code
cargo run -- [operation] [directory]
[operation] can be either "encrypt" or "decrypt".
[directory] is the path to the directory containing the files you want to process.
Follow the on-screen prompts:
For encryption, the program will encrypt the files and save them with the ".encrypted" extension.
For decryption, the program will search for ".encrypted" files and decrypt them using the stored encryption key. The decrypted files will be saved with the ".decrypted" extension.
Example Usage

To encrypt files in the "my_files" directory:
bash
Copy code
cargo run -- encrypt my_files
To decrypt files in the "my_files" directory:
bash
Copy code
cargo run -- decrypt my_files
License

### License

This project is licensed under the MIT License - see the LICENSE file for details.