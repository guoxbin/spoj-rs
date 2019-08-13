use std::io;
use std::env::args;
use std::io::Read;
use std::cmp::{min, max};
use std::time;

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
    //println!("{:?}", time::SystemTime::now());

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

    //println!("loaded: {:?}", time::SystemTime::now());

    let result = most_income(&mut order_list);

    //println!("done: {:?}", time::SystemTime::now());

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

fn search_order(order_list: &Vec<Order>, l: usize, r: usize, end_time: usize) -> Option<usize> {
    if l > r {
        return None;
    }
    let i = (l + r) / 2;

    let i_end_time = order_list[i].start + order_list[i].duration;
    let i1_end_time = if i >= order_list.len() - 1
    { end_time + 1 } else { order_list[i + 1].start + order_list[i + 1].duration };

    if i_end_time <= end_time
        && i1_end_time > end_time {
        return Some(i);
    } else if i_end_time > end_time {
        if i <= 0 {
            return None;
        }
        return search_order(order_list, l, i - 1, end_time);
    } else {
        return search_order(order_list, i + 1, r, end_time);
    }
}

fn most_income(order_list: &mut Vec<Order>) -> usize {
    let len = order_list.len();

    quick_sort(order_list, 0, len - 1);

    //println!("sorted: {:?}", time::SystemTime::now());

    let mut cache = vec![0usize; len + 1];

    for i in 0..=order_list.len() {
        if i == 0 {
            cache[i] = 0;
        } else {

            //not use
            let not_use_most_income = cache[i - 1];

            //use
            let mut searched = search_order(order_list, 0, i - 1, order_list[i - 1].start);

            let pre_i = match searched {
                None => 0,
                Some(searched) => searched + 1,
            };

            let use_most_income = cache[pre_i] + order_list[i - 1].price;

            cache[i] = max(use_most_income, not_use_most_income);
        }
    }
    cache[order_list.len()]
}

fn test() {
    println!("test");

    let mut order_list = vec![
        Order { start: 0, duration: 5, price: 10 },
        Order { start: 3, duration: 7, price: 14 },
        Order { start: 5, duration: 9, price: 7 },
        Order { start: 6, duration: 9, price: 8 },
    ];

    assert_eq!(18, most_income(&mut order_list));
}