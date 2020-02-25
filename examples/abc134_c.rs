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

    let mut vec: Vec<i64> = Vec::with_capacity(n);

    let mut first: i64 = 0;
    let mut second: i64 = 0;
    for _i in 0..n {
        let a: i64 = iterator.next().unwrap().parse().unwrap();
        if a > first {
            second = first;
            first = a;
        } else if a > second {
            second = a;
        }
        vec.push(a);
    }

    let mut answer = String::new();

    for i in 0..n {
        let a = vec[i];
        if a == first {
            answer += &format!("{}\n", second);
        } else {
            answer += &format!("{}\n", first);
        }
    }

    return answer;
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"3
1
4
3",
            "4
3
4
"
        ),
        (
            r"2
5
5",
            "5
5
"
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