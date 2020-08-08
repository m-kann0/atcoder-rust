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
    let mut d: Vec<Vec<usize>> = vec![vec![1_000_000_000; n]; n];
    for i in 0..n {
        d[i][i] = 0;
    }
    for _ in 0..m {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ti: usize = iterator.next().unwrap().parse().unwrap();
        d[ai - 1][bi - 1] = ti;
        d[bi - 1][ai - 1] = ti;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                d[i][j] = min(d[i][j], d[i][k] + d[k][j]);
            }
        }
    }

    let mut ans: usize = std::usize::MAX;
    for i in 0..n {
        ans = min(ans, *d[i].iter().max().unwrap());
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
1 2 10
2 3 10",
            "10"
        ),
        (
            r"5 5
1 2 12
2 3 14
3 4 7
4 5 9
5 1 18",
            "26"
        ),
        (
            r"4 6
1 2 1
2 3 1
3 4 1
4 1 1
1 3 1
4 2 1",
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