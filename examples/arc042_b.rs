use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let x: f64 = iterator.next().unwrap().parse().unwrap();
    let y: f64 = iterator.next().unwrap().parse().unwrap();
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let points: Vec<(f64, f64)> = (0..n)
        .map(|_| {
            let xi: f64 = iterator.next().unwrap().parse().unwrap();
            let yi: f64 = iterator.next().unwrap().parse().unwrap();
            (xi, yi)
        })
        .collect();

    let mut ans: f64 = 1_000_000_000_f64;
    for i in 0..n {
        let b = points[i];
        let c = if i + 1 < n {
            points[i + 1]
        } else {
            points[0]
        };

        let b_len = ((x - c.0) * (x - c.0) + (y - c.1) * (y - c.1)).sqrt();
        let c_len = ((x - b.0) * (x - b.0) + (y - b.1) * (y - b.1)).sqrt();
        let a_len = ((b.0 - c.0) * (b.0 - c.0) + (b.1 - c.1) * (b.1 - c.1)).sqrt();

        let d = c_len * (
            (a_len * a_len + c_len * c_len - b_len * b_len) / (2.0 * a_len * c_len)
        ).acos().sin();
        ans = ans.min(b_len);
        ans = ans.min(c_len);
        ans = ans.min(d);
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"0 0
4
100 100
-100 100
-100 -100
100 -100",
            "100"
        ),
        (
            r"10 10
3
0 100
-100 -100
100 -100",
            "31.3049516850"
        ),
        (
            r"34 6
7
-43 -65
-23 -99
54 -68
65 92
16 83
-18 43
-39 2",
            "25.0284205314"
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