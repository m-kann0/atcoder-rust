use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let w: usize = iterator.next().unwrap().parse().unwrap();
    let h: usize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut matrix: Vec<Vec<usize>> = vec![vec![1; w]; h];

    for _ in 0..n {
        let xi: usize = iterator.next().unwrap().parse().unwrap();
        let yi: usize = iterator.next().unwrap().parse().unwrap();
        let a: u8 = iterator.next().unwrap().parse().unwrap();
        for y in 0..h {
            for x in 0..w {
                if a == 1 && x < xi {
                    matrix[y][x] = 0;
                } else if a == 2 && x >= xi {
                    matrix[y][x] = 0;
                } else if a == 3 && y < yi {
                    matrix[y][x] = 0;
                } else if a == 4 && y >= yi {
                    matrix[y][x] = 0;
                }
            }
        }
    }

    let mut ans: usize = 0;
    for y in 0..h {
        for x in 0..w {
            ans += matrix[y][x];
        }
    }
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5 4 2
2 1 1
3 3 4",
            "9"
        ),
        (
            r"5 4 3
2 1 1
3 3 4
1 4 2",
            "0"
        ),
        (
            r"10 10 5
1 6 1
4 1 3
6 9 4
9 4 2
3 1 3",
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