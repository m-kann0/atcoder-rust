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

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut a: Vec<usize> = vec![0; n];
    let b0: usize = iterator.next().unwrap().parse().unwrap();
    a[0] = b0;
    a[1] = b0;
    for i in 1..(n-1) {
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        a[i + 1] = bi;
        a[i] = min(a[i], bi);
    }

    let answer = a.iter().fold(0, |acc, &x| acc + x);

    return answer.to_string();
}

#[test]
fn test() {
    assert_eq!(
        solve(
            r"6
0 153 10 10 23"
        ),
        "53"
    );
    assert_eq!(
        solve(
            r"2
3"
        ),
        "6"
    );
    assert_eq!(
        solve(
            r"3
2 5"
        ),
        "9"
    );
}
