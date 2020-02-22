use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: u32 = iterator.next().unwrap().parse().unwrap();

    let mut map = HashMap::new();
    for i in 1..(n + 1) {
        let pair = pair(i);
        let count = map.entry(pair).or_insert(0_u32);
        *count += 1;
    }

    let mut ans: u64 = 0;
    for i in 1..(n + 1) {
        let pair = pair(i);
        let count = map.get(&(pair.1, pair.0));
        if let Some(u) = count {
            ans += *u as u64
        }
    }

    return ans.to_string();
}

fn pair(i: u32) -> (u32, u32) {
    let last: u32 = i % 10;
    let first: u32 = i.to_string().chars().nth(0).unwrap().to_digit(10).unwrap();
    return (first, last);
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"25"
        ),
        "17"
    );
    assert_eq!(
        solve(
            r"1"
        ),
        "1"
    );
    assert_eq!(
        solve(
            r"100"
        ),
        "108"
    );
    assert_eq!(
        solve(
            r"200000"
        ),
        "400000008"
    );
}
