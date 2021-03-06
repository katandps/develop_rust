use std::io::*;
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
    let x: i32 = read();
    let y: i32 = read();
    let k: i32 = read();
    if k <= y {
        println!("{}", x + k);
        return;
    }

    println!("{}", y + x - k + y)
}
