use std::io::Read;

fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let circle = Circle {
        center: Point {
            x: iterator.next().unwrap().parse().unwrap(),
            y: iterator.next().unwrap().parse().unwrap()
        },
        r: iterator.next().unwrap().parse().unwrap(),
    };

    let rect = Rectangle {
        left_bottom: Point {
            x: iterator.next().unwrap().parse().unwrap(),
            y: iterator.next().unwrap().parse().unwrap(),
        },
        right_top: Point {
            x: iterator.next().unwrap().parse().unwrap(),
            y: iterator.next().unwrap().parse().unwrap(),
        },
    };

    let mut ans = String::new();

    if rect.wrap(&circle) {
        ans += "NO\n";
    } else {
        ans += "YES\n";
    }

    if circle.wrap(&rect) {
        ans += "NO";
    } else {
        ans += "YES";
    }

    return ans;
}

struct Point {
    x: isize,
    y: isize,
}

struct Circle {
    center: Point,
    r: isize,
}

struct Rectangle {
    left_bottom: Point,
    right_top: Point,
}

impl Circle {
    fn wrap(&self, rect: &Rectangle) -> bool {
        self.wrap_point(&rect.left_bottom)
            && self.wrap_point(&rect.right_top)
            && self.wrap_point(&Point { x: rect.left_bottom.x, y: rect.right_top.y })
            && self.wrap_point(&Point { x: rect.right_top.x, y: rect.left_bottom.y })
    }

    fn wrap_point(&self, point: &Point) -> bool {
        let dist = dist(
            point.x as f64,
            point.y as f64,
            self.center.x as f64,
            self.center.y as f64,
        );
        self.r as f64 >= dist
    }
}

fn dist(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}

impl Rectangle {
    fn wrap(&self, circle: &Circle) -> bool {
        self.left_bottom.x <= (circle.center.x - circle.r)
            && self.left_bottom.y <= (circle.center.y - circle.r)
            && self.right_top.x >= (circle.center.x + circle.r)
            && self.right_top.y >= (circle.center.y + circle.r)
    }
}


#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"-1 -1 2
2 3 4 5",
            "YES
YES"
        ),
        (
            r"0 1 1
-2 0 4 3",
            "NO
YES"
        ),
        (
            r"0 0 5
-2 -2 2 1",
            "YES
NO"
        ),
        (
            r"0 0 2
0 0 4 4",
            "YES
YES"
        ),
        (
            r"0 0 5
-4 -4 4 4",
            "YES
YES"
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