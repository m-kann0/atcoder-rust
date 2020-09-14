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
    let mut a: Vec<isize> = (0..n).map(|_| iterator.next().unwrap().parse().unwrap()).collect();
    a.sort();

    let mut result = String::new();
    let mut x: isize = a.remove(n - 1);
    let mut y: isize = a.remove(0);
    for ai in a {
        if ai < 0 {
            result.push_str(&format!("{} {}\n", x, ai));
            x -= ai;
        } else {
            result.push_str(&format!("{} {}\n", y, ai));
            y -= ai;
        }
    }
    result.push_str(&format!("{} {}\n", x, y));
    x -= y;
    result = format!("{}\n{}", x, result);
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1 -1 2",
            "4
-1 1
2 -2"
        ),
        (
            r"3
1 1 1",
            "1
1 1
1 0"
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