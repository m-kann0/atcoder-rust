use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut s_ab: Vec<String> = Vec::new();
    let mut s_a: Vec<String> = Vec::new();
    let mut s_b: Vec<String> = Vec::new();
    let mut s_o: Vec<String> = Vec::new();
    for _ in 0..n {
        let s: String = iterator.next().unwrap().to_string();
        if s.starts_with('B') && s.ends_with('A') {
            s_ab.push(s);
        } else if s.ends_with('A') {
            s_a.push(s);
        } else if s.starts_with('B') {
            s_b.push(s);
        } else {
            s_o.push(s);
        }
    }

    let mut t: String = pop(&mut s_a, &mut s_ab, &mut s_b, &mut s_o);
    for _ in 0..(n - 1) {
        if t.ends_with('A') {
            t += &pop(&mut s_ab, &mut s_b, &mut s_a, &mut s_o);
        } else {
            t += &pop(&mut s_a, &mut s_ab, &mut s_b, &mut s_o);
        }
    }

    let mut count: usize = 0;
    for i in 0..(t.len() - 1) {
        if &t[i..i + 2] == "AB" {
            count += 1;
        }
    }
    return count.to_string();
}

fn pop(v1: &mut Vec<String>, v2: &mut Vec<String>, v3: &mut Vec<String>, v4: &mut Vec<String>) -> String {
    if !v1.is_empty() {
        return v1.pop().unwrap();
    }
    if !v2.is_empty() {
        return v2.pop().unwrap();
    }
    if !v3.is_empty() {
        return v3.pop().unwrap();
    }
    return v4.pop().unwrap();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
ABCA
XBAZ
BAD",
            "2"
        ),
        (
            r"9
BEWPVCRWH
ZZNQYIJX
BAVREA
PA
HJMYITEOX
BCJHMRMNK
BP
QVFABZ
PRGKSPUNA",
            "4"
        ),
        (
            r"7
RABYBBE
JOZ
BMHQUVA
BPA
ISU
MCMABAOBHZ
SZMEHMA",
            "4"
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