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
    let mut a: Vec<usize> = Vec::with_capacity(n);
    let mut b: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(iterator.next().unwrap().parse().unwrap());
        b.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut ans = 0;
    for i in (0..n).rev() {
        let x = baisu(a[i] + ans, b[i]);
        ans += x - (a[i] + ans);
    }

    return ans.to_string()
}

fn baisu(a: usize, b: usize) -> usize {
    if a == 0 {
        return 0;
    }
    if b >= a {
        return b;
    }
    if a % b == 0 {
        a
    } else {
        b * (a / b + 1)
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
3 5
2 7
9 4",
            "7"
        ),
        (
            r"7
3 1
4 1
5 9
2 6
5 3
5 8
9 7",
            "22"
        ),
        (
            r"3
0 5
0 7
0 4",
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