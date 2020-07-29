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
    let h: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: usize = 0;
    let mut s = 0;
    let mut is_down = false;
    for i in 1..n {
        if h[i - 1] > h[i] {
            is_down = true;
        }
        if is_down && h[i - 1] < h[i] {
            is_down = false;
            ans = max(ans, i - s);
            s = i - 1;
        }
    }
    ans = max(ans, n - s);
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
4
5
1
6
9
7",
            "4"
        ),
        (
            r"7
90
70
50
30
20
10
5",
            "7"
        ),
        (
            r"3
1
2
3",
            "3"
        ),
        (
            r"1
1",
            "1"
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