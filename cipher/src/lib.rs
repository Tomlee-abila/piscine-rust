#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let res = atbash(original);

    match res == ciphered {
        true => Ok(()),
        false => Err(CipherError{
            expected: res,
        })
    }
}

pub fn atbash(s: &str) -> String {
    let mut s1 = String::from("");
    for c in s.chars() {
        if c >= 'a' && c <= 'z' {
            let n = (122 - c as u8) + 97;
            s1.push(n as char)
        } else if c >= 'A' && c <= 'Z' {
            let n = (90 - c as u8) + 65;
            s1.push(n as char)
        } else {
            s1.push(c)
        }
    }
    s1
}