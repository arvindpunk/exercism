pub fn rotate(input: &str, key: i8) -> String {
    input.chars().map(|c| match c.is_ascii_alphabetic() {
        true => _transform(c, key),
        false => c
    }).collect()
}

pub fn _transform(c: char, key: i8) -> char {
    let base = if c.is_ascii_uppercase() { 'A' } else { 'a' } as i8;
    let new_c = (c as i8 - base + key%26)%26 + base;
    new_c as u8 as char
}