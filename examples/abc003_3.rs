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
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut r: Vec<usize> = (0..n)
        .map(|_| iterator.next().unwrap().parse().unwrap())
        .collect();

    r.sort();

    let mut ans: f64 = 0.0;
    for i in 0..n {
        if i < n - k {
            continue;
        }
        ans = (ans + r[i] as f64) / 2.0;
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 2
1000 1500",
            r"1000.000000"
        ),
        (
            r"2 1
1000 1500",
            r"750"
        ),
        (
            r"10 5
2604 2281 3204 2264 2200 2650 2229 2461 2439 2211",
            r"2820.031250000"
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
