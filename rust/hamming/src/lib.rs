/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len() == s2.len() {
        false => None,
        _ => {
            Some(
                s1.chars().zip(s2.chars())
                    .fold(0, |sum, (c1, c2)| sum + if c1 != c2 {1} else {0})
            )
        }
    }
}
