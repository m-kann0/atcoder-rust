use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let deg: usize = iterator.next().unwrap().parse().unwrap();
    let dis: usize = iterator.next().unwrap().parse().unwrap();

    let deg = deg * 10;
    let h =
        if deg >= 1125 && deg < 3375 { "NNE" }
        else if deg >= 3375 && deg < 5625 { "NE" }
        else if deg >= 5625 && deg < 7875 { "ENE" }
        else if deg >= 7875 && deg < 10125 { "E" }
        else if deg >= 10125 && deg < 12375 { "ESE" }
        else if deg >= 12375 && deg < 14625 { "SE" }
        else if deg >= 14625 && deg < 16875 { "SSE" }
        else if deg >= 16875 && deg < 19125 { "S" }
        else if deg >= 19125 && deg < 21375 { "SSW" }
        else if deg >= 21375 && deg < 23625 { "SW" }
        else if deg >= 23625 && deg < 25875 { "WSW" }
        else if deg >= 25875 && deg < 28125 { "W" }
        else if deg >= 28125 && deg < 30375 { "WNW" }
        else if deg >= 30375 && deg < 32625 { "NW" }
        else if deg >= 32625 && deg < 34875 { "NNW" }
        else { "N" };

    let dis: usize = format!("{:.0}", (dis as f64 / 6.0).round()).parse().unwrap();
    let f: usize =
        if dis >= 0 && dis <= 2 {
            0
        } else if dis >= 3 && dis <= 15 {
            1
        } else if dis >= 16 && dis <= 33 {
            2
        } else if dis >= 34 && dis <= 54 {
            3
        } else if dis >= 55 && dis <= 79 {
            4
        } else if dis >= 80 && dis <= 107 {
            5
        } else if dis >= 108 && dis <= 138 {
            6
        } else if dis >= 139 && dis <= 171 {
            7
        } else if dis >= 172 && dis <= 207 {
            8
        } else if dis >= 208 && dis <= 244 {
            9
        } else if dis >= 245 && dis <= 284 {
            10
        } else if dis >= 285 && dis <= 326 {
            11
        } else {
            12
        };

    return if f == 0 {
        "C 0".to_string()
    } else {
        format!("{} {}", h, f)
    };
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (r"2750 628", r"W 5"),
        (r"161 8", r"C 0"),
        (r"3263 15", r"NNW 1"),
        (r"1462 1959", r"SE 12"),
        (r"1687 1029", r"SSE 8"),
        (r"2587 644", r"WSW 5"),
        (r"113 201", r"NNE 3"),
        (r"2048 16", r"SSW 1"),
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