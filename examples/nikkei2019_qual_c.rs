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
    let mut dishes: BinaryHeap<(isize, isize, isize)> = BinaryHeap::new();
    for _ in 0..n {
        let ai: isize = iterator.next().unwrap().parse().unwrap();
        let bi: isize = iterator.next().unwrap().parse().unwrap();
        dishes.push((ai + bi, ai, bi));
    }

    let mut takahashi: isize = 0;
    let mut aoki: isize = 0;
    for i in 0..n {
        let dish = dishes.pop().unwrap();
        if i % 2 == 0 {
            takahashi += dish.1;
        } else {
            aoki += dish.2;
        }
    }
    (takahashi - aoki).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
10 10
20 20
30 30",
            "20"
        ),
        (
            r"3
20 10
20 20
20 30",
            "20"
        ),
        (
            r"6
1 1000000000
1 1000000000
1 1000000000
1 1000000000
1 1000000000
1 1000000000",
            "-2999999997"
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