#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
    pub found: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut cipher: CipherError = CipherError {
        expected: ciphered,
        found: original.chars()
        .map(|c|{
            if c.is_uppercase() {
                ('A'+'Z') - c
            } else if c.is_lowercase(){
                ('a'+'z') - c
            }else{
                c
            }
        }).collect::String(),
    };
    if cipher.expected == cipher.found {
        Ok(())
    } else {
        Err(cipher.expected)
    }
}