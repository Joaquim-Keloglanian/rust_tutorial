fn main() {
    let s1: String = String::from("Hello, world! This is your boy, Skinny Penis!");
    let (s2, len): (String, usize) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(input_string: String) -> (String, usize) {
    let length: usize = input_string.len();
    return (input_string, length);
}