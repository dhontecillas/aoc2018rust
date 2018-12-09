use std::io;
use std::vec::Vec;

fn main() {
    let mut line = String::new();
    let mut candidates = Vec::new();

    while let Result::Ok(nread) = io::stdin().read_line(&mut line) {
        if nread == 0 {
            break;
        }
        candidates.push(line.clone());
        line.clear();
    }

    let max = candidates.len();
    for i in 0..max {
        let str_a = &candidates[i];
        for j in i+1..max {
            let str_b = &candidates[j];
            if str_a.len() != str_b.len() {
                panic!("Different len strings");
            }
            let mut diff = 0;
            for (cha, chb) in str_a.chars().zip(str_b.chars()) {
                if cha != chb {
                    diff += 1;
                    if diff > 1 {
                        break;
                    }
                }
            }
            if diff <= 1 {
                println!("Str_A: {}Str_B: {}", str_a, str_b);
                break;
            }
        }
    }

}
