//! Guess the Number Game - English / Français / Polski

use std::io;

fn main() {
    println!("======================================");
    println!("Select your language / Choisissez votre langue / Wybierz język:");
    println!("1. English");
    println!("2. Français");
    println!("3. Polski");
    println!("======================================");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim() {
        "2" => run_french(),
        "3" => run_polish(),
        _ => run_english(),
    }
}

// ==================== ENGLISH ====================
fn run_english() {
    let secret_number: u32 = 42;
    let mut attempt: u32 = 0;

    println!("\n=== GUESS THE NUMBER ===");
    println!("I'm thinking of a number between 1 and 100.");
    println!("Can you guess it?\n");

    loop {
        let mut user_input = String::new();
        println!("Enter your guess: ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        attempt += 1;

        let user_guess: u32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("❌ Invalid input. Please enter a number.");
                continue;
            }
        };

        if user_guess < secret_number {
            println!("📈 Too small!");
        } else if user_guess > secret_number {
            println!("📉 Too big!");
        } else {
            println!("\n🎉 BRAVO! The secret number was {} 🎉", secret_number);
            break;
        }
    }

    println!("Number of attempts: {}", attempt);
}

// ==================== FRANÇAIS ====================
fn run_french() {
    let secret_number: u32 = 42;
    let mut attempt: u32 = 0;

    println!("\n=== DEVINE LE NOMBRE ===");
    println!("Je pense à un nombre entre 1 et 100.");
    println!("Pouvez-vous le deviner ?\n");

    loop {
        let mut user_input = String::new();
        println!("Entrez votre proposition : ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Échec de la lecture");

        attempt += 1;

        let user_guess: u32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("❌ Entrée invalide. Veuillez entrer un nombre.");
                continue;
            }
        };

        if user_guess < secret_number {
            println!("📈 Trop petit !");
        } else if user_guess > secret_number {
            println!("📉 Trop grand !");
        } else {
            println!("\n🎉 BRAVO ! Le nombre secret était {} 🎉", secret_number);
            break;
        }
    }

    println!("Nombre de tentatives : {}", attempt);
}

// ==================== POLSKI ====================
fn run_polish() {
    let secret_number: u32 = 42;
    let mut attempt: u32 = 0;

    println!("\n=== ZGADNIJ LICZBĘ ===");
    println!("Myślę o liczbie między 1 a 100.");
    println!("Czy zgadniesz ?\n");

    loop {
        let mut user_input = String::new();
        println!("Podaj swoją propozycję : ");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Błąd odczytu");

        attempt += 1;

        let user_guess: u32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("❌ Nieprawidłowe dane. Wprowadź liczbę.");
                continue;
            }
        };

        if user_guess < secret_number {
            println!("📈 Za mało !");
        } else if user_guess > secret_number {
            println!("📉 Za dużo !");
        } else {
            println!("\n🎉 BRAWO ! Sekretna liczba to {} 🎉", secret_number);
            break;
        }
    }

    println!("Liczba prób : {}", attempt);
}
