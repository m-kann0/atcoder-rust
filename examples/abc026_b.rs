use std::io::Read;
use std::f64::consts::PI;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut r: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    r.sort();
    r.reverse();

    let mut ans: isize = 0;
    for i in 0..n {
        if i % 2 == 0 {
            ans += (r[i] * r[i]) as isize;
        } else {
            ans -= (r[i] * r[i]) as isize;
        }
    }

    let ans: f64 = ans as f64 * PI;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1
2
3",
            "18.8495559215"
        ),
        (
            r"6
15
2
3
7
6
9",
            "508.938009881546"
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