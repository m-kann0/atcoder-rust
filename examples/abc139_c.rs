use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut count: usize = 0;
    let mut prev: usize = 0;
    let mut answer: usize = 0;

    for _i in 0..n {
        let h: usize = iterator.next().unwrap().parse().unwrap();

        if h > prev {
            answer = max(answer, count);
            count = 0;
        } else {
            count += 1;
        }
        prev = h;
    }
    answer = max(answer, count);

    return format!("{}", answer);
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"5
10 4 8 7 3"
        ),
        "2"
    );
    assert_eq!(
        solve(
            r"7
4 4 5 6 6 5 5"
        ),
        "3"
    );
    assert_eq!(
        solve(
            r"4
1 2 3 4"
        ),
        "0"
    );
}
