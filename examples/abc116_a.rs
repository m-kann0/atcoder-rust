use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let ab: usize = iterator.next().unwrap().parse().unwrap();
    let bc: usize = iterator.next().unwrap().parse().unwrap();
    let _ca: usize = iterator.next().unwrap().parse().unwrap();

    let ans = ab * bc / 2;
    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3 4 5",
            "6"
        ),
        (
            r"5 12 13",
            "30"
        ),
        (
            r"45 28 53",
            "630"
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