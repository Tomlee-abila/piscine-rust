use std::io;

fn main() {
    let mut count = 0;
    loop{
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        let mut guess = String::new();
        count+=1;
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_string();

        if guess == "The letter e"{
            break;
        }
    }
    println!("Number of trials: {count}")
}
