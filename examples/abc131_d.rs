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
    let mut jobs: Vec<(usize, usize)> =
        (0..n)
            .map(|_| {
                (
                    iterator.next().unwrap().parse().unwrap(),
                    iterator.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

    jobs.sort_by(|a, b| a.1.cmp(&b.1));

    let mut ans = "Yes";

    let mut time = 0;
    for (need, limit) in jobs {
        time += need;
        if time > limit {
            ans = "No";
            break;
        }
    }

    return ans.to_string();
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"5
2 4
1 9
1 8
4 9
3 12",
            "Yes"
        ),
        (
            r"3
334 1000
334 1000
334 1000",
            "No"
        ),
        (
            r"30
384 8895
1725 9791
170 1024
4 11105
2 6
578 1815
702 3352
143 5141
1420 6980
24 1602
849 999
76 7586
85 5570
444 4991
719 11090
470 10708
1137 4547
455 9003
110 9901
15 8578
368 3692
104 1286
3 4
366 12143
7 6649
610 2374
152 7324
4 7042
292 11386
334 5720",
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