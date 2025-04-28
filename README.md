# Playfair

This is a Rust implementation of the **Playfair Cipher**

## Theory of the Playfair Cipher

The Playfair Cipher is a **digraph substitution cipher**, meaning it encrypts pairs of letters (digraphs) instead of single letters. Here's how it works:

1. **Encryption Table**:
   - A 5x5 matrix is generated using a keyword.
   - The keyword is entered into the matrix, skipping duplicate letters and treating `J` as `I`.
   - The remaining letters of the alphabet are added to fill the table.

2. **Encryption Rules**:
   - The plaintext is divided into pairs of letters.
   - If a pair contains the same letter (e.g., "LL"), an `X` is inserted between them.
   - If the plaintext length is odd, an `X` is appended to the end.
   - Each pair is encrypted based on their positions in the matrix:
     - **Same Row**: Replace each letter with the one to its right (wrapping around to the first column if necessary).
     - **Same Column**: Replace each letter with the one below it (wrapping around to the top row if necessary).
     - **Rectangle Rule**: Swap the column positions of the letters.

3. **Decryption Rules**:
   - Decryption follows the same logic as encryption but shifts positions in the opposite direction:
     - **Same Row**: Replace each letter with the one to its left.
     - **Same Column**: Replace each letter with the one above it.
     - **Rectangle Rule**: Swap the column positions of the letters.

4. **Case Insensitivity**:
   - The cipher is not case-sensitive. All input is converted to uppercase, and spaces or non-alphabetic characters are ignored.

5. **Decryption Output**:
   - Decrypted text may include `X` between repeated characters or at the end of a message, which were added during encryption for padding or disambiguation.

---

## How the Code Works

1. **Encryption Table Generation**:
   - The `generate_playfair_table` function creates the 5x5 matrix using the given key.
   - Duplicate letters and `J` are handled appropriately.

2. **Message Encryption/Decryption**:
   - The `playfair_cipher` function processes the text according to the rules of the Playfair Cipher.
   - It handles repeated characters and odd-length messages by inserting `X` as needed.

---

## How to Use

### Compiling the Code

To compile the code, ensure you have the Rust toolchain installed. Then, run:

```bash
cargo build --release
```

This will generate the executable in the `target/release` directory.

Alternatively, you can use the precompiled binaries:
- **Linux**: `playfair-x86_64-linux`
- **Windows**: `playfair-win.exe`

---

### Running the Program

You can run the program witj the precomiled binaries:

```bash
./playfair-x86_64-linux --key KEYWORD --input TEXT [--decrypt]
```
or if you compiled with `cargo build --release`
```bash
./target/release/playfair --key KEYWORD --input TEXT [--decrypt]
```

For Windows, use:

```bash
playfair-win.exe --key KEYWORD --input TEXT [--decrypt]
```

#### Example Commands:

1. **Encrypt a Message**:
   ```bash
   ./playfair-x86_64-linux -k KEYWORD -i "HELLO WORLD"
   ```

   **Output**:
   ```
    Generated Playfair Table:
    ['K', 'E', 'Y', 'W', 'O']
    ['R', 'D', 'A', 'B', 'C']
    ['F', 'G', 'H', 'I', 'L']
    ['M', 'N', 'P', 'Q', 'S']
    ['T', 'U', 'V', 'X', 'Z']
    Encrypted Text: GYIZSCOKCFBU
   ```

2. **Decrypt a Message**:
   ```bash
   ./playfair-x86_64-linux -k KEYWORD -i GYIZSCOKCFBU -d
   ```

   **Output**:
   ```
    Generated Playfair Table:
    ['K', 'E', 'Y', 'W', 'O']
    ['R', 'D', 'A', 'B', 'C']
    ['F', 'G', 'H', 'I', 'L']
    ['M', 'N', 'P', 'Q', 'S']
    ['T', 'U', 'V', 'X', 'Z']
    Decrypted Text: HELXLOWORLDX
   ```

---

### Help Message

You can get the help message using the `--help` or`-h` flag:

```bash
./playfair-x86_64-linux --help
```

---

### Running Tests

To run the unit tests run:

```bash
cargo test
```

---

## Limitataions

1. **Decrypted Text with `X`**:
   - Decrypted text may include `X` characters:
     - Between repeated characters (for ex. "BALLOON" -> "BALXLOON").
     - At the end of an odd-length message.

   This is a limitation of the Playfair Cipher, and not a bug.

2. **Non-Alphabetic Characters**:
   - Non-alphabetic characters (for ex. numbers) are ignored.

---

## Author

Developed by [martian58](https://github.com/martian58).