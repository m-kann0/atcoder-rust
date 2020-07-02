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
    let balls: Vec<(isize, isize)> = (0..n).map(|_| {
        let x: isize = iterator.next().unwrap().parse().unwrap();
        let y: isize = iterator.next().unwrap().parse().unwrap();
        (x, y)
    }).collect();

    if n == 1 {
        return "1".to_string();
    }

    let mut ans = 50;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let p = balls[i].0 - balls[j].0;
            let q = balls[i].1 - balls[j].1;
            let mut count = 0;
            for k in 0..n {
                for l in 0..n {
                    if k == l {
                        continue;
                    }
                    if balls[l].0 - balls[k].0 == p && balls[l].1 - balls[k].1 == q {
                        count += 1;
                    }
                }
            }
            ans = min(ans, n - count);
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2
1 1
2 2",
            "1"
        ),
        (
            r"3
1 4
4 6
7 8",
            "1"
        ),
        (
            r"4
1 1
1 2
2 1
2 2",
            "2"
        ),
        (
            r"5
1 4
4 6
7 8
4 4
7 6",
            "2"
        ),
        (
            r"7
1 1
1 3
3 2
3 4
3 6
8 2
8 4",
            "3"
        ),
        (
            r"7
1 1
1 3
3 2
3 4
3 6
8 4
8 2",
            "3"
        ),
        (
            r"4
1 1
3 3
6 7
2 5",
            "3"
        ),
        (
            r"1
1 1",
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