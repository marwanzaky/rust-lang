fn main() {
    let mut fs: [i32; 2] = [0, 1];

    loop {
        let f = fs[0] + fs[1];

        fs[0] = fs[1];
        fs[1] = f;

        println!("{}", f);
    }
}