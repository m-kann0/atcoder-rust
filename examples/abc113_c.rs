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
    let m: usize = iterator.next().unwrap().parse().unwrap();

    let mut cities: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for i in 0..m {
        let p: usize = iterator.next().unwrap().parse().unwrap();
        let y: usize = iterator.next().unwrap().parse().unwrap();
        cities[p - 1].push((y, i));
    }

    let mut result: Vec<String> = vec![String::new(); m];

    for i in 0..n {
        cities[i].sort();
        for (j, city) in cities[i].iter().enumerate() {
            result[city.1] = format!("{:0>6}{:0>6}", i + 1, j + 1);
        }
    }

    return result.join("\n").trim().to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3
1 32
2 63
1 12",
            "000001000002
000002000001
000001000001"
        ),
        (
            r"2 3
2 55
2 77
2 99",
            "000002000001
000002000002
000002000003"
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