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

    let mut ans: usize = 1;
    const MOD: usize = 1_000_000_007;
    let mut counts = vec![0_usize; n];
    for i in 0..n {
        if a[i] == 0 {
            ans *= 3 - counts[a[i]];
        } else {
            ans *= counts[a[i] - 1] - counts[a[i]];
        }
        counts[a[i]] += 1;
        ans %= MOD;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"6
0 1 2 3 4 5",
            "3"
        ),
        (
            r"3
0 0 0",
            "6"
        ),
        (
            r"54
0 0 1 0 1 2 1 2 3 2 3 3 4 4 5 4 6 5 7 8 5 6 6 7 7 8 8 9 9 10 10 11 9 12 10 13 14 11 11 12 12 13 13 14 14 15 15 15 16 16 16 17 17 17",
            "115295190"
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