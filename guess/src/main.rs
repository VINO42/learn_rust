use std::io;

fn main() {
    println!("大家来猜谜语");
    println!("Please Inut the num.");
    let mut guess =String::new ();

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read line ");
        println!("You guessed:{}",guess);
}
