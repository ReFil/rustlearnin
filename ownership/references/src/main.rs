fn main() {
    // Rust references allow one to pass non-copyable values to a function without moving their scope there
    // Whilst this is similar to C pointers, unlike a pointer a reference is guaranteed to be valid for its lifetime
    
    let s = String::from("hello");

    let len = calculate_length(&s);

    println!("The length of {s} is {len}");

    // When you use a reference to pass a value somewhere it has been "borrowed" as the reference does not have ownership of the varuable
    // Unlike with pointers you cannot modify a referenced variable by reference by defaut
    // Everything has to be mutable

    let mut s = s;

    change_str(&mut s);

    println!("String changed to {s}");

    // You cannot borrow mutably more than once in the same scope
    // e.g.
  //  let r1 = &mut s;
    // let r2 = &mut s;

    // If the second let was uncommented code wouldn't compile as this would allow all sorts of carnage like double uses and raceconditions
    // you can lock refs inside a scope so this works for example
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

  //  let r2 = &mut s;

    // youalso cannot have mutable and immutable references in scope at the same time
    // but you can have lots of immutable references in scope simultaneously
    // A reference is scoped for the time it's created until the last time it is used

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}", r1, r2);

    let r3 = &mut s;

    println!("{r3}");

    // Rust will not let you return references to variables that are about to go out of scope as this can create use-after-frees

    


}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change_str(str: &mut String) {
    str.push_str("bongus");
}
