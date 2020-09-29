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

    if a[0] != 0 {
        return "-1".to_string();
    }

    let mut ans: usize = 0;
    let mut prev = 0;
    for i in 1..n {
        let current = a[i];
        if current < prev {
            ans += current;
        } else if current == prev {
            ans += current;
        } else if current == prev + 1 {
            ans += 1;
        } else {
            return "-1".to_string();
        }
        prev = current;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
0
1
1
2",
            "3"
        ),
        (
            r"3
1
2
1",
            "-1"
        ),
        (
            r"9
0
1
1
0
1
2
2
1
2",
            "8"
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