// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

// fn main(){
//     println!("This is a Guessing Game");

//     let secret_num = rand::thread_rng().gen_range::<u32,_>(1..11);

//     println!("The secret number is: {} ", secret_num);

//     println!("Please enter a number");

//     let mut guess = String::new();
    
    
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     let guess: u32 = guess.trim().parse().unwrap();

//     println!("You guessed: {} ", guess);

//     match guess.cmp(&secret_num){
//         Ordering::Less => println!("Your guess is too small"),
//         Ordering::Greater => println!("Your guess is too high"),
//         Ordering::Equal => println!("You guessed the right number")
//     }
// }

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("This is a Guessing Game");

    let secret_num = rand::thread_rng().gen_range::<u32,_>(1..11);

    loop {
        println!("Please enter a number");

        let mut guess = String::new();
        
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
    
        };

        println!("You guessed: {} ", guess);

        match guess.cmp(&secret_num){
            Ordering::Less => println!("Your guess is too small"),
            Ordering::Greater => println!("Your guess is too high"),
            Ordering::Equal => {
                println!("You guessed the right number");
                break;
            }
        }
    }
}