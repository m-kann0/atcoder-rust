use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let na: usize = iterator.next().unwrap().parse().unwrap();
    let nb: usize = iterator.next().unwrap().parse().unwrap();

    let mut a: HashSet<usize> = HashSet::new();
    let mut b: HashSet<usize> = HashSet::new();

    for _ in 0..na {
        a.insert(iterator.next().unwrap().parse().unwrap());
    }

    for _ in 0..nb {
        b.insert(iterator.next().unwrap().parse().unwrap());
    }

    let inter = a.intersection(&b).count();
    let union = a.union(&b).count();

    let ans = inter as f64 / union as f64;
    return format!("{:.10}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
1 3 5
1 2",
            "0.2500000000"
        ),
        (
            r"9 10
11 2 33 4 55 6 77 8 99
10 11 14 19 55 1000000000 4 5 7 8",
            "0.2666666667"
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