use std::io::Read;
use std::collections::BinaryHeap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let k: usize = iterator.next().unwrap().parse().unwrap();
    let t: usize = iterator.next().unwrap().parse().unwrap();
    let mut pq: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    for i in 1..(t + 1) {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        pq.push((a, i));
    }

    let mut ans: usize = 0;
    let mut prev: usize = 0;
    for _ in 0..k {
        let mut first = pq.pop().unwrap();
        if first.1 != prev {
            prev = first.1;
            first.0 -= 1;
            if first.0 > 0 {
                pq.push(first);
            }
        } else {
            if let Some(mut second) = pq.pop() {
                prev = second.1;
                second.0 -= 1;
                if second.0 > 0 {
                    pq.push(second);
                }
                pq.push(first);
            } else {
                prev = first.1;
                first.0 -= 1;
                if first.0 > 0 {
                    pq.push(first);
                }
                ans += 1;
            }
        }
    }
    return format!("{}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7 3
3 2 2",
            "0"
        ),
        (
            r"6 3
1 4 1",
            "1"
        ),
        (
            r"100 1
100",
            "99"
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