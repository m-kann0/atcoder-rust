use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let w: usize = iterator.next().unwrap().parse().unwrap();
    let h: usize = iterator.next().unwrap().parse().unwrap();
    let x: usize = iterator.next().unwrap().parse().unwrap();
    let y: usize = iterator.next().unwrap().parse().unwrap();

    // (x, y)が長方形の中心でない場合、(x, y)と中心をとおる直線＝長方形を半分に分割する直線
    // (x, y)が長方形の中心の場合、どのような直線も長方形を半分に分割する
    // -> 「面積の大きくないほうの面積」は、必ず元の長方形の半分の面積となる
    // -> (x, y)が長方形の中心の場合、そのような直線は無限に存在する

    return format!(
        "{:.6} {}",
        w as f64 * h as f64 / 2.0,
        if x * 2 == w && y * 2 == h { "1" } else { "0" }
    );
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"2 3 1 2",
            "3.000000 0"
        ),
        (
            r"2 2 1 1",
            "2.000000 1"
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