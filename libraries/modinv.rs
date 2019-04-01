use std::mem::swap;

struct ModInv {}

impl ModInv {
    pub fn mod_inv(number: isize, modulo: isize) -> isize {
        let mut n = number;
        let mut b = modulo;
        let mut u: isize = 1;
        let mut v: isize = 0;

        while b > 0 {
            let t: isize = n / b;
            n -= t * b;
            swap(&mut n, &mut b);
            u -= t * v;
            swap(&mut u, &mut v);
        }
        u %= modulo;
        if u < 0 {
            u += modulo;
        }
        u
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calc() {
        assert_eq!(1, ModInv::mod_inv(1, 13));
        assert_eq!(7, ModInv::mod_inv(2, 13));
        assert_eq!(9, ModInv::mod_inv(3, 13));
        assert_eq!(10, ModInv::mod_inv(4, 13));
        assert_eq!(8, ModInv::mod_inv(5, 13));
        assert_eq!(11, ModInv::mod_inv(6, 13));
        assert_eq!(2, ModInv::mod_inv(7, 13));
        assert_eq!(5, ModInv::mod_inv(8, 13));
        assert_eq!(3, ModInv::mod_inv(9, 13));
        assert_eq!(4, ModInv::mod_inv(10, 13));
        assert_eq!(6, ModInv::mod_inv(11, 13));
        assert_eq!(12, ModInv::mod_inv(12, 13));
        assert_eq!(0, ModInv::mod_inv(13, 13));
        assert_eq!(1, ModInv::mod_inv(14, 13));
    }
}