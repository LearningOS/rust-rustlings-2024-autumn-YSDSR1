use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_number}");
    
    loop{
        let mut guess = String::new();
        println!("please input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let var_name = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
        };
        let guess: u32 = var_name;

        println!("you guessed:{}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
