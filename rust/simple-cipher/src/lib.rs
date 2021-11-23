use std::{cmp::max};

use rand::random;

pub fn encode(key: &str, s: &str) -> Option<String> {
    println!("e {} {}", key, s);
    if key.len()  == 0 {
        None
    } else 
    if key.chars().all(|c| c.is_ascii_lowercase()) {
        Some(key.chars().cycle().zip(s.chars()).map(|(k, c)| match c.is_ascii_alphabetic() {
            true => _transform(c, k.to_ascii_lowercase() as i8 - 'a' as i8),
            false => c
        }).collect())
    } else {
        None
    }
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    println!("d {} {}", key, s);
    if key.len() == 0 {
        None
    } else if key.chars().all(|c| c.is_ascii_lowercase()) {
        Some(key.chars().cycle().zip(s.chars()).map(|(k, c)| match c.is_ascii_alphabetic() {
            true => _transform(c, 26  - (k.to_ascii_lowercase() as i8 - 'a' as i8)),
            false => c
        }).collect())
    } else {
        None
    }
}

pub fn encode_random(s: &str) -> (String, String) {
    let key: String = vec!['a'; max(101, s.len())].iter().map(|_| (97 + random::<u8>()%26) as char).collect();
    let _key = key.as_str();
    (key.clone(), encode(_key, s).unwrap())
}

pub fn _transform(c: char, key: i8) -> char {
    let base = 'a' as i8;
    let new_c = (c as i8 - base + key%26)%26 + base;
    new_c as u8 as char
}