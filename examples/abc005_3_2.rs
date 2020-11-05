use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let t: isize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let m: usize = iterator.next().unwrap().parse().unwrap();
    let b: Vec<isize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut used: Vec<bool> = vec![false; n];
    for i in 0..m {
        let mut can_sell = false;
        for j in 0..n {
            if a[j] + t >= b[i] && a[j] <= b[i] && !used[j] {
                used[j] = true;
                can_sell = true;
                break;
            }
        }
        if !can_sell {
            return "no".to_string();
        }
    }
    "yes".to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1
3
1 2 3
3
2 3 4
",
            r"yes",
        ),
        (
            r"1
3
1 2 3
3
2 3 5
",
            r"no",
        ),
        (
            r"1
3
1 2 3
10
1 2 3 4 5 6 7 8 9 10
",
            r"no",
        ),
        (
            r"1
3
1 2 3
3
1 2 2
",
            r"no",
        ),
        (
            r"2
5
1 3 6 10 15
3
4 8 16
",
            r"yes",
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