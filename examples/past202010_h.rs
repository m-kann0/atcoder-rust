use std::io::Read;
use std::cmp::{min, max};

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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut map: Vec<Vec<usize>> = Vec::new();
    for _ in 0..n {
        let line: Vec<usize> = iterator.next().unwrap().chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        map.push(line);
    }

    let mut ans: usize = 1;
    for a in 2..=min(n, m) {
        for si in 0..=(n - a) {
            for sj in 0..=(m - a) {
                let mut count: Vec<usize> = vec![0; 10];
                for di in 0..a {
                    for dj in 0..a {
                        count[map[si + di][sj + dj]] += 1;
                    }
                }
                let sum: usize = count.iter().sum();
                let mx: usize = *count.iter().max().unwrap();
                if sum - mx <= k {
                    ans = max(ans, a);
                }
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4 3
1123
1214
1810",
            "3"
        ),
        (
            r"8 6 40
846444
790187
264253
967004
578258
204367
681998
034243",
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