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
    let a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut c: Vec<isize> = vec![0; n];
    for i in 1..n {
        if i == 1 {
            c[i] = (a[i] - a[i - 1]).abs();
        } else {
            c[i] = min(
                c[i - 2] + (a[i] - a[i - 2]).abs(),
                c[i - 1] + (a[i] - a[i - 1]).abs()
            );
        }
    }
    return format!("{}", c.last().unwrap());
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
100 150 130 120",
            "40"
        ),
        (
            r"4
100 125 80 110",
            "40"
        ),
        (
            r"9
314 159 265 358 979 323 846 264 338",
            "310"
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