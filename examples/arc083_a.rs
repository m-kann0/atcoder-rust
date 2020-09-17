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

    let a: usize = iterator.next().unwrap().parse().unwrap();
    let b: usize = iterator.next().unwrap().parse().unwrap();
    let c: usize = iterator.next().unwrap().parse().unwrap();
    let d: usize = iterator.next().unwrap().parse().unwrap();
    let e: usize = iterator.next().unwrap().parse().unwrap();
    let f: usize = iterator.next().unwrap().parse().unwrap();

    let mut ans_water = 0;
    let mut ans_sugar = 0;
    for i in 0..=30 {
        for j in 0..=30 {
            let water = a * 100 * i + b * 100 * j;
            if water > f {
                continue;
            }
            // eprintln!("water = {:?}", water);

            let sugar_max = min(f - water, e * water / 100);
            // eprintln!("sugar_max = {:?}", sugar_max);
            for x in 0..=3000 {
                let s1 = c * x;
                if s1 > sugar_max {
                    break;
                }
                let s2 = d * ((sugar_max - s1) / d);
                let sugar = s1 + s2;
                // eprintln!("sugar = {:?}", sugar);
                if ans_sugar == 0 {
                    ans_water = water;
                    ans_sugar = sugar;
                } else {
                    if sugar * (ans_sugar + ans_water) > ans_sugar * (sugar + water) {
                        ans_water = water;
                        ans_sugar = sugar;
                    }
                }
            }
        }
    }
    format!("{} {}", ans_water + ans_sugar, ans_sugar)
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1 2 10 20 15 200",
            "110 10"
        ),
        (
            r"1 2 1 2 100 1000",
            "200 100"
        ),
        (
            r"17 19 22 26 55 2802",
            "2634 934"
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