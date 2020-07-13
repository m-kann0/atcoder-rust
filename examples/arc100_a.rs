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
    let a: Vec<i64> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut c: Vec<i64> = a.iter().enumerate().map(|x| x.1 - (x.0 + 1) as i64).collect();
    c.sort();

    let ans: i64 = c.iter().map(|x| (x - c[n / 2]).abs()).sum::<i64>();
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2 2 3 5 5",
            "2"
        ),
        (
            r"9
1 2 3 4 5 6 7 8 9",
            "0"
        ),
        (
            r"6
6 5 4 3 2 1",
            "18"
        ),
        (
            r"7
1 1 1 1 2 3 4",
            "6"
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