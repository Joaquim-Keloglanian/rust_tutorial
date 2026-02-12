fn main() {
    let s1: String = String::from("Hello");

    let len: usize = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");

    borrow_example();

    let _reference_to_nothing: String = dangle();
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn borrow_example() -> () {
    let mut s: String = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) -> () {
    some_string.push_str(", world!");
}

fn dangle() -> String {
    let s: String = String::from("hello");
    return s;
}
