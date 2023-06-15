use rand::distributions::{Distribution, Uniform}; // 0.6.5
use std::io::{stdin, stdout, Read, Write};

fn ran_num() -> i32 {
    let step = Uniform::new(1, 100);
    let mut rng = rand::thread_rng();
    let randomnumber = {step.sample(&mut rng)};
    //println!("Random Num: {}", randomnumber);
    return randomnumber;
}
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to exit...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}
fn game(number: i32, tries: i32) {
    println!("What is your guess?");
    read!(input as i32);
    //println!("Input: {}", input);
    if input == number {
        println!("That is correct!");
        if tries == 1 {
            println!("It took you {} try!", tries);
            pause();
        }
        else {
            println!("It took you {} tries!", tries);
            pause();
        }
    }
    if input > number {
        println!("Sorry, but the number is lower than that!");
        let tries: i32 = tries + 1;
        //println!("Tries: {}", tries);
        game(number, tries);
    }
    if input < number {
        println!("Sorry, but the number is higher than that!");
        let tries: i32 = tries + 1;
        //println!("Tries: {}", tries);
        game(number, tries);
    }
}
fn main() {
    println!("Welcome to a Higher or Lower game! You will choose a number between 1 and 100, and the script will tell you if its higher, lower, or correct!");
    let tries: i32 = 1;
    let number: i32 = ran_num();
    game(number, tries);
}
