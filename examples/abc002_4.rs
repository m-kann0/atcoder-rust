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
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut friend: Vec<Vec<bool>> = vec![vec![false; n]; n];
    for _ in 0..m {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        friend[x - 1][y - 1] = true;
        friend[y - 1][x - 1] = true;
    }

    let mut ans: usize = 1;
    for bit in 0..(1<<n) {
        let mut can_group = true;
        let mut count: usize = 0;
        for i in 0..n {
            if bit & 1 << i == 0 {
                continue;
            }
            count += 1;
            for j in 0..n {
                if i == j {
                    continue;
                }
                if bit & 1 << j == 0 {
                    continue;
                }
                if !friend[i][j] {
                    can_group = false;
                }
            }
        }
        if can_group {
            ans = max(ans, count);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 3
1 2
2 3
1 3",
            r"3",
        ),
        (
            r"5 3
1 2
2 3
3 4",
            r"2",
        ),
        (
            r"7 9
1 2
1 3
2 3
4 5
4 6
4 7
5 6
5 7
6 7",
            r"4",
        ),
        (
            r"12 0",
            r"1",
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