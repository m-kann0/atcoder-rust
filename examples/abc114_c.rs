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

    let nums: Vec<usize> = vec![3, 5, 7];

    let mut ans: usize = 0;
    let mut q: VecDeque<usize> = VecDeque::from(nums.clone());
    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        if current > n {
            break;
        }
        let cs = current.to_string();
        if cs.contains('3') && cs.contains('5') && cs.contains('7') {
            ans += 1;
        }
        for num in &nums {
            let next = current * 10 + *num;
            q.push_back(next);
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"575",
            "4"
        ),
        (
            r"3600",
            "13"
        ),
        (
            r"999999999",
            "26484"
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