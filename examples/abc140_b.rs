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
    let b: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let c: Vec<usize> = (0..(n - 1)).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: usize = 0;
    for i in 0..n {
        ans += b[a[i] - 1];
        if i >= 1 && a[i - 1] + 1 == a[i] {
            ans += c[a[i - 1] - 1];
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3 1 2
2 5 4
3 6",
            "14"
        ),
        (
            r"4
2 3 4 1
13 5 8 24
45 9 15",
            "74"
        ),
        (
            r"2
1 2
50 50
50",
            "150"
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