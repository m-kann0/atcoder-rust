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
    let mut b: Vec<Vec<isize>> = Vec::new();
    for _ in 0..n {
        let line = iterator.next().unwrap()
            .chars()
            .map(|c| c.to_string().parse::<isize>().unwrap())
            .collect();
        b.push(line);
    }

    let mut a: Vec<Vec<isize>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            if b[i][j] != 0 {
                b[i + 1][j - 1] -= b[i][j];
                b[i + 1][j + 1] -= b[i][j];
                b[i + 2][j] -= b[i][j];
                a[i + 1][j] = b[i][j];
                b[i][j] = 0;
            }
        }
    }

    let mut result = String::new();
    for i in 0..n {
        for j in 0..m {
            result.push_str(&format!("{}", a[i][j]));
        }
        result.push('\n');
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 3
010
101
010",
            "000
010
000"
        ),
        (
            r"3 4
0230
2323
0230",
            "0000
0230
0000"
        ),
        (
            r"5 5
00100
03040
20903
05060
00300",
            "00000
00100
02030
00300
00000"
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