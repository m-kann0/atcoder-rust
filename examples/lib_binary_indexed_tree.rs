fn main() {
    // TODO
}

mod bit {
    pub struct BinaryIndexedTree {
        bit: Vec<usize>,
        n: usize,
    }

    impl BinaryIndexedTree {
        // 要素数nで初期化する
        pub fn new(n: usize) -> Self {
            Self {
                bit: vec![0; n + 1],
                n,
            }
        }

        // 1番目からi番目(1-indexed)までの要素の総和を求める
        pub fn sum(&self, i: usize) -> usize {
            let mut s = 0;
            let mut i = i as isize;
            while i > 0 {
                s += self.bit[i as usize];
                i -= i & -i;
            }
            s
        }

        // i番目(1-indexed)の要素にxを加算する
        pub fn add(&mut self, i: usize, x: usize) {
            assert_ne!(i, 0);
            let mut i = i as isize;
            while i <= self.n as isize {
                self.bit[i as usize] += x;
                i += i & -i;
            }
        }
    }
}
