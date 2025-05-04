// Rectangle struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are functions you can apply to struct element
// impl stands for implementation block for rectangle dtype
impl Rectangle {
    // All impl functions have their first arguments be self of type self
    // &self doesn't take ownership, you can hypothetically do &mut self

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // A method can share the same name as a struct element
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, comp: &Rectangle) -> bool {
        (self.width > comp.width) && (self.height > comp.height)
    }

    // You can have associated functions that dont take self as an argument
    // This function returns a rectangle
    fn square (size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle{
        width: 50,
        height: 40,
    };

    println!("The area of the rectangle is {0}", area(&r1));

    // It would be nice to print structs but we can't do this
    //println!("rect {0}", r1);
    // Display trait is not implemented by default
    // but we can use a debug print doohickey
    // This will get big mad if we do not implement the derived trait for debug
    println!("rect {0:?}", r1);
    // The pretty print options prints each element on a few row
    println!("rect {0:#?}", r1);

    // Another way to print out a debug value is using dbg!
    // dbg! prints line numbers, expression fed to it and expression returned if it's expected to be evaluated

    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);


    println!("{0}, {1}", rect1.area(), rect1.width());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Associated functions are called with Struct::fn (like String::from)

    let sq = Rectangle::square(300);

    dbg!(sq);

}


// Function uses immutable borrow to evaluate using struct elements without the full move
fn area(rect: &Rectangle) -> u32{
    rect.width * rect.height
}
