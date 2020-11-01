use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s: Vec<char> = iterator.next().unwrap().chars().collect();

    if s.len() == 1 {
        return if s[0] == '8' {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    }

    if s.len() == 2 {
        return if (s[0].to_digit(10).unwrap() * 10 + s[1].to_digit(10).unwrap()) % 8 == 0 ||
            (s[1].to_digit(10).unwrap() * 10 + s[0].to_digit(10).unwrap()) % 8 == 0 {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    }

    let mut counts: Vec<usize> = vec![0; 10];
    for i in 0..s.len() {
        counts[s[i].to_digit(10).unwrap() as usize] += 1;
    }
    // eprintln!("counts = {:?}", counts);

    for i in 111..=999 {
        let mut can_make = true;
        let mut counts = counts.clone();
        let mut k = i;
        for _ in 0..3 {
            let a = k % 10;
            if counts[a] > 0 {
                counts[a] -= 1;
            } else {
                can_make = false;
                break;
            }
            k /= 10;
        }
        if !can_make {
            continue;
        }
        if i % 2 == 0 && ((i / 2) % 100) % 4 == 0 {
            return "Yes".to_string();
        }
    }
    "No".to_string()
}

#[test]
fn test2() {
    for i in 1..=100 {
        println!("{}", 8 * i);
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1234",
            "Yes"
        ),
        (
            r"1333",
            "No"
        ),
        (
            r"8",
            "Yes"
        ),
        (
            r"46",
            "Yes"
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