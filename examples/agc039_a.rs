use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let s = iterator.next().unwrap();
    let k: u64 = iterator.next().unwrap().parse().unwrap();

    let count1: u64 = count(s.to_string());
    let count2: u64 = count(format!("{}{}", s, s));
    let count3: u64 = count(format!("{}{}{}", s, s, s));

    dbg!(count1);
    dbg!(count2);
    dbg!(count3);

    let ans: u64 =
        if (count1 * 2) == count2 {
            count1 * k
        } else if (count1 + 2 * (count2 - count1)) == count3 {
            count1 * k + (k - 1)
        } else {
            count1 * k + k / 2
        };

    return ans.to_string();
}

fn count(s: String) -> u64 {
    let mut result = 0;

    let mut prev = ' ';
    let mut con = 0;
    for curr in s.chars() {
        if curr != prev {
            result += con / 2;
            prev = curr;
            con = 1;
        } else {
            con += 1;
        }
    }
    result += con / 2;

    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"issii
2",
            "4"
        ),
        (
            r"qq
81",
            "81"
        ),
        (
            r"cooooooooonteeeeeeeeeest
999993333",
            "8999939997"
        ),
        (
            r"a
1",
            "0"
        ),
        (
            r"a
2",
            "1"
        ),
        (
            r"ab
1",
            "0"
        ),
        (
            r"ab
2",
            "0"
        ),
        (
            r"issiii
3",
            "8"
        ),
        (
            r"isi
3",
            "2"
        ),
        (
            r"iii
2",
            "3"
        ),
        (
            r"iii
3",
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