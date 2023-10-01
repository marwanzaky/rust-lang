use std::io;

fn main() {
    let mut f = String::new();

    println!("Please enter the fahrenheit.");

    io::stdin()
        .read_line(&mut f)
        .expect("Failed to read line");

    let f: f32 = f
        .trim()
        .parse()
        .expect("Failed to parse number");

    let c = fahrenheit_to_celsius(f);

    println!("Celsius: {c}");
}

fn fahrenheit_to_celsius(f: f32) -> f32 {
    ((f - 32.0) * 5.0) / 9.0
}