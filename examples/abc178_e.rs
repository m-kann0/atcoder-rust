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
    let mut points: Vec<(isize, isize)> = Vec::new();
    for _ in 0..n {
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        points.push((xi, yi));
    }

    let mx0 = points.iter().map(|p| p.0 - p.1).max().unwrap();
    let mn0 = points.iter().map(|p| p.0 - p.1).min().unwrap();
    let mx1 = points.iter().map(|p| p.0 + p.1).max().unwrap();
    let mn1 = points.iter().map(|p| p.0 + p.1).min().unwrap();

    let ans = max(mx0 - mn0, mx1 - mn1);
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 1
2 4
3 2",
            "4"
        ),
        (
            r"4
1 1
2 4
3 2
6 6",
            "10"
        ),
        (
            r"2
1 1
1 1",
            "0"
        ),
    ];

    let mut all_ok = true;
    for (i, case) in cases.iter().enumerate() {
        print!("case {} : ", i);

        let expected = case.1;
        let actual = solve(case.0);

        if expected == actual {
            println!("OK");
        } else {
            println!("NG");
            println!("    Expected: {}", expected);
            println!("    Actual  : {}", actual);

            all_ok = false;
        }
    }

    assert_eq!(all_ok, true);
}