fn main() {
    let temp_celcius = 100.0;
    // Celcius to fahrenheit
    let temp_fah = celcius_to_fah(temp_celcius);
    println!("{:.2} C = {:.2} F", temp_celcius, temp_fah);
    // fahrenheit to celcius
    let temp_fa = fah_to_celcius(temp_fah);
    println!("{:.2} F = {:.2} C ", temp_fah, temp_fa);
    // Celsius to kelvin
    let tem_c_k = celcius_to_kelvin(temp_celcius);
    println!("{:.2} C = {:.2} K", temp_celcius, tem_c_k);
    // Kelvin to celcius
    let tem_k_c = kelvin_to_celcius(tem_c_k);
    println!("{:.2} K = {:.2} C ", tem_c_k, tem_k_c);
    // fahrenheit to kelvin
    let tem_f_k = fah_to_kelvin(temp_fah);
    println!("{:.2} F = {:.2} K", temp_fah, tem_f_k);
    // kelvin to fahrenheit
    let tem_k_f = kelvin_to_fah(tem_c_k);
    println!("{:.2} k = {:.2} F ", tem_c_k, tem_k_f);
}

fn celcius_to_fah(celcius: f64) -> f64 {
    celcius * 9.0 / 5.0 + 32.0
}
fn fah_to_celcius(fah: f64) -> f64 {
    (fah - 32.0) * 5.0 / 9.0
}
fn celcius_to_kelvin(celcius: f64) -> f64 {
    celcius + 273.15
}
fn kelvin_to_celcius(kelvine: f64) -> f64 {
    kelvine - 273.15
}
fn fah_to_kelvin(fah: f64) -> f64 {
    let c = fah_to_celcius(fah);
    celcius_to_kelvin(c)
}
fn kelvin_to_fah(kelvine: f64) -> f64 {
    let c = kelvin_to_celcius(kelvine);
    celcius_to_fah(c)
}
