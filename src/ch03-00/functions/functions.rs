fn functions() {
  let x = plus_one(5);

  println!("The value of x is: {x}");
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
fn plus_one(x: i32) -> i32 {
  // This line could also be ```x + 1```
  x + 1
}
