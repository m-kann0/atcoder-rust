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

    let n: usize = iterator.next().unwrap().parse().unwrap();

    let mut map: HashMap<String, i64> = HashMap::new();
    for _i in 0..n {
        let s = iterator.next().unwrap();
        let sorted = sort(s);
        *map.entry(sorted).or_insert(0) += 1;
    }

    let ans: i64 = map.values()
        .map(|&x| {
            if x <= 1 {
                return 0;
            }
            x * (x - 1) / 2
        })
        .sum::<i64>();

    return format!("{}", ans);
}

fn sort(s: &str) -> String {
    let mut v = Vec::with_capacity(s.len());
    for c in s.chars() {
        v.push(c.to_string());
    }
    v.sort();
    v.join("")
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
acornistnt
peanutbomb
constraint",
            "1"
        ),
        (
            r"2
oneplustwo
ninemodsix",
            "0"
        ),
        (
            r"5
abaaaaaaaa
oneplustwo
aaaaaaaaba
twoplusone
aaaabaaaaa",
            "4"
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
