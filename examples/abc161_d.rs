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

    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut queue: VecDeque<usize> = VecDeque::new();
    for i in 1..10 {
        queue.push_back(i)
    }

    let mut ans: usize = 0;
    for _ in 0..k {
        ans = queue.pop_front().unwrap();

        let last = ans % 10;
        if last != 0 {
            queue.push_back(ans * 10 + last - 1);
        }
        queue.push_back(ans * 10 + last);
        if last != 9 {
            queue.push_back(ans * 10 + last + 1);
        }
    }

    return ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"15",
            "23"
        ),
        (
            r"1",
            "1"
        ),
        (
            r"13",
            "21"
        ),
        (
            r"100000",
            "3234566667"
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