fn main() {
    let condition: bool = false;
    let number: i32 = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}