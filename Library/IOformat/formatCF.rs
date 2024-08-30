#![allow(non_snake_case)]
use std::io::{BufWriter, stdin, stdout, Write};

fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let t: usize = scan.next();
    for _ in 0..t {}
}

#[derive(Default)]
pub struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.next::<T>()).collect()
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        self.next::<String>().bytes().collect()
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.next::<String>().chars().collect()
    }
}