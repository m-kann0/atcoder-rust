use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    iterator.next();
    let s = iterator.next().unwrap();
    let vec: Vec<_> = s.match_indices("ABC").collect();

    return vec.len().to_string();
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"10
ZABCDBABCQ"
        ),
        "2"
    );
    assert_eq!(
        solve(
            r"19
THREEONEFOURONEFIVE"
        ),
        "0"
    );
    assert_eq!(
        solve(
            r"33
ABCCABCBABCCABACBCBBABCBCBCBCABCB"
        ),
        "5"
    );
}
