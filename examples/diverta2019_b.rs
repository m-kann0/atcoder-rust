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
    let g: isize = iterator.next().unwrap().parse().unwrap();
    let b: isize = iterator.next().unwrap().parse().unwrap();
    let n: isize = iterator.next().unwrap().parse().unwrap();

    let mut ans = 0;
    for i in 0..(n / r + 1) {
        for j in 0..(n / g + 1) {
            let k1 = (n - i * r - j * g) / b;
            let k2 = (n - i * r - j * g) % b;
            if k1 >= 0 && k2 == 0 {
                ans += 1;
            }
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 2 3 4",
            "4"
        ),
        (
            r"13 1 4 3000",
            "87058"
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