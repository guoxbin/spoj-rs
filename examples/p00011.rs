use std::io;
use std::env::args;
use std::cmp::{max, min};
use std::collections::HashMap;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn process(buf: &str) {
    let f = factorial(buf.parse().unwrap());
    println!("{}", f);
}

/// reference: https://brilliant.org/wiki/trailing-number-of-zeros/
fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 && args[1] == "test".to_string() {
        test();
        return;
    }

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let trimed = buf.trim();

        process(trimed);
    }
}

fn factorial(n: u32) -> u32 {
    let mut m = 5;

    let mut sum = 0;
    while m <= n {

        //println!("{}", m);

        sum = sum + n/m;

        m = m * 5
    }

    sum
}

fn test() {
    println!("test");

    assert_eq!(0, factorial(3));

    assert_eq!(6, factorial(25));

    assert_eq!(14, factorial(60));

    assert_eq!(24, factorial(100));

    assert_eq!(2183837, factorial(8735373));
}