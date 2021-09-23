use std::io;
extern crate rand;
use rand::Rng; 
use std::cmp::Ordering;


fn main() {
    println!("guesss the number");
    println!("Please input your guess.");

   loop{ 
    let mut guess = String::new();
    let secret_number = rand::thread_rng().gen_range(1..101);


    io::stdin()
       .read_line(&mut guess)
      .expect("Failed to read line");



//let guess: u32 = guess.trim().parse().expect("please type a number");
let guess: u32 = match guess.trim().parse(){
    Ok(num) => num,
    Err(_) => continue,
};
    println!("YOu guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {println!("you win");
                                break;
            }
        }
    }
}
