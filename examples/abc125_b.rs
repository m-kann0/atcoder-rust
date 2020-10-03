use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let v: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    let c: Vec<usize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut ans: isize = std::isize::MIN;
    for i in 0..2_u32.pow(n as u32) {
        let bin: Vec<char> = binary_string(i as usize, n).chars().collect();
        let mut x: usize = 0;
        let mut y: usize = 0;
        for j in 0..n {
            if bin[j] == '1' {
                x += v[j];
                y += c[j];
            }
        }
        ans = max(ans, x as isize - y as isize);
    }
    ans.to_string()
}

fn binary_string(n: usize, keta: usize) -> String {
    return format!("{:0>1$b}", n, keta);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
10 2 5
6 3 4",
            "5"
        ),
        (
            r"4
13 21 6 19
11 30 6 15",
            "6"
        ),
        (
            r"1
1
50",
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