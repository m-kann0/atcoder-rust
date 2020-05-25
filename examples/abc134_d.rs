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
    let a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut result: Vec<bool> = vec![false; n + 1];
    for i in (1..(n + 1)).rev() {
        let mut b: usize = a[i - 1];
        let mut p: usize = i + i;
        while p <= n {
            if result[p] {
                b += 1;
            }
            p += i;
        }
        result[i] = b % 2 == 1;
    }

    let mut count: usize = 0;
    let mut ans: String = String::new();
    for i in 1..(n + 1) {
        if result[i] {
            count += 1;
            ans.push_str(&format!("{} ", i));
        }
    }
    if count == 0 {
        return "0".to_string();
    }
    return format!("{}\n{}", count, ans.trim());
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 0 0",
            "1
1"
        ),
        (
            r"5
0 0 0 0 0",
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