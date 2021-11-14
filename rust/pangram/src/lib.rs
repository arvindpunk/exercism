/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mask = sentence.chars().fold(0, |mask, c| mask | match c {
        'a'..='z' => 1 << (c as u32 - 'a' as u32),
        'A'..='Z' => 1 << (c as u32 - 'A' as u32),
        _ => 0
    });
    mask == ((1 << 26) - 1)
}