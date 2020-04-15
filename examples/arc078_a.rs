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
    let a: Vec<isize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    let mut b: Vec<isize> = vec![0; n];
    b[0] = a[0];
    for i in 1..n {
        b[i] = b[i - 1] + a[i];
    }

    let mut c: Vec<isize> = vec![0; n];
    c[n - 1] = a[n - 1];
    for i in (0..(n - 1)).rev() {
        c[i] = c[i + 1] + a[i];
    }

    let mut ans: isize = std::isize::MAX;
    for i in 0..(n - 1) {
        let x = b[i];
        let y = c[i + 1];
        let d = if x > y { x - y } else { y - x };
        ans = min(ans, d);
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
1 2 3 4 5 6",
            "1"
        ),
        (
            r"2
10 -10",
            "20"
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