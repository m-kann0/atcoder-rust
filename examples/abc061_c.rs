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
    let k: usize = iterator.next().unwrap().parse().unwrap();

    let mut map: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        *map.entry(a).or_insert(0) += b;
    }

    let mut keys: Vec<usize> = map.keys().map(|it| *it).collect();
    keys.sort();

    let mut sum: usize = 0;
    for key in keys {
        let count = map.get(&key).unwrap();
        sum += *count;
        if sum >= k {
            return key.to_string();
        }
    }

    unreachable!();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4
1 1
2 2
3 3",
            "3"
        ),
        (
            r"10 500000
1 100000
1 100000
1 100000
1 100000
1 100000
100000 100000
100000 100000
100000 100000
100000 100000
100000 100000",
            "1"
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