use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: isize = iterator.next().unwrap().parse().unwrap();

    let mut dictionary: Vec<String> = Vec::new();
    for _ in 0..n {
        let word = reverse(iterator.next().unwrap().to_string());
        dictionary.push(word);
    }

    dictionary.sort();

    let mut result = String::new();
    for word in dictionary {
        result.push_str(&format!("{}\n", reverse(word)));
    }

    return result.trim().to_string();
}

fn reverse(s: String) -> String {
    s.chars()
        .rev()
        .map(|it| it.to_string())
        .collect::<Vec<String>>()
        .join("")
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
chokudai
kensho
imos
yuichirw
ao",
            r"chokudai
ao
kensho
imos
yuichirw"
        ),
        (
            r"2
dart
art",
            r"art
dart"
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