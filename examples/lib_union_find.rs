fn main() {
    let mut uf = union_find::UnionFind::new(5);
    uf.unite(0, 1);
    uf.unite(2, 3);
    uf.unite(0, 4);
    eprintln!("uf.len() = {:?}", uf.len());
    eprintln!("uf.size(0) = {:?}", uf.size(0));
    eprintln!("uf.size(1) = {:?}", uf.size(1));
    eprintln!("uf.size(2) = {:?}", uf.size(2));
    eprintln!("uf.size(3) = {:?}", uf.size(3));
    eprintln!("uf.size(4) = {:?}", uf.size(4));
    for i in 0..5 {
        for j in 0..5 {
            eprintln!("uf.same({}, {}) = {:?}", i, j, uf.same(i, j));
        }
    }
}

#[allow(dead_code)]
mod union_find {
    pub struct UnionFind {
        parent: Vec<usize>,
        sizes: Vec<usize>,
        len: usize,
    }

    impl UnionFind {
        pub fn new(n: usize) -> UnionFind {
            UnionFind {
                parent: (0..n).map(|i| i).collect(),
                sizes: vec![1; n],
                len: n,
            }
        }

        pub fn find(&mut self, x: usize) -> usize {
            if self.parent[x] == x {
                x
            } else {
                let px = self.parent[x];
                self.parent[x] = self.find(px);
                self.parent[x]
            }
        }

        pub fn unite(&mut self, x: usize, y: usize) {
            let px = self.find(x);
            let py = self.find(y);

            if px == py {
                return;
            }

            let (large, small) = if self.sizes[px] < self.sizes[py] {
                (py, px)
            } else {
                (px, py)
            };

            self.parent[small] = large;
            self.sizes[large] += self.sizes[small];
            self.sizes[small] = 0;
            self.len -= 1;
        }

        pub fn same(&mut self, x: usize, y: usize) -> bool {
            self.find(x) == self.find(y)
        }

        pub fn size(&mut self, x: usize) -> usize {
            let px = self.find(x);
            self.sizes[px]
        }

        pub fn len(&self) -> usize {
            self.len
        }
    }
}
