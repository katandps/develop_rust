#[allow(dead_code)]
fn main() {
    let stdin = stdin();
    let mut reader = StdinReader::new(stdin.lock());
    let (n, m) = reader.u2();
    let mut ans = ModInt::new(0);
    let comb = Combination::new(n);
    let fact = Factorial::new(m);
    for i in 0..=n {
        let l = comb.get(i) * fact.npk(m, i) * fact.npk(m - i, n - i) * fact.npk(m - i, n - i);
        if i % 2 == 0 {
            ans += l;
        } else {
            ans -= l;
        }
    }
    println!("{}", ans);
}

#[allow(unused_imports)]
use factorial::Factorial;

#[allow(dead_code)]
mod factorial {
    use super::mod_int::ModInt;

    pub struct Factorial {
        stack: Vec<ModInt<usize>>,
    }

    impl Factorial {
        pub fn new(number: usize) -> Self {
            let mut stack: Vec<ModInt<usize>> = Vec::new();
            stack.push(ModInt::new(1));
            for i in 1..(number + 1) as usize {
                let k = stack[i - 1] * i;
                stack.push(k);
            }

            Factorial { stack: stack }
        }

        pub fn get(&self, number: usize) -> ModInt<usize> {
            self.stack[number]
        }

        /// 順列nPk
        pub fn npk(&self, n: usize, k: usize) -> ModInt<usize> {
            if k > n {
                ModInt::new(0)
            } else {
                self.stack[n] / self.stack[n - k]
            }
        }

        /// 組み合わせnCk
        pub fn nck(&self, n: usize, k: usize) -> ModInt<usize> {
            if k > n {
                ModInt::new(0)
            } else {
                self.npk(n, k) / self.stack[k]
            }
        }

        ///重複組合せnHk
        pub fn nhk(&self, n: usize, k: usize) -> ModInt<usize> {
            self.stack[n + k - 1] / self.stack[k] / self.stack[n - 1]
        }
    }
}

#[allow(unused_imports)]
use combination::Combination;

#[allow(dead_code)]
mod combination {
    use super::mod_int::*;

    pub struct Combination {
        stack: Vec<ModInt<usize>>,
    }

    impl Combination {
        pub fn new(number: usize) -> Self {
            let mut stack = vec![ModInt::new(1)];
            for i in 0..number {
                let t = stack[i] * (number - i);
                stack.push(t / (i + 1));
            }
            Combination { stack: stack }
        }

        pub fn get(&self, number: usize) -> ModInt<usize> {
            self.stack[number]
        }
    }
}

#[allow(unused_imports)]
use mod_int::*;

#[allow(dead_code)]
pub mod mod_int {
    use std::fmt;
    use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

    type Num = usize;

    const MOD: Num = 1_000_000_007;

    #[derive(Copy, Clone)]
    pub struct ModInt<T: Clone + Copy> {
        pub v: T,
    }

    impl Add<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            let mut t = rhs + self.v;
            if t >= MOD {
                t = t - MOD;
            }
            ModInt::new(t)
        }
    }

    impl Add<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn add(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self + rhs.v
        }
    }

    impl AddAssign<Num> for ModInt<Num> {
        fn add_assign(&mut self, other: Num) {
            *self = *self + other
        }
    }

    impl AddAssign<ModInt<Num>> for ModInt<Num> {
        fn add_assign(&mut self, other: ModInt<Num>) {
            *self = *self + other
        }
    }

    impl Sub<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: Num) -> ModInt<Num> {
            let rhs = if rhs >= MOD { rhs % MOD } else { rhs };
            let value = if self.v < rhs { self.v + MOD } else { self.v };
            ModInt::new(value - rhs)
        }
    }

    impl Sub<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn sub(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self - rhs.v
        }
    }

    impl SubAssign<Num> for ModInt<Num> {
        fn sub_assign(&mut self, other: Num) {
            *self = *self - other
        }
    }

    impl SubAssign<ModInt<Num>> for ModInt<Num> {
        fn sub_assign(&mut self, other: ModInt<Num>) {
            *self = *self - other
        }
    }

    impl Mul<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            ModInt::new((self.v * rhs) % MOD)
        }
    }

    impl Mul<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn mul(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self * rhs.v
        }
    }

    impl MulAssign<Num> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: Num) {
            *self = *self * rhs
        }
    }

    impl MulAssign<ModInt<Num>> for ModInt<Num> {
        fn mul_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self * rhs
        }
    }

    impl Div<Num> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, mut rhs: Num) -> ModInt<Num> {
            if rhs >= MOD {
                rhs %= MOD;
            }
            self * ModInt::new(rhs).pow(MOD - 2)
        }
    }

    impl Div<ModInt<Num>> for ModInt<Num> {
        type Output = ModInt<Num>;
        fn div(self, rhs: ModInt<Num>) -> ModInt<Num> {
            self / rhs.v
        }
    }

    impl DivAssign<Num> for ModInt<Num> {
        fn div_assign(&mut self, rhs: Num) {
            *self = *self / rhs
        }
    }

    impl DivAssign<ModInt<Num>> for ModInt<Num> {
        fn div_assign(&mut self, rhs: ModInt<Num>) {
            *self = *self / rhs
        }
    }

    impl fmt::Display for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.v)
        }
    }

    impl fmt::Debug for ModInt<Num> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.v)
        }
    }

    impl ModInt<Num> {
        pub fn pow(self, e: Num) -> ModInt<Num> {
            let mut result = ModInt::new(1);
            let mut cur = self;
            let mut e = e;
            while e > 0 {
                if e & 1 == 1 {
                    result *= cur;
                }
                e >>= 1;
                cur *= cur;
            }
            result
        }

        pub fn new(v: Num) -> ModInt<Num> {
            if v >= MOD {
                ModInt { v: v % MOD }
            } else {
                //                ModInt { v } // unstable in rust 1.15.1
                ModInt { v: v }
            }
        }

        pub fn get(&self) -> Num {
            self.v
        }
    }
}

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use std::{cmp::*, collections::*, io::*, num::*, str::*};
#[allow(unused_imports)]
use stdin_reader::StdinReader;

