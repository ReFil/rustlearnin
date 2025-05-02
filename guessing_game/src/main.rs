// Use the std io prelude
// Other preludes available here: https://doc.rust-lang.org/std/prelude/index.html

use std::io;

// Import random number prelude
use rand::Rng;

// Import ordering comparison prelude
use std::cmp::Ordering;

fn main() {

    // Exclamation mark denotes a macro
    println!("Guess the number");


    // Secret number is immutable variable
    // thread_rng is an OS seeded RNG generator local to current execution thread
    // gen_range takes a range expression as an argument, start..=end inclusive
    // cargo doc --open gives documentation on deps and crates
    let secret_number = rand::thread_rng().gen_range(1..=100);


    println!("Secret number is: {secret_number}");

    loop{
        println!("Please input guess");

        // let denotes variable
        // mut allows for variable to be changed, by default vars are const
        // String::new() binds a new string instance. new() is a common type instance
        // Analogous to a constructor?
        let mut guess = String::new();


        // io is the imported std::io
        // seems similar to python imports where std::io::stdin would also be valud
        // Call "read line" method on the stdin handle to get user input
        // & is a reference &mut needed so the read line function can actually change the reference
        // No specific indentation etc required, stdin().bongus().bingus(); perfectly acceptable syntax
        // read_line returns a "result" enum, so we can append .expect so if the result is Err
        // program will end and will display error message
        // If expect not present there will be a compiler warning
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        
        // We need to convert the guess string to an u32 for us to compare it properly
        // Let guess - declare new varibale, it's statis so no need for mut
        // u32 - uint32_t
        // guess - guess str
        // trim() - eliminates whitespace at beginning and end and cr/lf
        // parse() - converts to another type. Infered from the : u32
        // Parse returns a result because it might not be reading text, so we add expect
        // We apply match statement to catch errors, continue through loop if error observed, return num if not

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Like in python the {} are placeholders for the variable
        // when printing a variable it can sit inside the curled braces
        // If doing an evaluation the braces should be empty and the evaluation done after a comma
        // e.g println!("bingus: {}", bongus + 5);

        println!("you guessed: {guess}");


        // Ordering type that was imported has three variants, Less, Greater, Equal
        // cmp will return one of these three
        // match is a bit like a switch case type deal?
        // exits after first match, will not run on like a switch case
        // Match consists of arms, each arm has pattern to match against and code to run
        // guess.cmp will not implicitly let us convert guess to i32 for the cmp
        // & needed for a reference
        // can wrap match elements in {} to execute bigger things

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("right");
                break;
            },
        }
    }

}
