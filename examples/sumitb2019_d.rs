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
    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    let mut count = 0;
    for i in 0..1000 {
        let t: Vec<char> = format!("{:>03}", i).chars().collect();
        let mut j = 0;
        for k in 0..n {
            if j >= 3 {
                break;
            }
            if t[j] == s[k] {
                j += 1;
            }
        }
        if j == 3 {
            count += 1;
        }
    }
    return count.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
0224",
            "3"
        ),
        (
            r"6
123123",
            "17"
        ),
        (
            r"19
3141592653589793238",
            "329"
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