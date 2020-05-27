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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let x: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    const INF: isize = std::isize::MAX;
    let mut ans = INF;
    for i in 0..n {
        let j = i + k - 1;
        if j >= n {
            break;
        }
        let cost = (x[j] - x[i]).abs();
        ans = min(ans, x[i].abs() + cost);
        ans = min(ans, x[j].abs() + cost);
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
-30 -10 10 20 50",
            "40"
        ),
        (
            r"3 2
10 20 30",
            "20"
        ),
        (
            r"1 1
0",
            "0"
        ),
        (
            r"8 5
-9 -7 -4 -3 1 2 3 4",
            "10"
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