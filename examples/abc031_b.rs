use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let l: isize = iterator.next().unwrap().parse().unwrap();
    let h: isize = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();

    let mut result = String::new();
    for i in 0..n {
        if a[i] > h {
            result.push_str(&format!("{}\n", -1));
        } else if a[i] >= l {
            result.push_str(&format!("{}\n", 0));
        } else {
            result.push_str(&format!("{}\n", l - a[i]));
        }
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"300 400
3
240
350
480",
            "60
0
-1"
        ),
        (
            r"50 80
5
10000
50
81
80
0",
            "-1
0
-1
0
50"
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