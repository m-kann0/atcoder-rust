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
    let l: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: usize = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i >= j || j >= k {
                    continue;
                }
                if l[i] == l[j] || l[j] == l[k] || l[k] == l[i] {
                    continue;
                }
                if l[i] < l[j] + l[k] && l[j] < l[k] + l[i] && l[k] < l[i] + l[j] {
                    ans += 1;
                }
            }
        }
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
4 4 9 7 5",
            "5"
        ),
        (
            r"6
4 5 4 3 3 5",
            "8"
        ),
        (
            r"10
9 4 6 1 9 6 10 6 6 8",
            "39"
        ),
        (
            r"2
1 1",
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