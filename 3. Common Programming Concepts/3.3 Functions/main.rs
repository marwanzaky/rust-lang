/**
 * Rust code uses snake case as the conventional style for function and variable names
 */

fn main() {
    println!("Hello, world!");

    let x = five();
    print_labeled_measurement(x, 'h');
}

fn five() -> i32 {
    5
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}