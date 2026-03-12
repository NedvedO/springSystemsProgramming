const FREEZING_F_F64: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_F_F64) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + FREEZING_F_F64
}

fn main() {
    let mut temp_f: f64 = 32.0;

    let temp_c: f64 = fahrenheit_to_celsius(temp_f);
    println!("{temp_f}°F = {temp_c}°C");

    // Print the next 5 integer temperatures
    for _ in 0..5 {
        temp_f = temp_f + 1.0;
        let c = fahrenheit_to_celsius(temp_f);
        println!("{temp_f}°F = {c}°C");
    }

    // (Optional quick check of the other function)
    let back_to_f: f64 = celsius_to_fahrenheit(temp_c);
    println!("{temp_c}°C = {back_to_f}°F");
}