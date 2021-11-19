pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut series: Vec<String> = digits.char_indices()
        .map(|(i, _)| digits.get(i..(i + len)))
        .filter(|s| s.is_some())
        .map(|s| s.unwrap().to_owned())
        .collect();
    if len == 0 {
        series.push("".to_owned());
    }
    series
}
