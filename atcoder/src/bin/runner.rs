mod a;
mod b;
mod c;
mod d;
mod e;
mod f;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("sample");
    a(path);
}

#[allow(dead_code)]
fn a(mut path: PathBuf) {
    use a::{stdin_reader::*, *};
    path.push("a.txt");

    let f = File::open(path).unwrap();
    solve(StdinReader::new(BufReader::new(f)));
}

#[allow(dead_code)]
fn b(mut path: PathBuf) {
    use b::{stdin_reader::*, *};
    path.push("b.txt");

    let f = File::open(path).unwrap();
    solve(StdinReader::new(BufReader::new(f)));
}

#[allow(dead_code)]
fn c(mut path: PathBuf) {
    use c::{stdin_reader::*, *};
    path.push("c.txt");

    let f = File::open(path).unwrap();
    solve(StdinReader::new(BufReader::new(f)));
}

#[allow(dead_code)]
fn d(mut path: PathBuf) {
    use d::{stdin_reader::*, *};
    path.push("d.txt");

    let f = File::open(path).unwrap();
    solve(StdinReader::new(BufReader::new(f)));
}

#[allow(dead_code)]
fn e(mut path: PathBuf) {
    use e::{stdin_reader::*, *};
    path.push("e.txt");

    let f = File::open(path).unwrap();
    solve(StdinReader::new(BufReader::new(f)));
}

#[allow(dead_code)]
fn f(mut path: PathBuf) {
    use f::{stdin_reader::*, *};
    path.push("f.txt");

    let f = File::open(path).unwrap();
    solve(StdinReader::new(BufReader::new(f)));
}