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

    let mut map: HashMap<String, usize> = HashMap::with_capacity(n);

    for _ in 0..n {
        let name = iterator.next().unwrap().to_string();
        *map.entry(name).or_insert(0) += 1;
    }

    let mut m: usize = 0;
    let mut name: String = String::new();
    for (n, &v) in map.iter() {
        if v > m {
            m = v;
            name = n.clone();
        }
    }

    return name;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
taro
jiro
taro
saburo",
            "taro"
        ),
        (
            r"1
takahashikun",
            "takahashikun"
        ),
        (
            r"9
a
b
c
c
b
c
b
d
e",
            "b"
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