use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let days: Vec<usize> = vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut holidays: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        let md = iterator.next().unwrap();
        let md: Vec<&str> = md.split('/').collect();
        let m: usize = md[0].parse().unwrap();
        let d: usize = md[1].parse().unwrap();
        holidays.push((m, d));
    }
    holidays.sort();

    let mut is_holiday: Vec<bool> = vec![false; 366];
    let mut day = 1;
    while day <= 366 {
        if day % 7 == 1 || day % 7 == 0 {
            is_holiday[day - 1] = true;
        }
        day += 1;
    }

    for holiday in holidays {
        let mut day = days_from_month_and_day(holiday.0, holiday.1, &days);
        while day <= 366 {
            if !is_holiday[day - 1] {
                is_holiday[day - 1] = true;
                break;
            }
            day += 1;
        }
    }

    let mut c: Vec<usize> = vec![0; 366];
    c[0] = 1;
    for i in 1..366 {
        if is_holiday[i] {
            c[i] = c[i - 1] + 1;
        }
    }

    let ans: usize = *c.iter().max().unwrap();
    ans.to_string()
}

fn days_from_month_and_day(month: usize, day: usize, days: &Vec<usize>) -> usize {
    let mut result = 0;
    for i in 0..(month - 1) {
        result += days[i];
    }
    result += day;
    result
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"1
1/9",
            r"3"
        ),
        (
            r"1
1/10",
            r"2"
        ),
        (
            r"1
1/7",
            r"3"
        ),
        (
            r"2
1/7
1/9",
            r"4"
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