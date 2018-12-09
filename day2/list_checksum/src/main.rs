use std::io;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();

    let mut has_2 : u32 = 0;
    let mut has_3 : u32 = 0;

    let mut counter = HashMap::new();
    while let Result::Ok(nread) = io::stdin().read_line(&mut line) {
        if nread == 0 {
            break;
        }
        counter.clear();
        for ch in line.trim().chars() {
            if !counter.contains_key(&ch) {
                counter.insert(ch, 0);
            }
            let mut cnt = counter.get_mut(&ch).unwrap();
            *cnt = *cnt + 1;
        }
        for v in counter.values() {
            if *v == 2 {
                has_2 += 1;
                break;
            }
        }
        for v in counter.values() {
            if *v == 3 {
                has_3 += 1;
                break;
            }
        }
        line.clear();
    }
    println!("Checksum: {}", has_2 * has_3);
}
