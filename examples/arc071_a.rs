use std::io::Read;
use std::cmp::min;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut table = make_table(iterator.next().unwrap().to_string());
    for _i in 1..n {
        let table2 = make_table(iterator.next().unwrap().to_string());
        subtract(&mut table, &table2);
    }

    make_string(&table)
}

fn make_table(s: String) -> [u8; 26] {
    let mut table = [0 as u8; 26];
    for c in s.chars() {
        let index = c as u8 - 'a' as u8;
        table[index as usize] += 1;
    }
    table
}

fn subtract(table1: &mut [u8; 26], table2: &[u8; 26]) {
    for i in 0..26 {
        table1[i] = min(table1[i], table2[i]);
    }
}

fn make_string(table: &[u8; 26]) -> String {
    let mut result = String::new();
    for i in 0..26 {
        let mut count = table[i];
        while count > 0 {
            let c = ('a' as u8 + i as u8) as char;
            result.push(c);
            count -= 1;
        }
    }
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
cbaa
daacc
acacac",
            "aac"
        ),
        (
            r"3
a
aa
b",
            ""
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