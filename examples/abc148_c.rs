use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();

    let mut i_a: usize = 1;
    let mut i_b: usize = 1;

    loop {
        let mul_a = a * i_a;
        let mul_b = b * i_b;
        if mul_a == mul_b {
            return mul_a.to_string();
        } else if mul_a > mul_b {
            i_b += 1;
        } else {
            i_a += 1;
        }
    }
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"2 3"
        ),
        "6"
    );
    assert_eq!(
        solve(
            r"123 456"
        ),
        "18696"
    );
    assert_eq!(
        solve(
            r"100000 99999"
        ),
        "9999900000"
    );
}
