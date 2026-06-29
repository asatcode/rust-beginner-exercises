//! Average Calculator - Beginner Rust Exercise
//! Available in English, French, and Polish

use std::io;

fn main() {
    // Language selection
    println!("======================================");
    println!("Select your language / Choisissez votre langue / Wybierz język:");
    println!("1. English");
    println!("2. Français");
    println!("3. Polski");
    println!("======================================");

    let lang = get_language_choice();

    match lang {
        1 => run_english(),
        2 => run_french(),
        3 => run_polish(),
        _ => {
            println!("Invalid choice. Defaulting to English.");
            run_english();
        }
    }
}

fn get_language_choice() -> u8 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse::<u8>() {
        Ok(num) if num >= 1 && num <= 3 => num,
        _ => 1, // Default to English
    }
}

// ==================== ENGLISH VERSION ====================
fn run_english() {
    println!("\n=== AVERAGE CALCULATOR ===");
    println!("Enter numbers one by one.");
    println!("Type 'done' when you finish.\n");

    let mut numbers: Vec<f64> = Vec::new();

    loop {
        print!("Enter a number (or 'done'): ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "done" {
            break;
        }

        match input.parse::<f64>() {
            Ok(num) => {
                numbers.push(num);
                println!("✓ Added: {}", num);
            }
            Err(_) => {
                println!("✗ Invalid input. Please enter a number or 'done'.");
            }
        }
    }

    display_results_english(&numbers);
}

fn display_results_english(numbers: &[f64]) {
    if numbers.is_empty() {
        println!("\nNo numbers entered. Cannot calculate average.");
    } else {
        let sum: f64 = numbers.iter().sum();
        let count = numbers.len();
        let average = sum / count as f64;

        println!("\n=== RESULTS ===");
        println!("Numbers entered: {:?}", numbers);
        println!("Count: {}", count);
        println!("Sum: {:.2}", sum);
        println!("Average: {:.2}", average);
    }
}

// ==================== FRENCH VERSION ====================
fn run_french() {
    println!("\n=== CALCULATRICE DE MOYENNE ===");
    println!("Entrez les nombres un par un.");
    println!("Tapez 'fin' lorsque vous avez terminé.\n");

    let mut numbers: Vec<f64> = Vec::new();

    loop {
        print!("Entrez un nombre (ou 'fin'): ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "fin" {
            break;
        }

        match input.parse::<f64>() {
            Ok(num) => {
                numbers.push(num);
                println!("✓ Ajouté : {}", num);
            }
            Err(_) => {
                println!("✗ Entrée invalide. Veuillez entrer un nombre ou 'fin'.");
            }
        }
    }

    display_results_french(&numbers);
}

fn display_results_french(numbers: &[f64]) {
    if numbers.is_empty() {
        println!("\nAucun nombre entré. Impossible de calculer la moyenne.");
    } else {
        let sum: f64 = numbers.iter().sum();
        let count = numbers.len();
        let average = sum / count as f64;

        println!("\n=== RÉSULTATS ===");
        println!("Nombres saisis : {:?}", numbers);
        println!("Quantité : {}", count);
        println!("Somme : {:.2}", sum);
        println!("Moyenne : {:.2}", average);
    }
}

// ==================== POLISH VERSION ====================
fn run_polish() {
    println!("\n=== KALKULATOR ŚREDNIEJ ===");
    println!("Wprowadzaj liczby pojedynczo.");
    println!("Wpisz 'koniec' gdy skończysz.\n");

    let mut numbers: Vec<f64> = Vec::new();

    loop {
        print!("Wprowadź liczbę (lub 'koniec'): ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input == "koniec" {
            break;
        }

        match input.parse::<f64>() {
            Ok(num) => {
                numbers.push(num);
                println!("✓ Dodano: {}", num);
            }
            Err(_) => {
                println!("✗ Nieprawidłowe dane. Wprowadź liczbę lub 'koniec'.");
            }
        }
    }

    display_results_polish(&numbers);
}

fn display_results_polish(numbers: &[f64]) {
    if numbers.is_empty() {
        println!("\nNie wprowadzono żadnych liczb. Nie można obliczyć średniej.");
    } else {
        let sum: f64 = numbers.iter().sum();
        let count = numbers.len();
        let average = sum / count as f64;

        println!("\n=== WYNIKI ===");
        println!("Wprowadzone liczby: {:?}", numbers);
        println!("Ilość: {}", count);
        println!("Suma: {:.2}", sum);
        println!("Średnia: {:.2}", average);
    }
}
