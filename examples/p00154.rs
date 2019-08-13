use std::io;
use std::env::args;
use std::io::Read;
use std::cmp::{min, max};
use std::time;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 2 && args[1] == "test".to_string() {
        test();
        return;
    }

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();

    let count: u32 = buf.trim().parse().unwrap();

    for i in 0..count {
        process();
    }
}

fn str_to_sweet(s: &str) -> Vec<u8> {
    let len = s.len();
    let mut sweet = vec![0u8; len];
    let bytes = s.as_bytes();
    for i in 0..len {
        let c = bytes[i];
        let b = c - '0' as u8;
        sweet[i] = b;
    }
    sweet
}

fn process() {

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();

    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();

    let s = buf.trim();

    let result = most_part(&str_to_sweet(s));

    println!("{}", result);

}

fn most_part(sweet: &Vec<u8>) -> u8 {
    let mut cache = vec![0u8; sweet.len() + 1];

    for i in 0..=sweet.len() {
        if i == 0 {
            cache[i] = 0;
        } else {
            //for i-1..=0
            let available = get_available((0..=(i - 1)).rev().map(|x| (x, sweet[x])));

            let mut max_whole_length = cache[i-1];
            for (length, index) in available {
                let pre_length = cache[index];
                let whole_length = length + pre_length;
                max_whole_length = max(max_whole_length, whole_length);
            }
            if max_whole_length > 0 {
                cache[i] = max_whole_length;
            } else {
                cache[i] = cache[i - 1];
            }
        }
//        println!("{} {}", i, cache[i]);
    }

    cache[sweet.len()]
}

fn get_available<It>(it: It) -> Vec<(u8, usize)>
    where It: IntoIterator<Item=(usize, u8)> {
    let mut length = 0;
    let mut net_sum = 0;
    let mut result = Vec::new();
    for (i, b) in it {
        net_sum = net_sum + (if (b == 0) { -1 } else { 1 });
        length = length + 1;
        if net_sum > 0 {
            result.push((length, i));
        }
    }
    result
}

fn test() {
    println!("test");

    assert_eq!(vec![1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1], str_to_sweet("100110001010001"));

    let a = vec![1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1];

    assert_eq!(vec![(1, 10), (3, 8)], get_available((0..=10).rev().map(|x| (x, a[x]))));

    assert_eq!(vec![(1, 4), (2, 3), (3, 2), (5, 0)], get_available((0..=4).rev().map(|x| (x, a[x]))));

    assert_eq!(9, most_part(&str_to_sweet("100110001010001")));

    assert_eq!(13, most_part(&str_to_sweet("0010111101100000")));
}