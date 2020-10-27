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
    let c: isize = iterator.next().unwrap().parse().unwrap();
    let b: Vec<isize> = (0..m).map(|_| iterator.next().unwrap().parse().unwrap()).collect();


    let mut ans: usize = 0;
    for _ in 0..n {
        let mut x = c;
        for i in 0..m {
            let a: isize = iterator.next().unwrap().parse().unwrap();
            x += a * b[i];
        }
        if x > 0 {
            ans += 1;
        }
    };
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 -10
1 2 3
3 2 1
1 2 2",
            "1"
        ),
        (
            r"5 2 -4
-2 5
100 41
100 40
-3 0
-6 -2
18 -13",
            "2"
        ),
        (
            r"3 3 0
100 -100 0
0 100 100
100 100 100
-100 100 100",
            "0"
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