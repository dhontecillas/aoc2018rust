use std::io;
use std::vec::Vec;
use std::collections::HashSet;

fn main() {
    let mut line = String::new();
    let mut sequence = Vec::new();

    // load all numbers
    while let Result::Ok(n_read) = io::stdin().read_line(&mut line) {
        if n_read == 0 {
            break;
        }
        if let Result::Ok(in_num) = line.trim().parse::<i32>() {
            sequence.push(in_num);
        }else{
            println!("Not a number! {}", line);
            break;
        }
        line.clear();
    }

    let mut freqs = HashSet::new();

    let max = sequence.len();
    let mut idx : usize = 0;
    let mut curr_freq : i32 = 0;

    freqs.insert(0);
    let res = loop {
        curr_freq += sequence[idx % max];
        idx += 1;
        if freqs.contains(&curr_freq) {
            break curr_freq;
        }
        freqs.insert(curr_freq);
    };
    println!("Freq: {}", res);
}
