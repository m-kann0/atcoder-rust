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
    let mut a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    a.sort();
    a.reverse();

    let mut ans: usize = 0;
    for i in 0..n {
        if i == 0 {
            continue;
        }
        ans += a[i / 2];
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
2 2 1 3",
            "7"
        ),
        (
            r"7
1 1 1 1 1 1 1",
            "6"
        ),
        (
            r"10
1 2 3 4 5 6 7 8 9 10",
            "70"
        ),
        (
            r"9
2 3 4 5 6 7 8 9 10",
            "64"
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