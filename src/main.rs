// An example of how pointers work in Rust
fn main() {
    let s1 = String::new(); // String 1 enters scope

    let _len = get_length(&s1); // String 1 is passed into the function by referencing it

    let mut s2 = String::from("Hello"); // String 2 enters the scope

    change_string(&mut s2); // String 2 is altered by the function without going out of scope

    println!("{}", s2); // If we passed by value, s2 would be out of scope.

    let s3 = new_string(s1); // Passing by value to edit a string. Bad practice.

    println!("Printing s1 is impossible, because it's out of scope.");
    println!("Printing s3 = \"{s3}\" is fine, because it's taken ownership of s1.");

    // Having multiple references to a mutable value is illegal:
    // let r1 = &mut s2;
    // let r2 = &mut s2;             // Throws a compiler error here!
    // println!("{}, {}", r1, r2);   // Note that the error is only noticed when both references are recognized by the compiler.
    // The reason for this is to prevent racing conditions in multi-threaded processes. If no two mutable references can exist, no problem!
    
    // MOREOVER, you can't even have a mutable reference if there's already a reference in scope!
    let mut s4 = String::from("Hello");
    let r1 = &s4; // No problem
    let r2 = &s4; // No problem
    // let r3 = &mut s4; // BIG PROBLEM
    // println!("{}, {}, and {}", r1, r2, r3); // Error!
    
    // Multiple immutable reference are fine, since all they're doing is reading data.
    // As soon as you make a mutable reference, you're declaring that you're gonna change the underlying reference.
    // That means every previous immutable reference will go bad!
    // Therefore, it's a compiler error to make a "mut &var" after making a "&var" in the same scope.

    // Remember that functions can end the scope of variables, including "&vars"
    println!("{} and {}", r1, r2); // r1 and r2 go out of scope, since they're given to the function.
    let _r3 = &mut s4; // Totally fine now!

    // DANGLING REFERENCES:

}

// Accesses the properties of a string through borrowing (using a reference or pointer)
fn get_length(s: &String) -> usize {
    s.len()
}

// changes the properties of a mutable string through borrowing
fn change_string(s: &mut String) { // creates a mutable reference pointer to s (enters scope)
    s.push_str(", world!"); // does some work with s
} // s goes out of scope

// Changes a string in a slow, pass-by-value way:
fn new_string(s: String) -> String {
    let mut string = s; // transfer ownership of the string to a mutable one
    string.push_str(", world, but different!"); // change the string
    string // return the new, changed string
}

// An example of creating a reference to nowhere
// fn dangle() -> &String { // ERROR: unable to pass a reference to nowhere!
//     let s = String::new(); // s comes into scope
//     &s // return a pointer to s
// } // s goes out of scope!!
// The only way to do this function is to remove the "&" from the return type, or add a "'static" label to it
// The second option has to do with lifetimes in Ch 10.
