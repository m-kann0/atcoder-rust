#![allow(non_snake_case)]

use std::io::Read;
use std::cmp::{max, min};

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut points: Vec<(isize, isize, isize)> = Vec::new();
    for _ in 0..n {
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        let zi: isize = iterator.next().unwrap().parse().unwrap();
        points.push((xi, yi, zi));
    }

    const INF: usize = 100_000_000_000;
    let mut graph: Vec<Vec<usize>> = vec![vec![INF; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let dist = (points[j].0 - points[i].0).abs() + (points[j].1 - points[i].1).abs() + max(0, points[j].2 - points[i].2);
            graph[i][j] = dist as usize;
        }
    }

    let mut dp: Vec<Vec<usize>> = vec![vec![INF; n]; 1 << n];
    dp[(1 << n) - 1][0] = 0;

    let mut S = ((1 << n) - 2) as isize;
    while S >= 0 {
        let s = S as usize;
        for v in 0..n {
            for u in 0..n {
                if s >> u & 1 == 0 {
                    dp[s][v] = min(dp[s][v], dp[s | 1 << u][u] + graph[v][u]);
                }
            }
        }
        S -= 1;
    }

    dp[0][0].to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
0 0 0
1 2 3",
            "9"
        ),
        (
            r"3
0 0 0
1 1 1
-1 -1 -1",
            "10"
        ),
        (
            r"17
14142 13562 373095
-17320 508075 68877
223606 -79774 9979
-24494 -89742 783178
26457 513110 -64591
-282842 7124 -74619
31622 -77660 -168379
-33166 -24790 -3554
346410 16151 37755
-36055 51275 463989
37416 -573867 73941
-3872 -983346 207417
412310 56256 -17661
-42426 40687 -119285
43588 -989435 -40674
-447213 -59549 -99579
45825 7569 45584",
            "6519344"
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