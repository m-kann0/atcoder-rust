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
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut map: HashMap<usize, usize> = HashMap::new();
    for _i in 0..m {
        let s: usize = iterator.next().unwrap().parse().unwrap();
        let c: usize = iterator.next().unwrap().parse().unwrap();
        if n != 1 && s == 1 && c == 0 {
            return "-1".to_string();
        }

        let old = map.insert(s, c);
        if let Some(x) = old {
            if x != c {
                return "-1".to_string();
            }
        }
    }

    let mut ans: usize = 0;
    for i in 1..(n + 1) {
        ans *= 10;

        let value = map.get(&i);
        ans += match value {
            Some(&x) => x,
            _ => if i == 1 && n != 1 { 1 } else { 0 }
        };
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
1 7
3 2
1 7",
            "702"
        ),
        (
            r"3 2
2 1
2 3",
            "-1"
        ),
        (
            r"3 1
1 0",
            "-1"
        ),
        (
            r"3 0",
            "100"
        ),
        (
            r"1 1
1 0",
            "0"
        ),
        (
            r"1 0",
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