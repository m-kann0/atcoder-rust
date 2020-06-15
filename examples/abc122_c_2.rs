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
    let q: usize = iterator.next().unwrap().parse().unwrap();
    let s: Vec<char> = iterator.next().unwrap().chars().collect();
    let mut t: Vec<usize> = vec![0; n + 1];
    for i in 1..n {
        if s[i - 1] == 'A' && s[i] == 'C' {
            t[i + 1] = t[i] + 1;
        } else {
            t[i + 1] = t[i];
        }
    }

    let mut result = String::new();
    for _ in 0..q {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();
        result.push_str(&format!("{}\n", t[r] - t[l]));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"8 3
ACACTACG
3 7
2 3
1 8",
            "2
0
3"
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