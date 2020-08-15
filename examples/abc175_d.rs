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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let p: Vec<usize> = (0..n).map(|_| {
        let pi: usize = iterator.next().unwrap().parse().unwrap();
        pi - 1
    }).collect();
    let c: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: isize = std::isize::MIN;
    for start in 0..n {
        let mut visited: Vec<bool> = vec![false; n];
        let mut current = start;
        let mut count = 0;
        let mut score_all = 0;
        let mut score_max = std::isize::MIN;
        while count < k && !visited[p[current]] {
            current = p[current];
            visited[current] = true;
            score_all += c[current];
            score_max = max(score_max, score_all);
            count += 1;
        }

        // ループした場合
        if visited[p[current]] {
            // 1ループのスコアが正の場合
            if score_all > 0 {
                let mut score = score_all * (k / count - 1) as isize;
                let r = k % count + count;

                let mut current = start;
                let mut score_all = 0;
                let mut score_max = std::isize::MIN;
                let mut count = 0;
                while count < r {
                    current = p[current];
                    score_all += c[current];
                    score_max = max(score_max, score_all);
                    count += 1;
                }
                if score_max > 0 {
                    score += score_max;
                }
                ans = max(ans, score);
            } else {
                ans = max(ans, score_max);
            }
        } else {
            ans = max(ans, score_max);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 2
2 4 5 1 3
3 4 -10 -8 8",
            "8"
        ),
        (
            r"5 10
2 4 5 1 3
3 4 -10 5 8",
            "41"
        ),
        (
            r"2 3
2 1
10 -7",
            "13"
        ),
        (
            r"3 3
3 1 2
-1000 -2000 -3000",
            "-1000"
        ),
        (
            r"10 58
9 1 6 7 8 4 3 2 10 5
695279662 988782657 -119067776 382975538 -151885171 -177220596 -169777795 37619092 389386780 980092719",
            "29507023469"
        ),
        (
            r"5 2
2 4 5 1 3
-1 -2 -3 -4 -5",
            "-1"
        ),
        (
            r"5 10
2 4 5 1 3
-1 -2 -3 -4 -5",
            "-1"
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