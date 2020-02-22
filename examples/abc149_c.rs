use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut x: usize = iterator.next().unwrap().parse().unwrap();

    if x == 2 {
        return String::from("2")
    }

    if x % 2 == 0 {
        x += 1
    }

    loop {
        if is_prime(x) {
            return x.to_string();
        }
        x += 2;
    }
}

fn is_prime(x: usize) -> bool {
    let mut n = 3;

    while n < x / 2 {
        if x % n == 0 {
            return false;
        }
        n += 2;
    }
    return true;
}


#[test]
fn test() {
    assert_eq!(
        solve(
            r"20"
        ),
        "23"
    );
    assert_eq!(
        solve(
            r"2"
        ),
        "2"
    );
    assert_eq!(
        solve(
            r"99992"
        ),
        "100003"
    );
}
