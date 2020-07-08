use std::io::Read;

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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let mut b: Vec<usize> = vec![1; n];

    for i in 1..n {
        if a[i] > a[i - 1] {
            b[i] = b[i - 1] + 1;
        } else {
            b[i] = 1;
        }
    }

    let ans: usize = b.iter().filter(|it| **it >= k).count();
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 4
100
300
600
700
800
400
500
800
900
900",
            r"3"
        ),
        (
            r"10 3
10
40
50
80
90
30
20
40
90
95",
            r"5"
        ),
        (
            r"8 4
1
2
3
4
5
6
7
8",
            r"5"
        ),
        (
            r"8 2
100000
90000
50000
30000
10000
4000
200
1",
            r"0"
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