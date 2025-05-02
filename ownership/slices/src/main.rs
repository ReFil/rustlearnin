fn main() {

    // Slices let you reference a series of elements in a collection rather than the collection as awhole
    // They're a type of reference
    let s = String::from("hello world bongus");

    //let word = first_word(&s); // word will get the value 5

    //s.clear(); // this empties the String, making it equal to ""

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!

    // These are slices , they represent a short range of the bytes within the string
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // These use the conventional rust range syntax si tgese are identical
    let _slice = &s[0..2];
    let _slice = &s[..2];

    // The same syntax applies to the end 

    let len = s.len();

    let _slice = &s[0..len];
    let _slice = &s[..];

    let word = second_word(&s);

    // This clearing motion would make the build fail as word is no longer valid
    // s.clear();
    // Clear takes the mutable reference so you cant use an immutable reference afterward

    println!("{word}");

    // String literals are &str like slices, which is why they are immmutable

    // Slices work on arrays too

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);


}
/*
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}*/

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // return the &string  slice
            return &s[0..i];
        }
    }

    &s[..]
}

// we can do this for the second word function too
// Another trick is use &str as the argfument, then we can provide literals, slices or strings by reference to it

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut end = s.len();
    for (i, &item) in bytes.iter().enumerate() {
        if (item == b' ') && start == 0 {
            start = i;
        }
        else if item == b' ' {
            end = i;
        }
    }

    &s[start..end]
}