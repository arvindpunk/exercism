/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let encoded = _convert(plain);
    _split_at_n_by(encoded, 5, ' ')
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    _convert(cipher)
}

pub fn _convert(text: &str) -> String {
    text.chars().filter_map(|c| {
        match c.is_ascii_alphanumeric() {
            true => match c.is_ascii_alphabetic() {
                true => Some(('a' as u8 + (25 - (c.to_ascii_lowercase() as u8 - 'a' as u8))) as char),
                false => Some(c)
            },
            false => None
        }
    }).collect()

}

pub fn _split_at_n_by(text: String, n: u32, delimiter: char) -> String {
    text.chars().fold((vec![], 0), |(mut acc, ct), c| {
        if ct != 0 && ct%n == 0 {
            acc.push(delimiter);
        }
        acc.push(c);
        (acc , ct + 1)
    }).0.iter().collect()
}