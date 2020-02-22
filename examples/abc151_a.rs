use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let c: char = iterator.next().unwrap().chars().nth(0).unwrap();

    return std::char::from_u32(c as u32 + 1).unwrap().to_string();
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"a"
        ),
        "b"
    );
    assert_eq!(
        solve(
            r"y"
        ),
        "z"
    );
}
