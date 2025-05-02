fn main() {
    // Adding {} allows one to scope variables to minimise the range within which they're usable
    {
        // cannot use s as it's not been declared yet
        let s = "bongus";
        // Can use s
        println!("{s}");
    } //can no longer use s as scope is over
    // String literals are immutable, but what if we don't want that
    // String dtype allocates on the heap andcan store an amount of data unknown at compile time
    // :: namespaces the from function to the string operator
    // from function takes a string literal as the argument
    let s = String::from("hello");

    // These can be mutated e.g.
    let mut s = s;

    //.push_str function appends a literal to string

    s.push_str(", bongus");
    println!("{s}");

    // the memory used by s is freed when it goes out of scope

    // With some dtypes when you let x = y, y gets freed and is invalidated
    // This is becayse x is not copues, it's passed the pointer over from y
    // Freeing both x and y at end of scope would lead to a double free
    // The terminolodgy is y is moved to x

    {
        let s1 = String::from("hello");
        let _s2 = s1;

        // println!("{s1}, world!"); this line would cause build fail
    }

    {
        let mut s = String::from("hello");
        // When we assign a completely new value to s Rust knows this and will free the original memory
        s = String::from("bongus");

        println!("{s}, world!");

    }
    // Rust will never automatically do full copy by value, thus any automatic copying can be assumed to be very performant
    {
        //If you do want to do a full copy you can use the .clone method

        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("S1: {s1}, s2: {s2}");
    }

    // But this does work, so what gives?
    {
        let x = 5;
        let y = x;
    
        println!("x = {x}, y = {y}");
        // Since ints are aknown size rust will happily do a normal copy without drop
    }
    // Data types can have a "copy" trait which dictates whether they are moved/copied by reference or fully copied
    // Anything that has drop implemented cannot be given the copy trait
    // Ints, bools, floats, chars and tuples that only implement copyable types can have the copy trait
    // A move/copy happens with function calls too

    {    
        let s = String::from("hello");  // s comes into scope

        takes_ownership(s);             // s's value moves into the function...
                                        // ... and so is no longer valid here

        let x = 5;                      // x comes into scope

        makes_copy(x);                  // because i32 implements the Copy trait,
                                        // x does NOT move into the function,
        println!("{}", x);              // so it's okay to use x afterward}

    }

    // You cannot use non copyable vars after passing them to a function

    // This passing mechanic happens with returns too
    {
        let s1 = gives_ownership();         // gives_ownership moves its return
                                            // value into s1
    
        let s2 = String::from("hello");     // s2 comes into scope
    
        let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

      // assigning a value to another variable moves it. When a non copyable var moves out of scope, value is dropped
      // If it's been moved before going out of scope the value isn't dropped

    // Passing values back from functions if we want to reuse them kind of sucks, you can pass back a tupe with original and returned values

    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
    }
    // but this is still kind of tedious, there's got to be a better way!
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}