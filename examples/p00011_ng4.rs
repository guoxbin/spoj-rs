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
    get_factorial_prime_factor_5_count(n)
}

fn get_factorial_prime_factor_5_count(n: u32) -> u32 {
    let mut whole_count = 0;
    for i in 1..=n {
        let count = get_prime_factor_5_count(i);
        whole_count = whole_count + count;
    }
    whole_count
}

fn get_prime_factor_5_count(n: u32) -> u32 {
    let mut n = n;

    let mut count = 0;
    while n % 5 == 0 {
        count = count + 1;
        n = n / 5;
    }

    count
}

fn test() {
    println!("test");

    assert_eq!(0, get_prime_factor_5_count(8));
    assert_eq!(1, get_prime_factor_5_count(10));
    assert_eq!(1, get_prime_factor_5_count(20));
    assert_eq!(0, get_prime_factor_5_count(1024));
    assert_eq!(3, get_prime_factor_5_count(1000));

    assert_eq!(0, get_factorial_prime_factor_5_count(3));

    assert_eq!(get_prime_factor_5_count(3628800), get_factorial_prime_factor_5_count(10));

    assert_eq!(0, factorial(3));

    assert_eq!(6, factorial(25));

    assert_eq!(14, factorial(60));

    assert_eq!(24, factorial(100));

    assert_eq!(2183837, factorial(8735373));
}