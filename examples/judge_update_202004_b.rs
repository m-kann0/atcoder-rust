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
    let mut reds: Vec<usize> = Vec::new();
    let mut blues: Vec<usize> = Vec::new();
    for _ in 0..n {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let c = iterator.next().unwrap();
        if c == "R" {
            reds.push(x);
        } else {
            blues.push(x);
        }
    }
    reds.sort();
    blues.sort();

    let mut result = String::new();
    for r in reds {
        result.push_str(&format!("{}\n", r));
    }
    for b in blues {
        result.push_str(&format!("{}\n", b));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
10 B
6 R
2 R
4 B",
            "2
6
4
10"
        ),
        (
            r"2
5 B
7 B",
            "5
7"
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