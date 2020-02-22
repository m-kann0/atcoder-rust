use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let s = iterator.next().unwrap();

    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    let mut ans: usize = 1;

    for _i in 1..n {
        let current = chars.next().unwrap();
        if prev != current {
            prev = current;
            ans += 1;
        }
    }

    return format!("{}", ans);
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"10
aabbbbaaca"
        ),
        "5"
    );
    assert_eq!(
        solve(
            r"5
aaaaa"
        ),
        "1"
    );
    assert_eq!(
        solve(
            r"20
xxzaffeeeeddfkkkkllq"
        ),
        "10"
    );
}
