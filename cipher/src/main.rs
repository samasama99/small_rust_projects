// use std::str::FromStr;

static LOWER_START: u8 = 97;
static LOWER_END: u8 = 122;
static UPPER_START: u8 = 65;
static UPPER_END: u8 = 90;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    validation: bool,
    expected: String,
}

impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation,
            expected,
        }
    }
}

fn mirror(c: char) -> Option<char> {
    if !c.is_ascii_alphabetic() {
        return None;
    }
    Some(match c.is_ascii_uppercase() {
        true => (UPPER_END - (c as u8 - UPPER_START)) as char,
        false => (LOWER_END - (c as u8 - LOWER_START)) as char,
    })
}

pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original.is_empty() {
        return None;
    }

    let res = original
        .chars()
        .map(|c| match mirror(c) {
            Some(m) => m,
            None => c,
        })
        .collect();

    match res == ciphered {
        true => Some(Ok(true)),
        _ => Some(Err(CipherError::new(false, res))),
    }
}

fn main() {
    println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
    println!("{:?}", cipher("1Hello 2world!", "svool"));
    println!("{:?}", cipher("", "svool"));
}

// Some(Ok(true))
// Some(Err(CipherError { validation: false, expected: "1Svool 2dliow!" }))
// None
