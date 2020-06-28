fn main() {
    let mut n: usize = 42;
    while n < 130000000 {
        n += n;
    }
    println!("{}", n);
}