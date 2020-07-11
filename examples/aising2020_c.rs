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

    let mut f: Vec<usize> = vec![0; n + 1];
    for x in 1..=100 {
        for y in 1..=100 {
            for z in 1..=100 {
                let a = x * x + y * y + z * z + x * y + y * z + z * x;
                if a <= n {
                    f[a] += 1;
                }
            }
        }
    }

    let mut result = String::new();
    for i in 1..=n {
        result.push_str(&format!("{}\n", f[i]));
    }
    result.trim().to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"20",
            "0
0
0
0
0
1
0
0
0
0
3
0
0
0
0
0
3
3
0
0"
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