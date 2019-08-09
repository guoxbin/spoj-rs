use std::io;
use std::env::args;
use std::cmp::max;

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
    let mut result = Num::parse(1);
    for i in 1..=n {
        result = multiple(&result, &Num::parse(i as u128));
//        println!("{} {:?}", i, result);
    }
    result.zero_count
}

#[derive(PartialEq, Debug)]
struct Num {
    num: u128,
    zero_count: u32,
}

impl Num {
    fn parse(mut n: u128) -> Self {
        assert_ne!(n, 0);
        let mut end_with_zero = true;
        let mut count = 0;
        loop {
            let end = n % 10;
            end_with_zero = end == 0;
            if !end_with_zero {
                break;
            }
            n = n / 10;
            count = count + 1;
        }

        let n = compress(n);

        Num {
            num: n,
            zero_count: count,
        }
    }
}

fn compress(n: u128) -> u128 {
    n % 100000000
}

fn multiple(a: &Num, b: &Num) -> Num {
    let mut result = a.num * b.num;
    let mut result_zero_count = a.zero_count + b.zero_count;

    let result = Num::parse(result);

    result_zero_count = result_zero_count + result.zero_count;
    let result = result.num;

    Num {
        num: result,
        zero_count: result_zero_count,
    }
}

fn test() {
    println!("test");

    assert_eq!(Num { num: 3, zero_count: 2 }, Num::parse(300));
    assert_eq!(Num { num: 45, zero_count: 3 }, Num::parse(45000));
    assert_eq!(Num { num: 1, zero_count: 0 }, Num::parse(1));
    assert_eq!(Num { num: 123, zero_count: 5 }, Num::parse(12300000));
    assert_eq!(Num { num: 1, zero_count: 1 }, Num::parse(10));

    assert_eq!(Num { num: 1002, zero_count: 1 }, Num::parse(10020));

    assert_eq!(Num {
        num: 135,
        zero_count: 5,
    }, multiple(&Num::parse(300), &Num::parse(45000)));

    assert_eq!(Num {
        num: 18,
        zero_count: 6,
    }, multiple(&Num::parse(400), &Num::parse(45000)));

    assert_eq!(14, factorial(60));

    assert_eq!(24, factorial(100));

    assert_eq!(253, factorial(1024));

    assert_eq!(5861, factorial(23456));

    assert_eq!(2183837, factorial(8735373));

    assert_eq!(234567890, compress(123456789012345678901234567890));
}