#[allow(dead_code)]
mod stdin_reader {
    use std::fmt::Debug;
    use std::io::*;
    use std::str::*;

    pub struct StdinReader<R: BufRead> {
        reader: R,
        buf: Vec<u8>,
        // Should never be empty
        pos: usize, // Should never be out of bounds as long as the input ends with '\n'
    }

    impl<R: BufRead> StdinReader<R> {
        pub fn new(r: R) -> StdinReader<R> {
            StdinReader {
                reader: r,
                buf: Vec::new(),
                pos: 0,
            }
        }
        pub fn next<T: FromStr>(&mut self) -> T
        where
            T::Err: Debug,
        {
            if self.buf.is_empty() {
                self._read_next_line();
            }
            let mut start = None;
            loop {
                if self.pos == self.buf.len() {
                    break;
                }
                match (self.buf[self.pos], start.is_some()) {
                    (b' ', true) | (b'\n', true) => break,
                    (_, true) | (b' ', false) => self.pos += 1,
                    (b'\n', false) => self._read_next_line(),
                    (_, false) => start = Some(self.pos),
                }
            }
            let target = &self.buf[start.unwrap()..self.pos];
            from_utf8(target).unwrap().parse().unwrap()
        }

        fn _read_next_line(&mut self) {
            self.pos = 0;
            self.buf.clear();
            if self.reader.read_until(b'\n', &mut self.buf).unwrap() == 0 {
                panic!("Reached EOF");
            }
        }

        pub fn str(&mut self) -> String {
            self.next()
        }
        pub fn s(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }
        pub fn i(&mut self) -> i64 {
            self.next()
        }
        pub fn i2(&mut self) -> (i64, i64) {
            (self.next(), self.next())
        }
        pub fn i3(&mut self) -> (i64, i64, i64) {
            (self.next(), self.next(), self.next())
        }
        pub fn u(&mut self) -> usize {
            self.next()
        }
        pub fn u2(&mut self) -> (usize, usize) {
            (self.next(), self.next())
        }
        pub fn u3(&mut self) -> (usize, usize, usize) {
            (self.next(), self.next(), self.next())
        }
        pub fn u4(&mut self) -> (usize, usize, usize, usize) {
            (self.next(), self.next(), self.next(), self.next())
        }
        pub fn u5(&mut self) -> (usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn u6(&mut self) -> (usize, usize, usize, usize, usize, usize) {
            (
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
                self.next(),
            )
        }
        pub fn f(&mut self) -> f64 {
            self.next()
        }
        pub fn f2(&mut self) -> (f64, f64) {
            (self.next(), self.next())
        }
        pub fn c(&mut self) -> char {
            self.next::<String>().pop().unwrap()
        }
        pub fn iv(&mut self, n: usize) -> Vec<i64> {
            (0..n).map(|_| self.i()).collect()
        }
        pub fn iv2(&mut self, n: usize) -> Vec<(i64, i64)> {
            (0..n).map(|_| self.i2()).collect()
        }
        pub fn iv3(&mut self, n: usize) -> Vec<(i64, i64, i64)> {
            (0..n).map(|_| self.i3()).collect()
        }
        pub fn uv(&mut self, n: usize) -> Vec<usize> {
            (0..n).map(|_| self.u()).collect()
        }
        pub fn uv2(&mut self, n: usize) -> Vec<(usize, usize)> {
            (0..n).map(|_| self.u2()).collect()
        }
        pub fn uv3(&mut self, n: usize) -> Vec<(usize, usize, usize)> {
            (0..n).map(|_| self.u3()).collect()
        }
        pub fn uv4(&mut self, n: usize) -> Vec<(usize, usize, usize, usize)> {
            (0..n).map(|_| self.u4()).collect()
        }
        pub fn fv(&mut self, n: usize) -> Vec<f64> {
            (0..n).map(|_| self.f()).collect()
        }
        pub fn cmap(&mut self, h: usize) -> Vec<Vec<char>> {
            (0..h).map(|_| self.s()).collect()
        }
    }
}
