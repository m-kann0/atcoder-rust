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
    let g = gcd(w, h);
    return format!("{}:{}", w / g, h / g);
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        return x;
    }
    return gcd(y, x % y);
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4 3",
            "4:3"
        ),
        (
            r"16 9",
            "16:9"
        ),
        (
            r"28 21",
            "4:3"
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