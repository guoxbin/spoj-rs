use std::io;
use std::env::args;
use std::io::Read;
use std::cmp::{min, max};

const MAX: u32 = !0u32;

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

#[derive(Debug)]
struct Order {
    start: usize,
    duration: usize,
    price: usize,
}

fn process() {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let count = buf.trim().parse().unwrap();

    let mut order_list = Vec::with_capacity(count);

    for i in 0..count {
        let mut buf = String::new();
        read_from_stdin(&mut buf).unwrap();
        let a: Vec<&str> = buf.trim().split(" ").collect();
        order_list.push(Order {
            start: a[0].parse().unwrap(),
            duration: a[1].parse().unwrap(),
            price: a[2].parse().unwrap(),
        });
    }

    let result = most_income(&order_list);

    println!("{:?}", result);
}

fn most_income(order_list: &Vec<Order>) -> usize {
    let mut max_end_time = 0usize;
    for i in order_list {
        max_end_time = max(max_end_time, i.start + i.duration);
    }

    let mut cache = vec![0usize; max_end_time + 1];

    for i in 0..=max_end_time {
        if i == 0 {
            cache[i] = 0;
        } else {
            let mut max_income = 0usize;
            for order in order_list {
                let end_time = order.start + order.duration;
                if i >= end_time {
                    let pre_end_time = order.start;
                    let pre_max_income = cache[pre_end_time];
                    let current_max_income = pre_max_income + order.price;

                    max_income = max(max_income, current_max_income);
                }
            }
            cache[i] = max_income;
        }
    }

    cache[max_end_time]
}

fn test() {
    println!("test");

    println!("{}", MAX);

    println!("{}", most_income(&vec![Order { start: 0, duration: 5, price: 10 }, Order { start: 3, duration: 7, price: 14 }, Order { start: 5, duration: 9, price: 7 }, Order { start: 6, duration: 9, price: 8 }]));
}