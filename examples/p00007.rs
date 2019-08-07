use std::io;
use std::cmp::{min, max};
use std::collections::BTreeMap;
use std::fmt;

const MAX_U16: u16 = !0u16;

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

///y equal, x not equal
#[derive(Debug, Clone, PartialEq)]
struct XLine {
    y: u16,
    x: (u16, u16),
}

#[derive(Debug, PartialEq)]
struct XYSide {
    z: u16,
    lines: BTreeMap<u16, Vec<XLine>>,
    min: (u16, u16),
    max: (u16, u16),
}

#[derive(Debug, PartialEq)]
struct Bulk {
    sides: BTreeMap<u16, Vec<XYSide>>,
    min: (u16, u16, u16),
    max: (u16, u16, u16),
}

#[derive(PartialEq)]
struct XYSideFill {
    min: (u16, u16),
    max: (u16, u16),
    fill: Vec<Vec<u8>>,
}

impl fmt::Debug for XYSideFill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let y_len = self.max.1 - self.min.1;
        let x_len = self.max.0 - self.min.0;

        writeln!(f, "({:?}, {:?})", self.min, self.max)?;
        for i in 0..y_len {
            let i = y_len - i - 1;
            for j in 0..x_len {
                write!(f, "{}", self.fill[i as usize][j as usize])?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}

fn process(side_count: u32) {
    let mut buf = String::new();

    let mut sides = Vec::new();
    for _i in 0..side_count {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        if let Some(side) = build_side(&buf) {
            sides.push(side);
        }
    }

    let bulk = build_bulk(sides);

    //println!("{:?}", bulk);

    let units = get_bulk_units(bulk);


    println!("The bulk is composed of {} units.", units);
}

fn get_bulk_units(bulk: Bulk) -> u32 {
    let y_len = bulk.max.1 - bulk.min.1;
    let x_len = bulk.max.0 - bulk.min.0;
    let z_len = bulk.max.2 - bulk.min.2;

    let z_offset = bulk.min.2;

    let mut fill = Vec::with_capacity(y_len as usize);
    for _i in 0..y_len {
        fill.push(vec![0u8; x_len as usize]);
    }

    let mut total_units = 0u32;

    for k in 0..z_len {
        let z = z_offset + k;

        let side = bulk.sides.get(&z);
        match side {
            Some(side) => {
                for one_side in side {
                    let side_fill = build_side_fill(one_side);
                    update_fill(&mut fill, bulk.min, &side_fill);
                }
            }
            None => {}
        }
//        println!("{:?}", XYSideFill{
//            min: (bulk.min.0, bulk.min.1),
//            max: (bulk.max.0, bulk.max.1),
//            fill: fill.clone(),
//        });
        total_units = total_units + get_fill_units(&fill);
    }

    total_units
}

fn get_fill_units(fill: &Vec<Vec<u8>>) -> u32 {
    let mut units = 0u32;
    for i in fill {
        for j in i {
            units = units + (*j as u32);
        }
    }
    units
}

fn update_fill(fill: &mut Vec<Vec<u8>>, min: (u16, u16, u16), side_fill: &XYSideFill) {
    let y_len = side_fill.max.1 - side_fill.min.1;
    let x_len = side_fill.max.0 - side_fill.min.0;

    let y_offset = side_fill.min.1 - min.1;
    let x_offset = side_fill.min.0 - min.0;

    for i in 0..y_len {
        for j in 0..x_len {
            let fill_i = i + y_offset;
            let fill_j = j + x_offset;
            let side_filled = side_fill.fill[i as usize][j as usize];
            if side_filled == 1 {
                fill[fill_i as usize][fill_j as usize] = 1 - fill[fill_i as usize][fill_j as usize];
            }
        }
    }
}

fn build_side_fill(side: &XYSide) -> XYSideFill {
    let y_len = side.max.1 - side.min.1;
    let x_len = side.max.0 - side.min.0;

    let mut fill = Vec::with_capacity(y_len as usize);

    for _i in 0..y_len {
        fill.push(vec![0u8; x_len as usize]);
    }

    let x_offset = side.min.0;
    let y_offseet = side.min.1;

    for i in 0..y_len {
        let y = y_offseet + i;
        let line = side.lines.get(&y);

        //copy last y fill first
        for j in 0..x_len {
            if i == 0 {
                fill[i as usize][j as usize] = 0;
            } else {
                fill[i as usize][j as usize] = fill[i as usize - 1][j as usize];
            }
        }
        match line {
            Some(line) => {
                //fill according to line
                for one_line in line {
                    for j in (one_line.x.0 - x_offset)..(one_line.x.1 - x_offset) {
                        fill[i as usize][j as usize] = 1 - fill[i as usize][j as usize];
                    }
                }
            }
            None => {}
        }
    }

    XYSideFill {
        min: side.min,
        max: side.max,
        fill,
    }
}

fn build_bulk(sides: Vec<XYSide>) -> Bulk {
    let mut min_z = MAX_U16;
    let mut max_z = 0u16;

    let mut min_x = MAX_U16;
    let mut max_x = 0u16;

    let mut min_y = MAX_U16;
    let mut max_y = 0u16;

    for side in &sides {
        min_z = min(side.z, min_z);
        max_z = max(side.z, max_z);

        min_x = min(side.min.0, min_x);
        max_x = max(side.max.0, max_x);

        min_y = min(side.min.1, min_y);
        max_y = max(side.max.1, max_y);
    }

    let mut btmap_sides = BTreeMap::new();

    for side in sides {
        let e = btmap_sides.entry(side.z).or_insert(Vec::new());
        e.push(side);
    }

    Bulk {
        sides: btmap_sides,
        min: (min_x, min_y, min_z),
        max: (max_x, max_y, max_z),
    }
}

fn build_side(buf: &str) -> Option<XYSide> {
    let buf = buf.trim();

    let split = buf.split("  ");

    let mut point_count = 0;
    let mut points: Vec<(u16, u16, u16)> = Vec::new();

    let mut min_z = MAX_U16;
    let mut max_z = 0u16;

    let mut min_x = MAX_U16;
    let mut max_x = 0u16;

    let mut min_y = MAX_U16;
    let mut max_y = 0u16;

    for (i, v) in split.enumerate() {
        if i == 0 {
            point_count = v.parse().unwrap();
        } else {
            if i > point_count {
                break;
            }
            let point = build_point(v);

            let x = point.0;
            let y = point.1;
            let z = point.2;

            min_z = min(min_z, z);
            max_z = max(max_z, z);

            min_x = min(min_x, x);
            max_x = max(max_x, x);

            min_y = min(min_y, y);
            max_y = max(max_y, y);

            points.push(point);
        }
    }
    if min_z != max_z {
        return None;
    }

    let mut lines: BTreeMap<u16, Vec<XLine>> = BTreeMap::new();
    let len = points.len();
    for i in 0..len {
        let point1 = points[i];
        let point2 = points[(i + 1) % len];
        if point1.1 == point2.1 {
            let min_x = min(point1.0, point2.0);
            let max_x = max(point1.0, point2.0);
            let e = lines.entry(point1.1).or_insert(Vec::new());

            e.push(XLine {
                y: point1.1,
                x: (min_x, max_x),
            });
        }
    }

    Some(XYSide {
        z: min_z,
        lines,
        min: (min_x, min_y),
        max: (max_x, max_y),
    })
}

fn build_point(buf: &str) -> (u16, u16, u16) {
    let buf = buf.trim();
    let split = buf.split(" ");
    let (mut x, mut y, mut z) = (0, 0, 0);
    for (i, v) in split.enumerate() {
        let v = v.parse().unwrap();
        match i {
            0 => x = v,
            1 => y = v,
            _ => z = v,
        }
    }
    (x, y, z)
}

fn sort<T, F>(vs: &mut Vec<T>, is_lte: &mut F)
    where F: FnMut(&T, &T) -> bool {
    let len = vs.len();
    for i in 1..len {
        let i = len - i;
        for j in 0..i {
            if !is_lte(&vs[j], &vs[j + 1]) {
                vs.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    read_from_stdin(&mut buf).unwrap();
    let len: usize = buf.trim().parse().unwrap();

    for _i in 0..len {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let trimed = buf.trim();
        let side_count = trimed.parse().unwrap();
        process(side_count);
    }

//    test();
}

fn make_btmap<K, V>(v: Vec<(K, V)>) -> BTreeMap<K, V>
    where K: Ord {
    let mut map = BTreeMap::new();
    for i in v {
        map.insert(i.0, i.1);
    }
    map
}

fn test() {
    assert_eq!(65535u16, MAX_U16);

    let side = build_side("4  10 10 10  10 10 20  10 20 20  10 20 10");
    assert_eq!(None, side);

    let side = build_side("4  10 10 10  10 20 10  20 20 10  20 10 10");

    assert_eq!(Some(XYSide {
        z: 10,
        lines: make_btmap::<_, _>(vec![
            (10, vec![XLine { y: 10, x: (10, 20) }]),
            (20, vec![XLine { y: 20, x: (10, 20) }]),
        ]),
        min: (10, 10),
        max: (20, 20),
    }), side);

    let mut vs = vec![1, 3, 2, 4, 8, 7, 6, 5];
    sort::<_, _>(&mut vs, &mut |a, b| a <= b);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], vs);

    let sides = vec![
        build_side("4  10 10 10  10 20 10  20 20 10  20 10 10").unwrap(),
        build_side("5  10 10 20  10 20 20  20 20 20  20 15 20  20 10 20").unwrap(),
        build_side("4  14 14 14  14 16 14  16 16 14  16 14 14").unwrap(),
        build_side("4  14 14 16  14 16 16  16 16 16  16 14 16").unwrap(),
    ];

    let bulk = build_bulk(sides);
    let mut lines1 = BTreeMap::new();
    lines1.insert(10, vec![XLine { y: 10, x: (10, 20) }]);
    lines1.insert(20, vec![XLine { y: 20, x: (10, 20) }]);

    let mut lines2 = BTreeMap::new();
    lines2.insert(14, vec![XLine { y: 14, x: (14, 16) }]);
    lines2.insert(16, vec![XLine { y: 16, x: (14, 16) }]);
    assert_eq!(Bulk {
        sides: make_btmap(vec![
            (10, vec![XYSide {
                z: 10,
                lines: make_btmap(vec![
                    (10, vec![XLine { y: 10, x: (10, 20) }]),
                    (20, vec![XLine { y: 20, x: (10, 20) }]),
                ]),
                min: (10, 10),
                max: (20, 20),
            }]),
            (14, vec![XYSide {
                z: 14,
                lines: make_btmap(vec![
                    (14, vec![XLine { y: 14, x: (14, 16) }]),
                    (16, vec![XLine { y: 16, x: (14, 16) }]),
                ]),
                min: (14, 14),
                max: (16, 16),
            }]),
            (16, vec![XYSide {
                z: 16,
                lines: lines2,
                min: (14, 14),
                max: (16, 16),
            }]),
            (20, vec![XYSide {
                z: 20,
                lines: make_btmap(vec![
                    (10, vec![XLine { y: 10, x: (10, 20) }]),
                    (20, vec![XLine { y: 20, x: (10, 20) }]),
                ]),
                min: (10, 10),
                max: (20, 20),
            }]),
        ]),
        min: (10, 10, 10),
        max: (20, 20, 20),
    }, bulk);

    let side = build_side("6  10 10 20  20 10 20  20 30 20  30 30 20  30 40 20  10 40 20");
    let side_fill = build_side_fill(&side.unwrap());
    assert_eq!(XYSideFill {
        min: (10, 10),
        max: (30, 40),
        fill: vec![vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]],
    }, side_fill);
    println!("{:?}", side_fill);
}