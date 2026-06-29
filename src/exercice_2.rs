//! Temperature Converter - English / Français / Polski

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
    println!("\n=== TEMPERATURE CONVERTER ===\n");
    
    let temp_celsius = 100.0;
    
    let temp_fah = celsius_to_fah(temp_celsius);
    println!("{:.2} C = {:.2} F", temp_celsius, temp_fah);
    
    let temp_cel = fah_to_celsius(temp_fah);
    println!("{:.2} F = {:.2} C", temp_fah, temp_cel);
    
    let temp_kel = celsius_to_kelvin(temp_celsius);
    println!("{:.2} C = {:.2} K", temp_celsius, temp_kel);
    
    let temp_cel2 = kelvin_to_celsius(temp_kel);
    println!("{:.2} K = {:.2} C", temp_kel, temp_cel2);
    
    let temp_fah_to_kel = fah_to_kelvin(temp_fah);
    println!("{:.2} F = {:.2} K", temp_fah, temp_fah_to_kel);
    
    let temp_kel_to_fah = kelvin_to_fah(temp_kel);
    println!("{:.2} K = {:.2} F", temp_kel, temp_kel_to_fah);
}

// ==================== FRANÇAIS ====================
fn run_french() {
    println!("\n=== CONVERTISSEUR DE TEMPÉRATURE ===\n");
    
    let temp_celsius = 100.0;
    
    let temp_fah = celsius_to_fah(temp_celsius);
    println!("{:.2} °C = {:.2} °F", temp_celsius, temp_fah);
    
    let temp_cel = fah_to_celsius(temp_fah);
    println!("{:.2} °F = {:.2} °C", temp_fah, temp_cel);
    
    let temp_kel = celsius_to_kelvin(temp_celsius);
    println!("{:.2} °C = {:.2} K", temp_celsius, temp_kel);
    
    let temp_cel2 = kelvin_to_celsius(temp_kel);
    println!("{:.2} K = {:.2} °C", temp_kel, temp_cel2);
    
    let temp_fah_to_kel = fah_to_kelvin(temp_fah);
    println!("{:.2} °F = {:.2} K", temp_fah, temp_fah_to_kel);
    
    let temp_kel_to_fah = kelvin_to_fah(temp_kel);
    println!("{:.2} K = {:.2} °F", temp_kel, temp_kel_to_fah);
}

// ==================== POLSKI ====================
fn run_polish() {
    println!("\n=== KONWERTER TEMPERATURY ===\n");
    
    let temp_celsius = 100.0;
    
    let temp_fah = celsius_to_fah(temp_celsius);
    println!("{:.2} °C = {:.2} °F", temp_celsius, temp_fah);
    
    let temp_cel = fah_to_celsius(temp_fah);
    println!("{:.2} °F = {:.2} °C", temp_fah, temp_cel);
    
    let temp_kel = celsius_to_kelvin(temp_celsius);
    println!("{:.2} °C = {:.2} K", temp_celsius, temp_kel);
    
    let temp_cel2 = kelvin_to_celsius(temp_kel);
    println!("{:.2} K = {:.2} °C", temp_kel, temp_cel2);
    
    let temp_fah_to_kel = fah_to_kelvin(temp_fah);
    println!("{:.2} °F = {:.2} K", temp_fah, temp_fah_to_kel);
    
    let temp_kel_to_fah = kelvin_to_fah(temp_kel);
    println!("{:.2} K = {:.2} °F", temp_kel, temp_kel_to_fah);
}

// ==================== FONCTIONS (partagées) ====================
fn celsius_to_fah(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fah_to_celsius(fah: f64) -> f64 {
    (fah - 32.0) * 5.0 / 9.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn fah_to_kelvin(fah: f64) -> f64 {
    let c = fah_to_celsius(fah);
    celsius_to_kelvin(c)
}

fn kelvin_to_fah(kelvin: f64) -> f64 {
    let c = kelvin_to_celsius(kelvin);
    celsius_to_fah(c)
}
