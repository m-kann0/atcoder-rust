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

    let mut counts: Vec<isize> = vec![1; n];
    let mut has_red: Vec<bool> = vec![false; n];
    has_red[0] = true;

    for _ in 0..m {
        let x: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        let x = x - 1;
        let y = y - 1;

        if has_red[x] {
            has_red[y] = true;
        }
        counts[x] -= 1;
        counts[y] += 1;
        if counts[x] == 0 {
            has_red[x] = false;
        }
    }

    let ans = has_red.iter().filter(|it| **it).count();
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 2
1 2
2 3",
            r"2"
        ),
        (
            r"3 3
1 2
2 3
2 3",
            r"1"
        ),
        (
            r"4 4
1 2
2 3
4 1
3 4",
            r"3"
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