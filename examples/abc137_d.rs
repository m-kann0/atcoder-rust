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

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let mut jobs: Vec<Vec<usize>> = vec![Vec::new(); 100_005];
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        jobs[ai].push(bi);
    }

    let mut pq: BinaryHeap<usize> = BinaryHeap::new();
    let mut ans: usize = 0;
    for i in 1..=m {
        for &job in &jobs[i] {
            pq.push(job);
        }
        ans += pq.pop().unwrap_or(0);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
4 3
4 1
2 2",
            "5"
        ),
        (
            r"5 3
1 2
1 3
1 4
2 1
2 3",
            "10"
        ),
        (
            r"1 1
2 1",
            "0"
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