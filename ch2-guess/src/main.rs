use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let ans = rand::thread_rng().gen_range(1..101);
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) =>{
                println!("wtf?");
                continue;
            }
        };

        println!("{}", guess);

        match guess.cmp(&ans) {
            Ordering::Less => {
                println!("too small");
            },
            Ordering::Greater => {
                println!("too big");
            },
            Ordering::Equal => {
                println!("right!");
                break;
            }
        }
    }
}
