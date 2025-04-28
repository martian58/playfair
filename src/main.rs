use clap::{Arg, Command};

/// Enum to represent encryption or decryption mode
#[derive(Debug, Copy, Clone)]
enum CipherMode {
    Encrypt,
    Decrypt,
}

/// Generates a Playfair encryption table based on the provided key.
///
/// # Arguments
///
/// * `key` - A string slice that holds the encryption key.
///
/// # Returns
///
/// * A 5x5 vector containing the Playfair encryption table.
fn generate_playfair_table(key: &str) -> Vec<Vec<char>> {
    // Keeps track of characters already added to the table
    let mut seen: Vec<bool> = vec![false; 26];
    // The 5x5 table that will be generated
    let mut table: Vec<Vec<char>> = Vec::new();
    // Current row being filled in the table
    let mut row: Vec<char> = Vec::new();

    for c in key.chars().chain('A'..='Z') {
        // Convert 'J' to 'I' and ensure all characters are uppercase
        let c = if c == 'J' { 'I' } else { c.to_ascii_uppercase() };
        // Add the character to the table if it hasn't been added already and is alphabetic
        if c.is_ascii_alphabetic() && !seen[(c as u8 - b'A') as usize] {
            seen[(c as u8 - b'A') as usize] = true;
            row.push(c);
            if row.len() == 5 {
                table.push(row);
                row = Vec::new();
            }
        }
    }
    table
}

/// Finds the position of a character in the Playfair table.
///
/// # Arguments
///
/// * `table` - The Playfair encryption table.
/// * `c` - The character to find.
///
/// # Returns
///
/// * A tuple `(row, col)` representing the position of the character in the table.
fn find_position(table: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for (i, row) in table.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == c) {
            return (i, j);
        }
    }
    panic!("Character not found in table");
}

/// Encrypts or decrypts a text using the Playfair cipher.
///
/// # Arguments
///
/// * `text` - The input text to encrypt or decrypt.
/// * `table` - The Playfair encryption table.
/// * `mode` - The encryption or decryption mode.
///
/// # Returns
///
/// * The encrypted or decrypted text.
fn playfair_cipher(text: &str, table: &Vec<Vec<char>>, mode: CipherMode) -> String {
    let mut result: String = String::new();
    // Filter alphabetic characters and convert them to uppercase
    let mut chars: Vec<char> = text
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect();

    // Insert 'X' between repeated characters in a pair
    let mut i: usize = 0;
    while i < chars.len() {
        if i + 1 < chars.len() && chars[i] == chars[i + 1] {
            chars.insert(i + 1, 'X');
        }
        i += 2;
    }

    // If the text length is odd, make it even by appending 'X'
    if chars.len() % 2 != 0 {
        chars.push('X');
    }

    // Process pairs of characters
    for chunk in chars.chunks(2) {
        let (r1, c1) = find_position(table, chunk[0]);
        let (r2, c2) = find_position(table, chunk[1]);

        match mode {
            CipherMode::Encrypt => {
                if r1 == r2 {
                    // Same row: shift columns to the right
                    result.push(table[r1][(c1 + 1) % 5]);
                    result.push(table[r2][(c2 + 1) % 5]);
                } else if c1 == c2 {
                    // Same column: shift rows down
                    result.push(table[(r1 + 1) % 5][c1]);
                    result.push(table[(r2 + 1) % 5][c2]);
                } else {
                    // Rectangle swap
                    result.push(table[r1][c2]);
                    result.push(table[r2][c1]);
                }
            }
            CipherMode::Decrypt => {
                if r1 == r2 {
                    // Same row: shift columns to the left
                    result.push(table[r1][(c1 + 4) % 5]);
                    result.push(table[r2][(c2 + 4) % 5]);
                } else if c1 == c2 {
                    // Same column: shift rows up
                    result.push(table[(r1 + 4) % 5][c1]);
                    result.push(table[(r2 + 4) % 5][c2]);
                } else {
                    // Rectangle swap
                    result.push(table[r1][c2]);
                    result.push(table[r2][c1]);
                }
            }
        }
    }

    result
}

fn main() {
    // Parse command-line arguments
    let matches = Command::new("Playfair")
        .bin_name("playfair")
        .version("1.0")
        .author("martian58")
        .about("Encrypts or decrypts text using the Playfair cipher")
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .value_name("KEY")
                .help("Sets the encryption/decryption key")
                .required(true),
        )
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("TEXT")
                .help("The text to encrypt or decrypt")
                .required(true),
        )
        .arg(
            Arg::new("decrypt")
                .short('d')
                .long("decrypt")
                .help("Decrypt the input text instead of encrypting")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let key: &String = matches.get_one::<String>("key").unwrap();
    let input_text: &String = matches.get_one::<String>("input").unwrap();
    let decrypt: bool = matches.get_flag("decrypt");

    // Generate the Playfair table
    let table: Vec<Vec<char>> = generate_playfair_table(key);
    println!("Generated Playfair Table:");
    for row in &table {
        println!("{:?}", row);
    }

    // Determine the mode
    let mode: CipherMode = if decrypt {
        CipherMode::Decrypt
    } else {
        CipherMode::Encrypt
    };

    // Encrypt or decrypt the text
    let result: String = playfair_cipher(input_text, &table, mode);

    match mode {
        CipherMode::Encrypt => println!("Encrypted Text: {}", result),
        CipherMode::Decrypt => println!("Decrypted Text: {}", result),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_playfair_table() {
        let key: &str = "KEYWORD";
        let table: Vec<Vec<char>> = generate_playfair_table(key);
        assert_eq!(table.len(), 5);
        assert_eq!(table[0].len(), 5);
        assert!(table.iter().flatten().all(|&c| c != 'J'));
    }

    #[test]
    fn test_playfair_encrypt() {
        let key: &str = "KEYWORD";
        let table: Vec<Vec<char>> = generate_playfair_table(key);
        let plaintext: &str = "HELLO";
        let encrypted: String = playfair_cipher(plaintext, &table, CipherMode::Encrypt);
        assert_eq!(encrypted, "GYIZSC");
    }

    #[test]
    fn test_playfair_decrypt() {
        let key: &str = "KEYWORD";
        let table: Vec<Vec<char>> = generate_playfair_table(key);
        let encrypted: &str = "GYIZSC";
        let decrypted: String = playfair_cipher(encrypted, &table, CipherMode::Decrypt);
        assert_eq!(decrypted, "HELXLO");
    }

    #[test]
    fn test_playfair_with_repeated_characters() {
        let key: &str = "KEYWORD";
        let table: Vec<Vec<char>> = generate_playfair_table(key);
        let plaintext: &str = "BALLOON";
        let encrypted: String = playfair_cipher(plaintext, &table, CipherMode::Encrypt);
        assert_eq!(encrypted, "CBIZSCES");
    }

    #[test]
    fn test_playfair_with_odd_length() {
        let key: &str = "KEYWORD";
        let table: Vec<Vec<char>> = generate_playfair_table(key);
        let plaintext: &str = "TEST";
        let encrypted: String = playfair_cipher(plaintext, &table, CipherMode::Encrypt);
        assert!(encrypted.len() % 2 == 0);
    }
}