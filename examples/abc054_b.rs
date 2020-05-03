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
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut a: Vec<Vec<char>> = Vec::new();
    let mut b: Vec<Vec<char>> = Vec::new();

    for _ in 0..n {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        a.push(line);
    }
    for _ in 0..m {
        let line: Vec<char> = iterator.next().unwrap().chars().collect();
        b.push(line);
    }

    let mut ans = false;
    for i in 0..(n - m + 1) {
        for j in 0..(n - m + 1) {
            let mut matched = true;
            for k in 0..m {
                for l in 0..m {
                    if b[k][l] != a[i + k][j + l] {
                        matched = false;
                    }
                }
            }
            if matched {
                ans = true;
            }
        }
    }

    return if ans {
        "Yes".to_string()
    } else {
        "No".to_string()
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
#.#
.#.
#.#
#.
.#",
            "Yes"
        ),
        (
            r"4 1
....
....
....
....
#",
            "No"
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