use std::io;

fn main() {
    loop {
        println!("Select temperature direction:");
        println!("1: F to C");
        println!("2: C to F");

        let mut selection = String::new();


        // io is the imported std::io
        // seems similar to python imports where std::io::stdin would also be valud
        // Call "read line" method on the stdin handle to get user input
        // & is a reference &mut needed so the read line function can actually change the reference
        // No specific indentation etc required, stdin().bongus().bingus(); perfectly acceptable syntax
        // read_line returns a "result" enum, so we can append .expect so if the result is Err
        // program will end and will display error message
        // If expect not present there will be a compiler warning
        io::stdin()
            .read_line(&mut selection)
            .expect("failed to read line");

        let selection :u32 = match selection.trim().parse() {
            Ok(num)=> num,
            Err(_) => {println!("Please input either 1 or 2");
                    continue;},
            };

        if selection == 2 {
            println!("Please input temperature in Celcius");
            let mut intemp = String::new();

            io::stdin()
                .read_line(&mut intemp)
                .expect("Failed to read line");

            let intemp :f32 = match intemp.trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("Please input a number");
                    continue;},
                };
            

            let outemp :f32 = (intemp * (9.0/5.0)) + 32.0;

            println!("{intemp}F is {outemp}C");

        } else if selection == 1 {
            println!("Please input temperature in Fahrenheit");
            let mut intemp = String::new();

            io::stdin()
                .read_line(&mut intemp)
                .expect("Failed to read line");

            let intemp :f32 = match intemp.trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("Please input a number");
                    continue;},
                };
            

            let outemp :f32 = (intemp - 32.0) * (5.0/9.0);

            println!("{intemp}F is {outemp}C");
        } else {
            println!("Please input either 1 or 2");
        }
    }
}
