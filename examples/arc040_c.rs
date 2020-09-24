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
    let mut map: Vec<Vec<char>> = Vec::new();
    for _ in 0..n {
        let line = iterator.next().unwrap().chars().collect();
        map.push(line);
    }

    let mut ans = 0;
    for i in 0..n {
        let mut last = 1_000_000;
        for j in 0..n {
            if map[i][j] == '.' {
                last = j;
            }
        }
        if last == 1_000_000 {
            continue;
        }
        for j in 0..=last {
            map[i][j] = 'o';
        }
        if i != n - 1 {
            for j in last..n {
                map[i + 1][j] = 'o';
            }
        }
        ans += 1;
    }
    ans.to_string()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"7
...oooo
oo.....
ooooooo
ooooooo
.....oo
oooo...
ooooooo",
            "2"
        ),
        (
            r"4
.oo.
..oo
o..o
oo..",
            "3"
        ),
        (
            r"1
o",
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