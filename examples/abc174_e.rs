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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    let mut sum: usize = 0;
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        a.push(ai);
        sum += ai;
    }

    let mut kl = k;
    let mut logs: BinaryHeap<(usize, usize, usize)> = BinaryHeap::new();
    for i in 0..n {
        let ai = a[i];
        let l = ai * k / sum;
        logs.push(((ai + l) / (l + 1), ai, l + 1));
        kl -= l;
    }

    for _ in 0..kl {
        let mut log = logs.pop().unwrap();
        log.2 += 1;
        log.0 = (log.1 + log.2 - 1) / log.2;
        logs.push(log);
    }

    let ans = logs.pop().unwrap().0;
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3
7 9",
            "4"
        ),
        (
            r"3 0
3 4 5",
            "5"
        ),
        (
            r"10 10
158260522 877914575 602436426 24979445 861648772 623690081 433933447 476190629 262703497 211047202",
            "292638192"
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