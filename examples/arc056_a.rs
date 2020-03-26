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

    let a: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();
    let k: isize = iterator.next().unwrap().parse().unwrap();
    let l: isize = iterator.next().unwrap().parse().unwrap();

    let ans1 = (k / l) * b + (k % l) * a;
    let ans2 = if k % l == 0 {
        (k / l) * b
    } else {
        (k / l + 1) * b
    };

    let ans = min(ans1, ans2);

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 7 10 3",
            "24"
        ),
        (
            r"4 5 11 3",
            "20"
        ),
        (
            r"3 8 3 3",
            "8"
        ),
        (
            r"3 8 2 3",
            "6"
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