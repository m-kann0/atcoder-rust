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
    let mut sum: Vec<isize> = vec![0; 1_000_001];
    for _ in 0..n {
        let a: usize = iterator.next().unwrap().parse().unwrap();
        let b: usize = iterator.next().unwrap().parse().unwrap();
        sum[a] += 1;
        if b + 1 <= 1_000_000 {
            sum[b + 1] -= 1;
        }
    }
    for i in 1..1_000_001 {
        sum[i] += sum[i - 1];
    }

    let ans = sum.iter().max().unwrap();
    return format!("{}", ans);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
0 2
2 3
2 4
5 6",
            "3"
        ),
        (
            r"4
1000000 1000000
1000000 1000000
0 1000000
1 1000000",
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