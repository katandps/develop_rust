use std::cmp::*;
use std::io::*;
use std::num::*;
use std::str::*;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let s = stdin
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect::<String>();
    s.parse::<T>().ok().unwrap()
}

fn main() {
    let a: i32 = read();
    let b: i32 = read();
    let n: i32 = read();

    for i in n.. {
        if i % a == 0 && i % b == 0 {
            println!("{}", i);
            return;
        }
    }
}
