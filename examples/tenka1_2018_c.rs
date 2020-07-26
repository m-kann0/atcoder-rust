use std::io::Read;
use std::collections::VecDeque;
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
    let mut a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    a.sort();
    let mut ans: isize = 0;
    for i in 0..4 {
        let mut q: VecDeque<isize> = a.clone().into_iter().collect();
        let mut result: VecDeque<isize> = VecDeque::new();
        let mut count: usize = i;
        while !q.is_empty() {
            match count % 4 {
                0 => {
                    result.push_back(q.pop_front().unwrap());
                },
                1 => {
                    result.push_front(q.pop_back().unwrap());
                },
                2 => {
                    result.push_back(q.pop_back().unwrap());
                },
                _ => {
                    result.push_front(q.pop_front().unwrap());
                },
            }
            count += 1;
        }

        let mut s = 0;
        for i in 1..n {
            s += (result[i] - result[i - 1]).abs();
        }
        ans = max(ans, s);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
6
8
1
2
3",
            "21"
        ),
        (
            r"6
3
1
4
1
5
9",
            "25"
        ),
        (
            r"3
5
5
1",
            "8"
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