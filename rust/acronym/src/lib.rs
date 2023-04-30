pub fn abbreviate(phrase: &str) -> String {
    let mut abbr = "".to_owned();
    let slices: Vec<&str> = phrase.trim().split_whitespace().collect();
    for slice in slices {
        let curr = {
            if is_all_caps(slice) {
                get_all_caps_abbr(slice)
            } else {
                "".to_owned()
            }
        };
        abbr = format!("{}{}", curr, abbr);
    }
    abbr
}

pub fn is_all_caps(slice: &str) -> bool {
    slice.chars().all(|c| !(c >= 'a' && c <= 'z'))
}

pub fn get_all_caps_abbr(slice: &str) -> String {
    slice.chars().filter(|&c| c >= 'A' && c <= 'Z').collect()
}