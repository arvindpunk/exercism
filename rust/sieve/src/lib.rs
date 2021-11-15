pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut sieve = vec![true; (upper_bound + 1) as usize];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..=upper_bound {
        if sieve[i as usize] {
            (2..).take_while(|n| n * i <= upper_bound)
                .for_each(|n| sieve[(n * i) as usize] = false);
        }
    }
    sieve.iter()
        .enumerate()
        .filter_map(|(i, &b)| if b {Some(i as u64)} else {None})
        .collect()
}
