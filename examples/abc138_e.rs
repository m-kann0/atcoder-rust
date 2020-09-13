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

    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let t: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut map: HashMap<char, Vec<usize>> = HashMap::new();
    for i in 0..s.len() {
        map.entry(s[i]).or_insert(Vec::new()).push(i);
    }

    let mut prev: isize = -1;
    let mut offset: usize = 0;
    for i in 0..t.len() {
        if let Some(v) = map.get(&t[i]) {
            let mut ok: isize = v.len() as isize;
            let mut ng: isize = -1;
            while (ok - ng).abs() > 1 {
                let mid = (ok + ng) / 2;
                if (v[mid as usize] + offset) as isize > prev {
                    ok = mid;
                } else {
                    ng = mid;
                }
            }
            if ok == v.len() as isize {
                offset += s.len();
                prev = (v[0] + offset) as isize;
            } else {
                prev = (v[ok as usize] + offset) as isize;
            }
        } else {
            return "-1".to_string();
        }
    }
    (prev + 1).to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"contest
son",
            "10"
        ),
        (
            r"contest
programming",
            "-1"
        ),
        (
            r"contest
sentence",
            "33"
        ),
        (
            r"contest
nttst",
            "14"
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