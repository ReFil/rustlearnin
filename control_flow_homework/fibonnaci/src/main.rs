use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let n :u32 = match args[1].trim().parse() {
            Ok(num) => num,
            Err(_) => {println!("Please provide a single numerical argument");
            return;
            }
        };
        println!("Argument is {n}");
        let mut val1 = 1;
        let mut val2 = 1;
        for number in 0..n {
            println!("{val1}");
            let val3 = val1 + val2;
            val1 = val2;
            val2 = val3;
        }
    }
    else {
        println!("Please provide a single numerical argument");
        return;
    }
    

}