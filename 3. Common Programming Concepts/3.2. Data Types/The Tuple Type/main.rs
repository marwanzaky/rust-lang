fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup;

    println!("The value of x is: {x}");
    println!("The value of y is: {}", tup.1);
    println!("The value of z is: {}", tup.2);
}
