use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let r: isize = iterator.next().unwrap().parse().unwrap();
    let d: isize = iterator.next().unwrap().parse().unwrap();
    let mut x: isize = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();
    for _ in 0..10 {
        x = r * x - d;
        result.push_str(&format!("{}\n", x));
    }
    return result.trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 10 20",
            "30
50
90
170
330
650
1290
2570
5130
10250"
        ),
        (
            r"4 40 60",
            "200
760
3000
11960
47800
191160
764600
3058360
12233400
48933560"
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