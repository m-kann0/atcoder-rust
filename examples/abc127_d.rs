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

    let mut h: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        h.push((ai, 1));
    }
    for _ in 0..m {
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        h.push((ci, bi));
    }

    let mut ans = 0;
    for _ in 0..n {
        let p = h.pop().unwrap();
        ans += p.0;
        if p.1 > 1 {
            h.push((p.0, p.1 - 1));
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
5 1 4
2 3
1 5",
            "14"
        ),
        (
            r"10 3
1 8 5 7 100 4 52 33 13 5
3 10
4 30
1 4",
            "338"
        ),
        (
            r"3 2
100 100 100
3 99
3 99",
            "300"
        ),
        (
            r"11 3
1 1 1 1 1 1 1 1 1 1 1
3 1000000000
4 1000000000
3 1000000000",
            "10000000001"
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