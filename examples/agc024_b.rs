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
    let mut indexes: Vec<usize> = vec![0; n];
    for i in 0..n {
        let pi: usize = iterator.next().unwrap().parse().unwrap();
        let pi = pi - 1;
        indexes[pi] = i;
    }

    let mut c_max = 0;
    let mut c = 1;
    let mut prev = indexes[0];
    for i in 1..n {
        if indexes[i] > prev {
            c += 1;
        } else {
            c_max = max(c_max, c);
            c = 1;
        }
        prev = indexes[i];
    }
    c_max = max(c_max, c);

    let ans = n - c_max;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1
3
2
4",
            "2"
        ),
        (
            r"6
3
2
5
1
4
6",
            "4"
        ),
        (
            r"8
6
3
1
2
7
4
8
5",
            "5"
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