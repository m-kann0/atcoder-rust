use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u64 = iterator.next().unwrap().parse().unwrap();

    let x_max: u64 = (n as f64).sqrt() as u64;

    let mut ans = n - 1;

    let mut x: u64 = 2;
    while x <= x_max {
        if n % x != 0 {
            x += 1;
            continue;
        }
        let y: u64 = n / x;
        if x > y {
            break;
        }
        ans = min(ans, x + y - 2);
        x += 1;
    }

    return format!("{}", ans);
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"10"
        ),
        "5"
    );
    assert_eq!(
        solve(
            r"50"
        ),
        "13"
    );
    assert_eq!(
        solve(
            r"10000000019"
        ),
        "10000000018"
    );
}
