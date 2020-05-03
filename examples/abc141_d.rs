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
    let mut pq: BinaryHeap<usize> = BinaryHeap::new();
    for _ in 0..n {
        pq.push(iterator.next().unwrap().parse().unwrap());
    }

    for _ in 0..m {
        let discounted = pq.pop().unwrap() / 2;
        pq.push(discounted);
    }

    let ans: usize = pq.iter().sum();

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
2 13 8",
            "9"
        ),
        (
            r"4 4
1 9 3 5",
            "6"
        ),
        (
            r"1 100000
1000000000",
            "0"
        ),
        (
            r"10 1
1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000 1000000000",
            "9500000000"
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