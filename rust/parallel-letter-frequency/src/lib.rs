use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut index = 0;
    let freq = HashMap::new();

    for i in 0..worker_count {
        std::thread::spawn(move || {
           
            
        });
    }
    freq
}
