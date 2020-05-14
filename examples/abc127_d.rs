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

    let mut a: BinaryHeap<isize> = BinaryHeap::new();
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        a.push(-(ai as isize));
    }

    let mut vec: Vec<(usize, usize)> = Vec::new();
    for _ in 0..m {
        let bi: usize = iterator.next().unwrap().parse().unwrap();
        let ci: usize = iterator.next().unwrap().parse().unwrap();
        vec.push((ci, bi));
    }

    vec.sort();

    while !vec.is_empty() {
        let mut v = vec.pop().unwrap();
        while v.1 > 0 {
            let ai = (-a.pop().unwrap()) as usize;
            if ai >= v.0 {
                a.push(-(ai as isize));
                break;
            }
            a.push(-(v.0 as isize));
            v.1 -= 1;
        }
    }

    let ans: usize = a.iter().map(|it| (-it) as usize).sum();
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