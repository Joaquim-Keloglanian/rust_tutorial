fn main() {
    let s: String = String::from("hello");      // s comes into scope

    takes_ownership(s);                         // the value of s is passed into this function
                                                // ... and so it is no longer valid

    let x: i32 = 5;                             // x comes into scope

    makes_copy(x);                              // i32 implements the Copy trait,
                                                // so x does not move into the function
                                                // so it's okay to use x afterward
}

fn takes_ownership(some_string: String) -> () { // some_string comes into scope
    println!("{some_string}");
}                                               // some_string goes out of scope and 'drop' is called
                                                // The allocated memory is freed

fn makes_copy(some_integer: i32) -> () {        // some_integer comes into scope
    println!("{some_integer}");
}                                               // some_integers goes out of scope
                                                // it is stored on the stack so nothing special happens here