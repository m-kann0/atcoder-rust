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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let x: usize = iterator.next().unwrap().parse().unwrap();

    let mut c: Vec<usize> = Vec::new();
    let mut a: Vec<Vec<usize>> = vec![vec![0; m]; n];
    for i in 0..n {
        c.push(iterator.next().unwrap().parse().unwrap());
        for j in 0..m {
            a[i][j] = iterator.next().unwrap().parse().unwrap();
        }
    }

    let mut ans: usize = std::usize::MAX;
    for i in 0..(2_i64.pow(n as u32)) {
        let bin: Vec<char> = format!("{:0>1$b}", i, n).chars().collect();

        let mut cost: usize = 0;
        let mut sum: Vec<usize> = vec![0; m];
        for j in 0..n {
            if bin[j] == '0' {
                continue;
            }
            cost += c[j];
            for k in 0..m {
                sum[k] += a[j][k];
            }
        }
        if sum.iter().all(|it| *it >= x) {
            ans = min(ans, cost);
        }
    }

    if ans == std::usize::MAX {
        return "-1".to_string();
    }
    return ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3 10
60 2 2 4
70 8 7 9
50 2 3 9",
            "120"
        ),
        (
            r"3 3 10
100 3 1 4
100 1 5 9
100 2 6 5",
            "-1"
        ),
        (
            r"8 5 22
100 3 7 5 3 1
164 4 5 2 7 8
334 7 2 7 2 9
234 4 7 2 8 2
541 5 4 3 3 6
235 4 8 6 9 7
394 3 6 1 6 2
872 8 4 3 7 2",
            "1067"
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