use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Russian roulette");

    loop {
        let bullet = rand::thread_rng().gen_range(1..=6);

        let mut roll = String::new();

        io::stdin().read_line(&mut roll).expect("what the fuck?");

        let roll : u32 = match roll.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Are you stupid?");
                continue;
            }
        };

        match roll.cmp(&bullet) {
            Ordering::Equal => {println!("You are dead"); break;},
            Ordering::Less => println!("Still alive"),
            Ordering::Greater => println!("Lucky bastard")
        }
    }
}
