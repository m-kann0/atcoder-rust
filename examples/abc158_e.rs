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
    let p: usize = iterator.next().unwrap().parse().unwrap();

    let s = iterator.next().unwrap();

    let mut memo: HashMap<String, usize> = HashMap::new();

    let mut ans: usize = 0;
    for i in 1..(n + 1) {
        for j in 0..(n - i + 1) {
            if is_mod_zero(&s[j..(j + i)], p, &mut memo) {
                ans += 1;
            }
        }
    }

    return ans.to_string();
}

fn is_mod_zero(s: &str, p: usize, memo: &mut HashMap<String, usize>) -> bool {
    let mut s: String = String::from(s);
    if let Some(x) = memo.get(&s) {
        return x % p == 0;
    }

    while s.len() >= 20 {
        let a1: usize = *&s[0..19].parse::<usize>().unwrap();
        let m1 = a1 % p;
        s = format!("{}{}", m1, &s[19..]);
    }

    let mo = s.parse::<usize>().unwrap() % p;
    memo.insert(s, mo);
    return mo == 0;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3
3543",
            "6"
        ),
        (
            r"4 2
2020",
            "10"
        ),
        (
            r"20 11
33883322005544116655",
            "68"
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