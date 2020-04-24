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
    let t: i64 = iterator.next().unwrap().parse().unwrap();

    let mut d: Vec<i64> = Vec::new();
    let mut a_sum: i64 = 0;
    for _ in 0..n {
        let a: i64 = iterator.next().unwrap().parse().unwrap();
        let b: i64 = iterator.next().unwrap().parse().unwrap();
        d.push(a - b);
        a_sum += a;
    }

    d.sort();

    let mut count = 0;
    while !d.is_empty() && a_sum > t {
        a_sum -= d.pop().unwrap();
        count += 1;
    }

    if a_sum > t {
        return "-1".to_string();
    }

    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 7
1 0
3 0
5 0
2 0
4 0",
            "2"
        ),
        (
            r"1 1000000000
5 0",
            "0"
        ),
        (
            r"1 0
100 99",
            "-1"
        ),
        (
            r"3 11
5 2
6 4
7 3",
            "2"
        ),
        (
            r"6 92
31 4
15 9
26 5
35 8
97 9
32 3",
            "3"
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