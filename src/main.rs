mod chapters;

fn main() {
  println!("Rust tutorial runner");

  // chapters::ch02::guessing_game::run();
  // chapters::ch03::functions::run();
  // chapters::ch03::branches::run();
  // chapters::ch03::loops::run();
  // chapters::ch04::ownership::run();
  // chapters::ch04::reference::run();

  // Testing out coloured print statement in Git Bash
  const RESET: &str = "\x1b[0m";
  const RED: &str = "\x1b[38;2;255;0;0m";
  const GREEN: &str = "\x1b[38;2;0;255;0m";
  const BLUE: &str = "\x1b[38;2;0;255;255m";
  const WHITE_BACKGROUND: &str = "\x1b[47m";
  const BLINK: &str = "\x1b[5m";

  println!("{RED}{BLINK}This is red text!{RESET}");
  println!("{GREEN}This is green text!{RESET}");
  println!("{BLUE}This is blue text!{RESET}");
  println!("{WHITE_BACKGROUND}This is white background!{RESET}");
}
