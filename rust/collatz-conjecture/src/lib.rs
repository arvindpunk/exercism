pub fn collatz(n: u64) -> Option<u64> {
    let res: Result<u64, Option<u64>> = (0..).try_fold(n, |acc, c| {
        let a = match acc {
            0 => Result::Err(None),
            1 => Result::Err(Some(c)),
            _ => {
                match acc%2 {
                    0 => Result::Ok(acc/2),
                    _ => match u64::checked_mul(3, acc) {
                        None => Result::Err(None),
                        Some(x) => match u64::checked_add(x, 1) {
                            None => Result::Err(None),
                            Some(y) => Result::Ok(y)
                        }
                    }
                }
            }
        };
        return a
    });
    res.err().unwrap()
}
