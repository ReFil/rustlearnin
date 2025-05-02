fn main() {
    let check = 6;


    // You cannot convert int to bool implicitly like in C
    // Rust exits after the first else if correct, does not 
    
    if check > 5 {
        println!("condition was true");
    } else if check == 5 {
        println!("Condition is ssihjgkjh")
    } else {
        println!("condition was false");
    }

    // You can use let var = if to make a very neat construct
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    // Note the value types need to be the same, for example this wouldn't compile
    // let number = if condition { 5 } else {"bongus"};

    println!("The outcome of the number condition is {number}");

    // loop {} is the equivalent of while {1} you love to see it

    /*loop {
        println!("bongus");
    }*/

    // you can return values from a loop when you break them which is nice by saying break <val>
    // as with C if you return using return it exits the whole function not just the loop


    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!{"The returned value is {result}"};

    // You can label a loop too to break a specific one using '

    let mut count = 0;

    'counting_up: loop {
        let mut remaining = 10;
        println!("count = {count}");


        loop {
            println!("Remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Can do a while loop like most other languages

    let mut number = 3;

    while number != 0 {
        println!("Number is {number}");
        number -= 1;
    }

    println!("Bongus");

    // Can use while and a counter to iterate through an array

    let arr = [10, 200, 3000, 40000, 500000];

    let mut index = 0;

    while index < 5 {
        println!("Array at index {} is {}", index, arr[index]);
        index += 1;
    }

    // But there's a better way

    for element in arr {
        println!("Array value is {element}");
    }

    // For loops are good because they have a guaranteed end of execution
    // a range indication can be 1..6. last element is 5, not 6

    for number in 1..6 {
        println!("Number is {number}");
    }

    // You can also wrap it in brackets and use operators on it

    for number in (1..6).rev() {
        println!("Number is {number}");
    }

}
