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
    let mut x: usize = iterator.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    a.sort();

    let mut ans = 0;
    for i in 0..n {
        if x >= a[i] {
            x -= a[i];
            ans += 1;
        } else {
            break;
        }
    }
    if ans == n && x > 0 {
        ans -= 1;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 70
20 30 10",
            "2"
        ),
        (
            r"3 10
20 30 10",
            "1"
        ),
        (
            r"4 1111
1 10 100 1000",
            "4"
        ),
        (
            r"2 10
20 20",
            "0"
        ),
        (
            r"100 263
5 1 6 5 1 3 10 4 9 3 3 8 10 5 10 10 10 4 5 9 3 8 1 5 7 2 1 1 1 9 6 5 4 6 2 2 8 10 5 1 5 5 5 1 2 9 2 10 9 4 3 2 1 5 4 2 4 2 10 1 7 1 9 4 1 4 8 7 1 8 3 4 10 2 8 7 3 6 6 8 8 6 3 10 8 10 5 1 3 9 8 9 7 7 9 6 5 5 6 1",
            "70"
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