use std::io;

fn main() {
    let ans =5;
    let mut guess=String::new();

    io::stdin().read_line(&mut guess).expect("failed");

    println!("{}",guess)
}
