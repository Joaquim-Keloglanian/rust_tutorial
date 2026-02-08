mod chapters;

fn main() {
  println!("Rust tutorial runner");

  // Pick what you want to run while learning each chapter.
  chapters::ch02::guessing_game::run();
  chapters::ch03::functions::run();
  chapters::ch03::branches::run();
  chapters::ch03::loops::run();
  chapters::ch04::ownership::run();
  chapters::ch04::reference::run();
}
