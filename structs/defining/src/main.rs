
// Structs are defined similarly to C

struct Rabbit {
    hungry: bool,
    breed: String,
    location: String,
    age: u64,

}

// These are called tuple structs

struct Colour(i32, i32, i32);
struct Point(i32, i32, i32);

// These are unit like structs

struct Aunitstruct;



fn main() {
    // All fields have to be filled upon instantiation
    let delilah = Rabbit {
        hungry: false,
        breed: String::from("Netherlands Dwarf"),
        location: String::from("Inside your walls"),
        age: 8,
    };

    // Structs can naturally be mutable
    let mut delilah = delilah;

    delilah.location = String::from("Approaching you");

    // The whole thing has to be mutable for any part to be mutated

    let colin = new_rabbit(String::from("Holland Lop"), String::from("Bunny heaven"));

    // you can say elements of another struct = elements of the first struct

   /* let truffles = Rabbit {
        hungry: colin.hungry,
        breed: String::from("Big and clueless"),
        location: colin.location,
        age: colin.age,
    }*/

    // but an easier way of doing this is 

    let truffles = Rabbit {
        breed: String::from("Big and clueless"),
        ..colin
    };

    // Note that when you use values like this you move them, so you couldn't reuse colin.location for example, but you can use the unmoved elements
    // If you instantiated new elements for all non Copyable elements both original and new struct would remain valid. 
    println!("{0}, {1}, {2}", delilah.location, truffles.location, colin.breed);


    let origin = Point(0, 0, 0);
    let red = Colour(255, 0, 0);

    println!("{0}, {1}", origin.0, red.0);

    let _units = Aunitstruct;
    
}

fn new_rabbit(breed: String, location: String) -> Rabbit {

    // Rusts field init shorthand means we dont need to do breed: breed etc
    Rabbit {
        hungry: true,
        breed,
        location,
        age: 0,
    }
}