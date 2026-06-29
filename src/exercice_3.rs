use std::io;
fn main() {
    let secret_number: u32 = 42;
    let mut attempt: i32 = 0;
    loop {
        let mut user_seizure: String = String::new();
        println!("Plaease enter your number of choices: ");
        io::stdin()
            .read_line(&mut user_seizure)
            .expect("faled to read the number  ");
        attempt += 1;
        let user_seizure: u32 = match user_seizure.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Reading failure");
                continue;
            }
        };
        if user_seizure < secret_number {
            println!("Too big");
        } else if user_seizure > secret_number {
            println!("Too small");
        } else {
            println!("\n*** Bravo the number secret was {} ***", secret_number);
            break;
        }
    }
    println!("Number of attempt : {}", attempt);
}
