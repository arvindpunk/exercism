use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mag_hash = HashMap::new();
    for &word in magazine {
        let entry = mag_hash.entry(word).or_insert(0);
        *entry += 1;
    }
    for &word in note {
        let entry = mag_hash.entry(word).or_insert(0);
        if *entry == 0 {
            return false
        }
        *entry -= 1;
    }
    true
}
