use std::cmp::min;

fn main() {
    let a = vec![5, 3, 7, 9, 6, 4, 1, 2];
    let mut st = segtree::SegmentTree::new(
        a.len(),
        std::usize::MAX,
        |x, y| min(x, y)
    );
    for i in 0..a.len() {
        st.update(i, a[i]);
    }
    println!("{}", st.query(2, 5)); // [2, 5) -> min(7, 9, 6) -> 6
}

#[allow(dead_code)]
mod segtree {
    pub struct SegmentTree<T, F> {
        n: usize,
        dat: Vec<T>,
        initial_value: T,
        op: F,
    }

    impl<T: Copy, F> SegmentTree<T, F>
        where
            F: Fn(T, T) -> T,
    {
        pub fn new(n: usize, initial_value: T, op: F) -> Self {
            let mut m: usize = 1;
            while m < n {
                m *= 2;
            }
            Self {
                n: m,
                dat: vec![initial_value; 2 * m - 1],
                initial_value,
                op,
            }
        }

        pub fn get(&self, i: usize) -> T {
            self.dat[i + self.n - 1]
        }

        pub fn update(&mut self, i: usize, value: T) {
            let mut i = i + self.n - 1;
            self.dat[i] = value;
            while i > 0 {
                i = (i - 1) / 2;
                self.dat[i] = (self.op)(self.dat[i * 2 + 1], self.dat[i * 2 + 2]);
            }
        }

        pub fn query(&self, a: usize, b: usize) -> T {
            self._query(a, b, 0, 0, self.n)
        }

        fn _query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
            if r <= a || b <= l {
                self.initial_value
            } else if a <= l && r <= b {
                self.dat[k]
            } else {
                let vl = self._query(a, b, k * 2 + 1, l, (l + r) / 2);
                let vr = self._query(a, b, k * 2 + 2, (l + r) / 2, r);
                (self.op)(vl, vr)
            }
        }
    }
}
