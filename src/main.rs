use std::fs::{self, File};
use std::io::{self, Read, Write};
use rand::Rng;
use std::env;
use std::process;

const KEY_LENGTH: usize = 32; // Adjust key length as needed

fn generate_key() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key = vec![0u8; KEY_LENGTH];
    rng.fill(&mut key[..]);
    key
}

fn encrypt_data(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, byte)| byte ^ key[i % key.len()])
        .collect()
}

fn decrypt_data(data: &[u8], key: &[u8]) -> Vec<u8> {
    encrypt_data(data, key)
}

fn encrypt_file(input_path: &str) -> io::Result<()> {
    let key = generate_key();
    let mut file = File::open(input_path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let encrypted_data = encrypt_data(&data, &key);

    let output_path = format!("{}.encrypted", input_path);
    let mut output_file = File::create(&output_path)?;

    // Save the key at the beginning of the output file
    output_file.write_all(&key)?;
    output_file.write_all(&encrypted_data)?;

    println!("File '{}' encrypted and saved as '{}'", input_path, output_path);

    Ok(())
}

fn decrypt_file(input_path: &str) -> io::Result<()> {
    let mut file = File::open(input_path)?;
    let mut key = vec![0u8; KEY_LENGTH];
    
    // Read the first 32 bytes to get the key
    file.read_exact(&mut key)?;
    
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let decrypted_data = decrypt_data(&data, &key);

    let output_path = format!("{}.decrypted", input_path);
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(&decrypted_data)?;

    println!("File '{}' decrypted and saved as '{}'", input_path, output_path);

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} [encrypt|decrypt] [directory]", args[0]);
        process::exit(1);
    }

    let operation = &args[1];
    let input_directory = &args[2];

    if operation == "encrypt" {
        for entry in fs::read_dir(input_directory)? {
            let entry = entry?;
            let input_path = entry.path();

            if input_path.is_file() {
                encrypt_file(input_path.to_str().unwrap())?;
            }
        }
    } else if operation == "decrypt" {
        for entry in fs::read_dir(input_directory)? {
            let entry = entry?;
            let input_path = entry.path();

            if input_path.is_file() && input_path.extension().unwrap_or_default() == "encrypted" {
                decrypt_file(input_path.to_str().unwrap())?;
            }
        }
    } else {
        eprintln!("Invalid operation. Use 'encrypt' or 'decrypt'.");
        process::exit(1);
    }

    Ok(())
}
