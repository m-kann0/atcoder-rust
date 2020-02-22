use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let a: i64 = iterator.next().unwrap().parse().unwrap();
    let b: i64 = iterator.next().unwrap().parse().unwrap();
    let k: i64 = iterator.next().unwrap().parse().unwrap();

    if k >= a + b {
        return String::from("0 0")
    } else if k > a {
        return format!("{} {}", 0, b - (k - a))
    } else {
        return format!("{} {}", a - k, b)
    }
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"2 3 3"
        ),
        "0 2"
    );
    assert_eq!(
        solve(
            r"500000000000 500000000000 1000000000000"
        ),
        "0 0"
    );
}
