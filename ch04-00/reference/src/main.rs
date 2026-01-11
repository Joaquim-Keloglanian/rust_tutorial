fn main() {
    let s1: String = String::from("Hello, world!");

    let len: usize = calculate_length(&s1);

    println!("The length of '{s1}' is {len}");
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}