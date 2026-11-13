use rand::RngExt;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess nr bitch");
    //i now need to create an unassigned 32 bit integer (32u) called random_num, the first thing i have to do is declare the variable, give it a type and then do the rng bullshit
    let secret_num: u32 = rand::rng().random_range(1..=1000);
    //now i need to print that number to ensure shit works
    //print!("result is : {}", secret_num);
    //shit works yay now i need to make you actually guess it (A.K.A FUCK the line i just wrote)
    //this precious premium loop will make you actually guess the number so coom (also i need to test positive for an std to continue)
    //i have to make a mutable guess string here and then use std::io for user input
    loop {
        let mut guess: String = String::new();
        //here i have to do the standard input shenanigans and refference the guess variable
        io::stdin()
            .read_line(&mut guess)
            .expect("mission failed m8");
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("bitch i said number not letter give me a fucking number");
        println!("you guessed : {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("cold"),
            Ordering::Greater => println!("SO FUCKING HOTTTTTTTTTTTTTTTTTTTTTTTTTTTTT"),
            Ordering::Equal => {
                println!("yeah right thank youuuuuuuuu");
                break;
            }
        }
    }
}
