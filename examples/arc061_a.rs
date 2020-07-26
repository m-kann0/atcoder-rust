use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    if s.len() == 1 {
        return s[0].to_string()
    }

    let mut ans: usize = 0;
    for i in 0..(1 << s.len() - 1) {
        let bin: Vec<char> = binary_string(i, s.len() - 1).chars().collect();
        let mut current = 0;
        for j in 0..s.len() {
            current += s[j].to_digit(10).unwrap() as usize;
            if j == s.len() - 1 || bin[j] == '1' {
                ans += current;
                current = 0;
            } else {
                current *= 10;
            }
        }
    }
    ans.to_string()
}

fn binary_string(n: usize, keta: usize) -> String {
    return format!("{:0>1$b}", n, keta);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"125",
            "176"
        ),
        (
            r"9999999999",
            "12656242944"
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