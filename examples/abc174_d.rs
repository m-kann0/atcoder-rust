use std::io::Read;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let c: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut r: BinaryHeap<usize> = BinaryHeap::new();
    let mut w: BinaryHeap<Reverse<usize>> = BinaryHeap::new();
    for i in 0..n {
        if c[i] == 'R' {
            r.push(i);
        } else {
            w.push(Reverse(i));
        }
    }

    if r.is_empty() || w.is_empty() {
        return "0".to_string();
    }

    let mut ans: usize = 0;
    loop {
        let r_max = r.pop().unwrap();
        let w_min = w.pop().unwrap().0;
        if r_max > w_min {
            r.push(w_min);
            w.push(Reverse(r_max));
            ans += 1;
        } else {
            break;
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
WWRR",
            "2"
        ),
        (
            r"2
RR",
            "0"
        ),
        (
            r"8
WRWWRWRR",
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