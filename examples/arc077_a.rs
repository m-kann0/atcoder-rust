use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);

    for i in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        if i % 2 == 0 {
            queue.push_back(a);
        } else {
            queue.push_front(a);
        }
    }

    let mut result = String::new();

    if n % 2 == 0 {
        for v in queue.iter() {
            result.push_str(&format!("{} ", v));
        }
    } else {
        for v in queue.iter().rev() {
            result.push_str(&format!("{} ", v));
        }
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1 2 3 4",
            "4 2 1 3"
        ),
        (
            r"3
1 2 3",
            "3 1 2"
        ),
        (
            r"1
1000000000",
            "1000000000"
        ),
        (
            r"6
0 6 7 6 7 0",
            "0 6 6 0 7 7"
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