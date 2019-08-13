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

#[derive(Debug, Clone)]
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

    let result = most_income(&mut order_list);

    println!("{:?}", result);
}

fn quick_sort(nums: &mut Vec<Order>, left: usize, right: usize) {
    if left >= right {
        return;
    }

    let mut l = left;
    let mut r = right;
    while l < r {
        while l < r && nums[r].start + nums[r].duration >= nums[left].start + nums[left].duration {
            r -= 1;
        }
        while l < r && nums[l].start + nums[l].duration <= nums[left].start + nums[left].duration {
            l += 1;
        }

        nums.swap(l, r);
    }
    nums.swap(left, l);
    if l > 1 {
        quick_sort(nums, left, l - 1);
    }

    quick_sort(nums, r + 1, right);
}

fn sort(order_list: &mut Vec<Order>) {
    for i in 0..order_list.len() - 1 {
        for j in i + 1..order_list.len() {
            let should_swap = {
                let a = &order_list[i];
                let b = &order_list[j];
                a.start + a.duration > b.start + b.duration
            };

            if should_swap {
                order_list.swap(i, j);
            }
        }
    }
}

fn get_max_end_time(order_list: &Vec<Order>) -> usize {
    let mut max_end_time = 0usize;
    for i in order_list {
        max_end_time = max(max_end_time, i.start + i.duration);
    }
    max_end_time
}

fn search_order_before_end_time(order_list: &Vec<Order>, end_time: usize) -> Option<usize> {
    let (mut st, mut en) = (0, order_list.len());

    loop {
        //println!("{} {}", st, en);
        if st == 2048 && en == 2047 {
            return None;
        }
        let i = (st + en) / 2;
        let order = &order_list[i];
        let order_end_time = order.start + order.duration;

        if i + 1 >= order_list.len() {
            if order_end_time <= end_time {
                return Some(i);
            } else {
                if i < 1 {
                    return None;
                }
                en = i - 1;
            }
        } else {
            let next_order_end_time = {
                let o = &order_list[i + 1];
                o.start + o.duration
            };

            if order_end_time <= end_time && next_order_end_time > end_time {
                return Some(i);
            } else if next_order_end_time <= end_time {
                if i + 1 >= order_list.len() {
                    return None;
                }
                st = i + 1;
            } else {
                if i < 1 {
                    return None;
                }
                en = i - 1;
            }
        }
    }
}

fn most_income(order_list: &mut Vec<Order>) -> usize {
    let len = order_list.len();

    quick_sort(order_list, 0, len - 1);

    let last = &order_list[len - 1];
    let mut max_end_time = last.start + last.duration;

    let mut cache = vec![0usize; max_end_time + 1];

    for i in 0..=max_end_time {
        if i == 0 {
            cache[i] = 0;
        } else {
            let mut max_income = 0usize;

            let find_order_id = search_order_before_end_time(order_list, i).unwrap_or(0);

            println!("{}", find_order_id);

            for order_id in 0..=find_order_id {
                let order = &order_list[order_id];
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

    let mut order_list = vec![
        Order { start: 0, duration: 5, price: 10 },
        Order { start: 3, duration: 7, price: 14 },
        Order { start: 5, duration: 9, price: 7 },
        Order { start: 6, duration: 9, price: 8 },
    ];

    for i in 0..11 {
        order_list.extend(order_list.clone());
    }
    println!("order_list len: {}", order_list.len());

    let len = order_list.len();

//    quick_sort(&mut order_list, 0, len - 1);

//    println!("sorted: {:?}", order_list);

//    let a = search_order_before_end_time(&order_list, 10);

//    assert_eq!(Some(4095), a);

    assert_eq!(18, most_income(
        &mut order_list
    )
    );
}