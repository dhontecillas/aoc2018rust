use std::io;

fn main() {
    let mut line = String::new();
    let mut counter : i32 = 0;

    while let Result::Ok(n_read) = io::stdin().read_line(&mut line) {
        if n_read == 0 {
            break;
        }
        if let Result::Ok(in_num) = line.trim().parse::<i32>() {
            counter = counter + in_num;
        }else{
            println!("Not a number! {}", line);
            break;
        }
        line.clear();
    }
    println!("Counter: {}", counter);
}
