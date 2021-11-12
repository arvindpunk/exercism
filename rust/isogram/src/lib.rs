pub fn check(candidate: &str) -> bool {
    let result = candidate.chars().try_fold(0, |mask, c| {
        let bit = match c {
            'a'..='z' => 1 << (c as u32 - 'a' as u32),
            'A'..='Z' => 1 << (c as u32 - 'A' as u32),
            ' ' | '-' => 0,
            _ => -1
        };
        if bit == -1 {
            return Result::Err(-1)
        }
        if mask & bit != 0 {
            return Result::Err(-1)
        }
        Result::Ok(mask | bit)
    });
    result.ok().is_some()
}