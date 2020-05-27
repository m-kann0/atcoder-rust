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

    let h: usize = iterator.next().unwrap().parse().unwrap();
    let w: usize = iterator.next().unwrap().parse().unwrap();
    let mut c: Vec<Vec<usize>> = vec![vec![0; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            c[i][j] = iterator.next().unwrap().parse().unwrap();
        }
    }
    let mut a: Vec<usize> = Vec::new();
    for _ in 0..(h * w) {
        let ai: isize = iterator.next().unwrap().parse().unwrap();
        if ai >= 0 {
            a.push(ai as usize);
        }
    }

    // ワーシャルフロイド法
    let mut d: Vec<Vec<usize>> = c.clone();
    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }

    let mut ans = 0;
    for ai in a {
        ans += d[ai][1];
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 4
0 9 9 9 9 9 9 9 9 9
9 0 9 9 9 9 9 9 9 9
9 9 0 9 9 9 9 9 9 9
9 9 9 0 9 9 9 9 9 9
9 9 9 9 0 9 9 9 9 2
9 9 9 9 9 0 9 9 9 9
9 9 9 9 9 9 0 9 9 9
9 9 9 9 9 9 9 0 9 9
9 9 9 9 2 9 9 9 0 9
9 2 9 9 9 9 9 9 9 0
-1 -1 -1 -1
8 1 1 8",
            "12"
        ),
        (
            r"5 5
0 999 999 999 999 999 999 999 999 999
999 0 999 999 999 999 999 999 999 999
999 999 0 999 999 999 999 999 999 999
999 999 999 0 999 999 999 999 999 999
999 999 999 999 0 999 999 999 999 999
999 999 999 999 999 0 999 999 999 999
999 999 999 999 999 999 0 999 999 999
999 999 999 999 999 999 999 0 999 999
999 999 999 999 999 999 999 999 0 999
999 999 999 999 999 999 999 999 999 0
1 1 1 1 1
1 1 1 1 1
1 1 1 1 1
1 1 1 1 1
1 1 1 1 1",
            "0"
        ),
        (
            r"3 5
0 4 3 6 2 7 2 5 3 3
4 0 5 3 7 5 3 7 2 7
5 7 0 7 2 9 3 2 9 1
3 6 2 0 2 4 6 4 2 3
3 5 7 4 0 6 9 7 6 7
9 8 5 2 2 0 4 7 6 5
5 4 6 3 2 3 0 5 4 3
3 6 2 3 4 2 4 0 8 9
4 6 5 4 3 5 3 2 0 8
2 1 3 4 5 7 8 6 4 0
3 5 2 6 1
2 5 3 2 1
6 9 2 5 6",
            "47"
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