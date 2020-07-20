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

    let r: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();
    let d: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<Vec<usize>> = Vec::new();
    for _ in 0..r {
        let mut line: Vec<usize> = Vec::new();
        for _ in 0..c {
            let aij: usize = iterator.next().unwrap().parse().unwrap();
            line.push(aij);
        }
        a.push(line);
    }

    let mut ans: usize = 0;
    for i in 0..r {
        for j in 0..c {
            if i + j <= d && (i + j) % 2 == d % 2 {
                ans = max(ans, a[i][j]);
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2 1
9 5
3 1
8 9",
            "5"
        ),
        (
            r"4 4 100
999 999 999 999
999 999 999 999
999 999 999 999
999 999 999 999",
            "999"
        ),
        (
            r"3 4 5
700 198 700 198
198 700 198 700
700 198 700 198",
            "198"
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