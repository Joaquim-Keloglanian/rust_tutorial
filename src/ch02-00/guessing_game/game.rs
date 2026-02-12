use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    const ORANGE: &str = "\x1b[38;2;255;128;0m";
    const GREEN: &str = "\x1b[38;2;0;255;0m";
    const YELLOW: &str = "\x1b[38;2;255;255;0m";
    const CYAN: &str = "\x1b[38;2;0;255;255m";
    const RESET: &str = "\x1b[0m";

    loop {
        println!("{YELLOW}Please input your guess.{RESET}");

        let mut guess: String = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("{CYAN}Too small!{RESET}"),
            std::cmp::Ordering::Greater => println!("{ORANGE}Too big{RESET}"),
            std::cmp::Ordering::Equal => {
                println!("{GREEN}You win!{RESET}");
                break;
            }
        }
    }
}
