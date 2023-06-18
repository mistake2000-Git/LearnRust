use std::os::unix::io;


fn main() {
    println!("Hello, world!");
    let mut guess:String = String::new();
    io::std()
        .read_line(&mut guess)
        .expect("Failed to readline");
    println!("You guessed: {}",guess);
}
