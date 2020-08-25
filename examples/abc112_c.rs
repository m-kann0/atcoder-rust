use std::io::Read;
use std::cmp::max;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut points: Vec<(isize, isize, isize)> = Vec::new();
    for _ in 0..n {
        let xi: isize = iterator.next().unwrap().parse().unwrap();
        let yi: isize = iterator.next().unwrap().parse().unwrap();
        let hi: isize = iterator.next().unwrap().parse().unwrap();
        points.push((xi, yi, hi));
    }
    points.sort_by_key(|p| -p.2);

    for cx in 0..=100 {
        for cy in 0..=100 {
            let p0 = points[0];
            let h = p0.2 + (cx - p0.0).abs() + (cy - p0.1).abs();
            let mut is_center = true;
            for i in 1..n {
                let p = points[i];
                let h_calc = max(h - (cx - p.0).abs() - (cy - p.1).abs(), 0);
                if h_calc != p.2 {
                    is_center = false;
                    break;
                }
            }
            if is_center {
                return format!("{} {} {}", cx, cy, h);
            }
        }
    }

    String::new()
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"4
2 3 5
2 1 5
1 2 5
3 2 5",
            "2 2 6"
        ),
        (
            r"2
0 0 100
1 1 98",
            "0 0 100"
        ),
        (
            r"3
99 1 191
100 1 192
99 0 192",
            "100 0 193"
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

mod test {
    use rand::Rng;
    use std::cmp::max;
    use crate::solve;

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let h: u32 = (rng.gen::<f32>() * 1000000000.0) as u32;
            let cx: u32 = (rng.gen::<f32>() * 100.0) as u32;
            let cy: u32 = (rng.gen::<f32>() * 100.0) as u32;
            let n = 10;
            let mut input = String::new();
            input.push_str(&format!("{}\n", n));
            for _ in 0..n {
                let xi: u32 = (rng.gen::<f32>() * 100.0) as u32;
                let yi: u32 = (rng.gen::<f32>() * 100.0) as u32;
                let hi = max(h as isize - (cx as isize - xi as isize).abs() - (cy as isize - yi as isize).abs(), 0);
                input.push_str(&format!("{} {} {}\n", xi, yi, hi));
            }
            if solve(&input) != format!("{} {} {}", cx, cy, h) {
                println!("expected: {} {} {}", cx, cy, h);
                println!("actual: {}", solve(&input));
                println!("for: ");
                println!("{}", input);
            }
        }
    }
}
