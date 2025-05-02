fn main() {
    println!("Hello, world!");

    another_function();

    a_second_function(3, 'a');

    a_third_function(4);

    let x = a_fourth_function();

    println!("A fourth function returns {x}");

    let x = a_fifth_function(5);

    println!("A fifth function returns {x}")
}


// Functions defined with fn <function name>
// snake_case_is_the_standard
fn another_function() {
    println!("Another function.");
}

// To give an argument to a function define with fn name(<varname>: <vartype>)
// All types need explicit definition

fn a_second_function(x: i32, b: char){
    println!("Second function arguments is {x}{b}");
}

// You can let a var = {expression that returns something}
// You cannot let a var = (statement that returns nothing)
// Worth noting let var returns nothing

fn a_third_function(x: i32){

    // let y = (let x = 5);

    let y = {
        let x = x;
        x + 1
    };

    println!("Third function method is {y}")
}

// You can return a value without a return statement, just put a value WITHOUT a semicolon 

fn a_fourth_function() -> i32 {
    4
}

// You can also return with a return statement and semicolon

fn a_fifth_function(x: i32) -> i32 {
    return x + 1;
}