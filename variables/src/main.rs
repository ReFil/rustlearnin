fn main() {
    let x = 5;
    println!("Hello, world! {x}");
    let x = x+1;
    // Newscope time
    {
        let x = x *2;
        println!("New inner scope value {x}");
    }
    println!("new value {x}");

    // Variable types
    // Four main scalar types, Ints, floats, bools and chars
    // To explicitly type a var use let var: <type> = thing;

    // Ints, i8, i16, i32, i64, i128 and special archsize one isize
    // Also available in unsigned flavours u_size_
    // Rust will check for overflows in debug builds and panic at runtime
    // In relase builds it will not panic, 2's complement overflow
    // Multiple solutions for handlign overflow

    // Floats f32 and f64, defaults to f64 if implicitly defined

    // Bools bool. true or false, primarily evaluated through conditionals

    // Char expressed with '' note rust chars are a 4 byte value, can repesent a lot of unicode

    // Tuples let you squish multiple values with different types into one type
    // Fixed length. initialised by using (x, y, z, a, b, c);
    // Data type is inferred but can be explicitly typed

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Can access by patterning multiple variables which will sort of map them

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    // Can also accexx using tup.index (Zero indexed)

    let a = tup.1;

    println!("Second tuple index valus is: {a}");

    // Arrays
    // Defined a bit like c 

    let a = [1, 2, 3, 4, 5];

    // Arrays will be allocated on stack. Can specifiy type and size like let arr: [<type>; <len>] = [<elems>]
    // <elem> can be structyred as <val>; <len> to fill array with len of value
    // Out of bounds array will cause panic

    let elem = a[3];

    println!("Elem: {elem}")
}
