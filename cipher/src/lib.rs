#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
    pub found: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut cipher = Cipher::new(original, ciphered);
    cipher.cipher()
}