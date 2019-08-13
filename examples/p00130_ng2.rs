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

    let result = run_most_income(&order_list);

    println!("{:?}", result);
}

fn run_most_income(order_list: &Vec<Order>) -> usize {
    let mut max_end_time = 0usize;
    for i in order_list {
        max_end_time = max(max_end_time, i.start + i.duration);
    }

    let use_list_len = (order_list.len() - 1) / 8 + 1;

    let mut use_list = vec![0u8; use_list_len+1];

    most_income(order_list, max_end_time, &mut use_list)
}

fn most_income(order_list: &Vec<Order>, end_time: usize, use_list: &mut Vec<u8>) -> usize {

    //(most_income, i)
    let mut all_most_income: Option<(usize, usize)> = None;

    for i in 0..order_list.len() {
        let order = &order_list[i];
        let use_list_index = if i == 0 { 0 } else { (i - 1) / 8 + 1 };
        let of_use_list = use_list[use_list_index];
        let digit = i % 8;
        let mask = 1u8 << (digit as u8);

        let used = of_use_list & mask > 0;

        if !used {
            if order.start + order.duration <= end_time {
                let pre_end_time = order.start;
                let pre_of_use_list = of_use_list | mask;
                use_list[use_list_index] = pre_of_use_list;
                let pre_most_income = most_income(order_list, pre_end_time, use_list);
                use_list[use_list_index] = of_use_list;

                let current_most_income = pre_most_income + order.price;

                all_most_income = Some(match all_most_income {
                    None => (current_most_income, i),
                    Some((current_most_income2, order_id2)) => {
                        if current_most_income > current_most_income2 {
                            (current_most_income, i)
                        } else if current_most_income < current_most_income2 {
                            (current_most_income2, order_id2)
                        } else {
                            //TODO
                            (current_most_income, i)
                        }
                    }
                });
            }
        }
    }

    match all_most_income {
        Some((most_income, i)) => most_income,
        None => 0,
    }
}

fn test() {
    println!("test");

    println!("{}", MAX);

    assert_eq!(18, run_most_income(
        &vec![Order { start: 0, duration: 5, price: 10 }, Order { start: 3, duration: 7, price: 14 }, Order { start: 5, duration: 9, price: 7 }, Order { start: 6, duration: 9, price: 8 }]
    )
    );
}