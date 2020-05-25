use std::io::Read;
use std::collections::HashMap;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let mut table: HashMap<char, char> = HashMap::new();
    table.insert('b', '1');
    table.insert('c', '1');
    table.insert('t', '3');
    table.insert('j', '3');
    table.insert('l', '5');
    table.insert('v', '5');
    table.insert('p', '7');
    table.insert('m', '7');
    table.insert('n', '9');
    table.insert('g', '9');
    table.insert('d', '2');
    table.insert('w', '2');
    table.insert('f', '4');
    table.insert('q', '4');
    table.insert('s', '6');
    table.insert('x', '6');
    table.insert('h', '8');
    table.insert('k', '8');
    table.insert('z', '0');
    table.insert('r', '0');

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut result = String::new();
    for _ in 0..n {
        let s: String = iterator.next().unwrap().to_string();
        let converted = convert(s, &table);
        if !converted.is_empty() {
            result.push_str(&format!("{} ", converted));
        }
    }

    return result.trim().to_string();
}

fn convert(s: String, table: &HashMap<char, char>) -> String {
    let mut result = String::new();
    for c in s.to_lowercase().chars() {
        if let Some(a) = table.get(&c) {
            result.push(*a);
        }
    }
    return result;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
Mozart plays magic.",
            r"7003 756 791"
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