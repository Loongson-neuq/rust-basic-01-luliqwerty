use rand::Rng;
use std::cmp::Ordering;
// use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");

    // loop {
    //     println!("input your number");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");

    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You number: {guess}");

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    let mut left: i32 = 0;
    let mut right: i32 = 100;
    loop {

        let mid = (left + right) / 2;
        println!("{mid}");
        // if mid > secret_number {
        //     println!("too big");
        //     right = mid;
        // } else if mid < secret_number {
        //     println!("too small");
        //     left = mid
        // } else {
        //     println!("you win");
        //     break;
        // }
        match mid.cmp(&secret_number) {
            Ordering::Less => {
                println!("too small");
                left = mid;
            }
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Greater => {
                println!("too big");
                right = mid;
            }
        }
    }
}