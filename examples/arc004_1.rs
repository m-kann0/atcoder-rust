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
    let mut x: Vec<isize> = Vec::new();
    let mut y: Vec<isize> = Vec::new();
    for _ in 0..n {
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        x.push(xi);
        y.push(yi);
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let a = (x[i] - x[j]).abs().pow(2) + (y[i] - y[j]).abs().pow(2);
            ans = max(ans, a);
        }
    }

    let ans = (ans as f64).sqrt();
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 1
2 4
4 3",
            r"3.605551",
        ),
        (
            r"10
1 8
4 0
3 7
2 4
5 9
9 1
6 2
0 2
8 6
7 8",
            r"10.630146",
        ),
        (
            r"4
0 0
0 100
100 0
100 100",
            r"141.421356",
        ),
        (
            r"5
3 0
1 0
0 0
4 0
2 0",
            r"4.000000",
        ),
        (
            r"4
2 2
0 0
1 1
3 3",
            r"4.242641",
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