use matrix_sdk_crypto::{decrypt_room_key_export, encrypt_room_key_export};
use rpassword::read_password;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        eprintln!("Usage: KEY_FILE ROOM_ID [ROOM_ID...]");
        exit(1);
    }

    let (input_path, rooms) = args.split_first().unwrap();

    println!("Filtering room keys: {:?}", rooms);

    println!("Enter passphrase:");
    let passphrase = read_password().expect("Failed to read input.");

    let mut input = File::open(input_path).expect("Failed to read file.");

    let decrypted =
        decrypt_room_key_export(&mut input, &passphrase).expect("Failed to decrypt file.");

    let total = decrypted.len();
    let mut selected = Vec::new();

    for key in decrypted {
        if rooms.contains(&key.room_id.to_string()) {
            selected.push(key);
        }
    }

    let encrypted =
        encrypt_room_key_export(&selected, &passphrase, 10_000).expect("Failed to encrypt output.");

    let input_path = Path::new(input_path);
    let output_path = input_path.parent().unwrap().join(format!(
        "filtered-{}",
        input_path.file_name().unwrap().to_string_lossy()
    ));

    let mut output = File::create(&output_path).expect("Failed to create output file.");
    write!(output, "{}", encrypted).expect("Failed to create output file.");

    println!(
        "Successfully wrote {} out of {} keys to {}",
        selected.len(),
        total,
        output_path.display()
    );
}
