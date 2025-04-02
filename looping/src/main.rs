use std::io;

fn main() {
    println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");

    let mut guess = String::new();

    let mut count = 0;

    loop{
        count+=1;
        io::stdin()
            .read_line(&mut guess)
            .expect("msg");

        if guess == "The letter e"{
            break;
        }
    }
    print!("Number of trials: {count}")
}
