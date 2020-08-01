use rand::Rng;
use std::cmp::Ordering;
use std::io;


fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret Number is: {}", secret_number);

    loop {
        println!("Please input the guess!");
    
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        //let guess: u32 = guess.trim().parse().expect("Please Type a Number!");

        // anche se stringa, non int continua, non crashare
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break; // rompi il loop
            }
        }
    }
}


