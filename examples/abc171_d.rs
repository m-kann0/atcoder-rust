use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut counts: HashMap<isize, isize> = HashMap::new();
    let mut ans: isize = 0;
    for _ in 0..n {
        let ai: isize = iterator.next().unwrap().parse().unwrap();
        ans += ai;
        *counts.entry(ai).or_insert(0) += 1;
    }

    let mut result = String::new();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..q {
        let bi: isize = iterator.next().unwrap().parse().unwrap();
        let ci: isize = iterator.next().unwrap().parse().unwrap();
        if let Some(x) = counts.get(&bi) {
            ans += (ci - bi) * *x;
            *counts.entry(ci).or_insert(0) += *x;
            counts.remove(&bi);
        }
        result.push_str(&format!("{}\n", ans));
    }

    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
1 2 3 4
3
1 2
3 4
2 4",
            "11
12
16"
        ),
        (
            r"4
1 1 1 1
3
1 2
2 1
3 5",
            "8
4
4"
        ),
        (
            r"2
1 2
3
1 100
2 100
100 1000",
            "102
200
2000"
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