#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    solve(Reader::new(stdin.lock()));
}

pub fn solve<R: BufRead>(mut reader: Reader<R>) {
    let n: usize = reader.n();
    let a: Vec<usize> = reader.v(n);
    let q: usize = reader.n();
    let bc: Vec<(usize, usize)> = reader.v2(q);

    let mut map = HashMap::new();
    let mut sum: usize = a.iter().sum();
    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }
    for (b, c) in bc {
        let &count = map.get(&b).unwrap_or(&0);
        sum -= count * b;
        sum += count * c;
        map.remove(&b);
        *map.entry(c).or_insert(0) += count;

        println!("{}", sum);
    }
}

#[allow(unused_imports)]
use {
    itertools::Itertools,
    std::{cmp::*, collections::*, io::*, num::*, str::*},
};

pub use reader::*;

#[allow(dead_code)]
pub mod reader {
    #[allow(unused_imports)]
    use itertools::Itertools;
    use std::{fmt::Debug, io::*, str::*};

    pub struct Reader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        pos: usize,
    }

    impl<R: BufRead> Reader<R> {
        pub fn new(reader: R) -> Reader<R> {
            let (buf, pos) = (Vec::new(), 0);
            Reader { reader, buf, pos }
        }

        pub fn n<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            self.n_op().unwrap()
        }

        pub fn v<T: FromStr>(&mut self, n: usize) -> Vec<T>
        where
            T::Err: Debug,
        {
            (0..n).map(|_| self.n()).collect()
        }
        pub fn v2<T: FromStr, U: FromStr>(&mut self, n: usize) -> Vec<(T, U)>
        where
            T::Err: Debug,
            U::Err: Debug,
        {
            (0..n).map(|_| (self.n(), self.n())).collect()
        }
        pub fn v3<T: FromStr, U: FromStr, V: FromStr>(&mut self, n: usize) -> Vec<(T, U, V)>
        where
            T::Err: Debug,
            U::Err: Debug,
            V::Err: Debug,
        {
            (0..n).map(|_| (self.n(), self.n(), self.n())).collect()
        }
        pub fn v4<T: FromStr, U: FromStr, V: FromStr, W: FromStr>(
            &mut self,
            n: usize,
        ) -> Vec<(T, U, V, W)>
        where
            T::Err: Debug,
            U::Err: Debug,
            V::Err: Debug,
            W::Err: Debug,
        {
            (0..n)
                .map(|_| (self.n(), self.n(), self.n(), self.n()))
                .collect()
        }

        pub fn v5<T: FromStr, U: FromStr, V: FromStr, W: FromStr, X: FromStr>(
            &mut self,
            n: usize,
        ) -> Vec<(T, U, V, W, X)>
        where
            T::Err: Debug,
            U::Err: Debug,
            V::Err: Debug,
            W::Err: Debug,
            X::Err: Debug,
        {
            (0..n)
                .map(|_| (self.n(), self.n(), self.n(), self.n(), self.n()))
                .collect()
        }

        pub fn n_op<T: FromStr>(&mut self) -> Option<T>
        where
            T::Err: Debug,
        {
            if self.buf.is_empty() {
                self._read_next_line();
            }
            let mut start = None;
            while self.pos != self.buf.len() {
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) => self.pos += 1,
                    (b'\n', false) => self._read_next_line(),
                    (_, false) => start = Some(self.pos),
                }
            }
            start.map(|s| from_utf8(&self.buf[s..self.pos]).unwrap().parse().unwrap())
        }

        fn _read_next_line(&mut self) {
            self.pos = 0;
            self.buf.clear();
            self.reader.read_until(b'\n', &mut self.buf).unwrap();
        }
        pub fn s(&mut self) -> Vec<char> {
            self.n::<String>().chars().collect()
        }
        pub fn char_map(&mut self, h: usize) -> Vec<Vec<char>> {
            (0..h).map(|_| self.s()).collect()
        }
        /// charの2次元配列からboolのmapを作る ngで指定した壁のみfalseとなる
        pub fn bool_map(&mut self, h: usize, ng: char) -> Vec<Vec<bool>> {
            self.char_map(h)
                .iter()
                .map(|v| v.iter().map(|&c| c != ng).collect())
                .collect()
        }
        /// h*w行列を取得する
        pub fn matrix<T: FromStr>(&mut self, h: usize, w: usize) -> Vec<Vec<T>>
        where
            T::Err: Debug,
        {
            (0..h).map(|_| self.v(w)).collect()
        }
    }
}
