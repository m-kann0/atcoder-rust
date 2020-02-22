use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: u32 = iterator.next().unwrap().parse().unwrap();
    let x: u32 = iterator.next().unwrap().parse().unwrap();

    return if k * 500 >= x {
        String::from("Yes")
    } else {
        String::from("No")
    }
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"2 900"
        ),
        "Yes"
    );
    assert_eq!(
        solve(
            r"1 501"
        ),
        "No"
    );
    assert_eq!(
        solve(
            r"4 2000"
        ),
        "Yes"
    );
}
