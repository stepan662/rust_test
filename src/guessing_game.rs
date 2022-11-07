use rand::Rng;
use std::io;
use std::cmp::Ordering;
use colored::*;

pub fn game() {
  let mut rng = rand::thread_rng();


  let random_number: i32 = rng.gen_range(0..100);

  loop {
    println!("Insert a number:");

    let mut guess = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read from stdin");

    

    let guess = match guess.trim().parse::<i32>() {
      Ok(guess) => guess,
      Err(_) => continue
    };

    match guess.cmp(&random_number) {
      Ordering::Less => println!("{}", "Too small".red()),
      Ordering::Greater => println!("{}", "Too large".red()),
      Ordering::Equal => {
        println!("{}", "You win!".green());
        return;
      }
    }
  }

}
