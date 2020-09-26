use std::io::Read;


//---------- begin SegmentTree Point update Range query ----------
pub trait PURQ {
    type T: Clone;
    fn fold(l: &Self::T, r: &Self::T) -> Self::T;
    fn e() -> Self::T;
}

struct SegmentTreePURQ<R: PURQ> {
    seg: Vec<R::T>,
    size: usize,
}

#[allow(dead_code)]
impl<R: PURQ> SegmentTreePURQ<R> {
    fn new(n: usize) -> SegmentTreePURQ<R> {
        let size = n.next_power_of_two();
        SegmentTreePURQ {
            seg: vec![R::e(); 2 * size],
            size: size,
        }
    }
    fn build_by(a: &[R::T]) -> SegmentTreePURQ<R> {
        let size = a.len().next_power_of_two();
        let mut b = vec![R::e(); 2 * size];
        for i in 0..a.len() {
            b[i + size] = a[i].clone();
        }
        let mut seg = SegmentTreePURQ { seg: b, size: size };
        seg.update_all();
        seg
    }
    fn update(&mut self, x: usize, v: R::T) {
        assert!(x < self.size);
        let mut x = x + self.size;
        let a = &mut self.seg;
        a[x] = v;
        x >>= 1;
        while x > 0 {
            a[x] = R::fold(&a[2 * x], &a[2 * x + 1]);
            x >>= 1;
        }
    }
    fn update_tmp(&mut self, x: usize, v: R::T) {
        self.seg[self.size + x] = v;
    }
    fn update_all(&mut self) {
        let a = &mut self.seg;
        for i in (1..self.size).rev() {
            a[i] = R::fold(&a[2 * i], &a[2 * i + 1]);
        }
    }
    fn find(&self, l: usize, r: usize) -> R::T {
        assert!(l <= r && r <= self.size);
        let mut x = R::e();
        let mut y = R::e();
        let mut l = l + self.size;
        let mut r = r + self.size;
        let a = &self.seg;
        while l < r {
            if l & 1 == 1 {
                x = R::fold(&x, &a[l]);
                l += 1;
            }
            if r & 1 == 1 {
                r -= 1;
                y = R::fold(&a[r], &y);
            }
            l >>= 1;
            r >>= 1;
        }
        R::fold(&x, &y)
    }
    // f(a[l..k]) がkについてfalse, false, ..., true みたいに単調であるとした時
    // 戻り値Some(x)は
    // f(&a[l..x]) = false
    // f(&a[l..=x]) = true
    // を満たす
    fn search_right<F>(&self, l: usize, f: F) -> Option<usize>
        where
            F: Fn(&R::T) -> bool,
    {
        let a = &self.seg;
        let mut v = R::e();
        let mut r = self.size * 2;
        let mut l = l + self.size;
        while l < r {
            if l & 1 == 1 {
                let u = R::fold(&v, &a[l]);
                if f(&u) {
                    break;
                }
                l += 1;
                v = u;
            }
            l >>= 1;
            r >>= 1;
        }
        if l == r {
            return None;
        }
        let mut x = l;
        while x < self.size {
            x <<= 1;
            let u = R::fold(&v, &a[x]);
            if !f(&u) {
                x += 1;
                v = u;
            }
        }
        Some(x - self.size)
    }
}
//---------- end SegmentTree Point update Range query ----------

struct RMQ;
impl PURQ for RMQ {
    type T = i32;
    fn fold(l: &Self::T, r: &Self::T) -> Self::T {
        std::cmp::max(*l, *r)
    }
    fn e() -> Self::T {
        -1
    }
}


fn main() {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();

    let answer = solve(&buf);

    println!("{}", answer);
}

fn solve(input: &str) -> String {
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let k: usize = iterator.next().unwrap().parse().unwrap();
    let mut seg = SegmentTreePURQ::<RMQ>::build_by(&vec![0; 300_100]);
    for _ in 0..n {
        let ai: usize = iterator.next().unwrap().parse().unwrap();
        let l = if ai < k { 0 } else { ai - k };
        let r = if ai + k + 1 > 300_100 { 300_010 } else { ai + k + 1 };
        seg.update(ai, seg.find(l, r) + 1);
    }
    seg.find(0, 300_000).to_string()
}

#[test]
fn test2() {
    let mut seg = SegmentTreePURQ::<RMQ>::build_by(&vec![0; 10]);
    for i in 0..10 {
        println!("{}", seg.find(i, i + 1));
    }
}

#[test]
fn test() {
    let cases: Vec<(&str, &str)> = vec![
        (
            r"10 3
1
5
4
3
8
6
9
7
2
4",
            "7"
        ),
        (
            r"10 3
4
3
2
1
5
6
7
8
9
10",
            "9"
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