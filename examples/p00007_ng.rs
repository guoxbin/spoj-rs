use std::io;
use std::cmp::{min, max};

fn read_from_stdin(buf: &mut String) -> io::Result<usize> {
    io::stdin().read_line(buf)
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
    z: u8,
}

#[derive(Debug)]
struct Side {
    points: Vec<Point>,
    mode: SideMode,
}

#[derive(Debug)]
enum SideMode {
    X {
        x: u8,
        min_y: u8,
        min_z: u8,
        max_y: u8,
        max_z: u8,
    },
    Y {
        y: u8,
        min_x: u8,
        min_z: u8,
        max_x: u8,
        max_z: u8,
    },
    Z {
        z: u8,
        min_x: u8,
        min_y: u8,
        max_x: u8,
        max_y: u8,
    },
}

#[derive(Debug)]
struct Bulk {
    sides: Vec<Side>,
    mode: BulkMode,
}

#[derive(Debug)]
struct BulkMode {
    min_x: u8,
    min_y: u8,
    min_z: u8,
    max_x: u8,
    max_y: u8,
    max_z: u8,
}

#[derive(Debug, PartialEq)]
enum PointSideRelation {
    Positive,
    Negative,
    Off,
}

fn process(side_count: u32) {
    let mut buf = String::new();

    let mut sides = Vec::new();
    for _i in 0..side_count {
        buf.clear();
        read_from_stdin(&mut buf).unwrap();
        let side = build_side(&buf);
        sides.push(side);
    }
    let bulk = build_bulk(sides);
    //println!("{:?}", bulk);
    let units = get_units(bulk);
    println!("The bulk is composed of {} units.", units);
}

fn get_units(bulk: Bulk) -> u32 {
    let mut units = 0;
    for x in bulk.mode.min_x..bulk.mode.max_x {
        for y in bulk.mode.min_y..bulk.mode.max_y {
            for z in bulk.mode.min_z..bulk.mode.max_z {
                let point = Point { x, y, z };
                let is_in_bulk = is_in_bulk(&point, &bulk);
                if is_in_bulk {
                    units = units + 1;
                }
            }
        }
    }
    units
}

fn is_in_bulk(point: &Point, bulk: &Bulk) -> bool {
    let mut x_positive_count = 0;
    let mut x_negative_count = 0;

    let mut y_positive_count = 0;
    let mut y_negative_count = 0;

    let mut z_positive_count = 0;
    let mut z_negative_count = 0;

    for side in &bulk.sides {
        match side.mode {
            SideMode::X { .. } => {
                let r = get_point_side_relation(point, side);
                match r {
                    PointSideRelation::Positive => {
                        x_positive_count = x_positive_count + 1;
                    }
                    PointSideRelation::Negative => {
                        x_negative_count = x_negative_count + 1;
                    }
                    _ => {}
                }
            }
            SideMode::Y { .. } => {
                let r = get_point_side_relation(point, side);
                match r {
                    PointSideRelation::Positive => {
                        y_positive_count = y_positive_count + 1;
                    }
                    PointSideRelation::Negative => {
                        y_negative_count = y_negative_count + 1;
                    }
                    _ => {}
                }
            }
            SideMode::Z { .. } => {
                let r = get_point_side_relation(point, side);
                match r {
                    PointSideRelation::Positive => {
                        z_positive_count = z_positive_count + 1;
                    }
                    PointSideRelation::Negative => {
                        z_negative_count = z_negative_count + 1;
                    }
                    _ => {}
                }
            }
        }
    }

    //println!("{} {} {} {}", x_positive_count, x_negative_count, z_positive_count, z_negative_count);

    if x_positive_count % 2 != 0 && x_negative_count % 2 != 0 && z_positive_count % 2 != 0 && z_negative_count % 2 != 0 {
        return true;
    }

    false
}

fn get_point_side_relation(point: &Point, side: &Side) -> PointSideRelation {
    match &side.mode {
        SideMode::X { x, .. } => {
            if &point.x >= x {
                if is_on_side(&Point { x: *x, y: point.y, z: point.z }, side) {
                    return PointSideRelation::Positive;
                } else {
                    return PointSideRelation::Off;
                }
            } else {
                if is_on_side(&Point { x: *x, y: point.y, z: point.z }, side) {
                    return PointSideRelation::Negative;
                } else {
                    return PointSideRelation::Off;
                }
            }
        }
        SideMode::Y { y, .. } => {
            if &point.y >= y {
                if is_on_side(&Point { x: point.x, y: *y, z: point.z }, side) {
                    return PointSideRelation::Positive;
                } else {
                    return PointSideRelation::Off;
                }
            } else {
                if is_on_side(&Point { x: point.x, y: *y, z: point.z }, side) {
                    return PointSideRelation::Negative;
                } else {
                    return PointSideRelation::Off;
                }
            }
        }
        SideMode::Z { z, .. } => {
            if &point.z >= z {
                if is_on_side(&Point { x: point.x, y: point.y, z: *z }, side) {
                    return PointSideRelation::Positive;
                } else {
                    return PointSideRelation::Off;
                }
            } else {
                if is_on_side(&Point { x: point.x, y: point.y, z: *z }, side) {
                    return PointSideRelation::Negative;
                } else {
                    return PointSideRelation::Off;
                }
            }
        }
    }
}

fn is_on_side(point: &Point, side: &Side) -> bool {
    let len = side.points.len();

    match side.mode {
        SideMode::X { .. } => {
            //_positive_count count for lines locating on the positive direction of the point
            //_negative_count count for lines location on the negative direction of the point

            let mut y_positive_count = 0;
            let mut y_negative_count = 0;

            let mut z_positive_count = 0;
            let mut z_negative_count = 0;

            for i in 0..len {
                //point1 and point2 stand for a line
                let point1 = &side.points[i];
                let point2 = &side.points[(i + 1) % len];
                //println!("h: {} {} {:?} {:?} vs {:?}", i, len, point1, point2, point);

                //y direction
                if point1.y == point2.y {
                    let min_z = min(point1.z, point2.z);
                    let max_z = max(point1.z, point2.z);
                    if point.z >= min_z && point.z < max_z {
                        if point1.y > point.y {
                            y_positive_count = y_positive_count + 1;
                        } else {
                            y_negative_count = y_negative_count + 1;
                        }
                    }
                }

                //z direction
                if point1.z == point2.z {
                    let min_y = min(point1.y, point2.y);
                    let max_y = max(point1.y, point2.y);
                    if point.y >= min_y && point.y < max_y {
                        if point1.z > point.z {
                            z_positive_count = z_positive_count + 1;
                        } else {
                            z_negative_count = z_negative_count + 1;
                        }
                    }
                }
            }

            //println!("{} {} {} {}", y_positive_count, y_negative_count, z_positive_count, z_negative_count);

            if y_positive_count % 2 != 0 && y_negative_count % 2 != 0 && z_positive_count % 2 != 0 && z_negative_count % 2 != 0 {
                return true;
            }
        }
        SideMode::Y { .. } => {
            let mut x_positive_count = 0;
            let mut x_negative_count = 0;

            let mut z_positive_count = 0;
            let mut z_negative_count = 0;

            for i in 0..len {
                //point1 and point2 stand for a line
                let point1 = &side.points[i];
                let point2 = &side.points[(i + 1) % len];
                //println!("h: {} {} {:?} {:?} vs {:?}", i, len, point1, point2, point);

                //x direction
                if point1.x == point2.x {
                    let min_z = min(point1.z, point2.z);
                    let max_z = max(point1.z, point2.z);
                    if point.z >= min_z && point.z < max_z {
                        if point1.x > point.x {
                            x_positive_count = x_positive_count + 1;
                        } else {
                            x_negative_count = x_negative_count + 1;
                        }
                    }
                }

                //z direction
                if point1.z == point2.z {
                    let min_x = min(point1.x, point2.x);
                    let max_x = max(point1.x, point2.x);
                    if point.x >= min_x && point.x < max_x {
                        if point1.z > point.z {
                            z_positive_count = z_positive_count + 1;
                        } else {
                            z_negative_count = z_negative_count + 1;
                        }
                    }
                }
            }

            //println!("{} {} {} {}", x_positive_count, x_negative_count, z_positive_count, z_negative_count);

            if x_positive_count % 2 != 0 && x_negative_count % 2 != 0 && z_positive_count % 2 != 0 && z_negative_count % 2 != 0 {
                return true;
            }
        }
        SideMode::Z { .. } => {
            let mut x_positive_count = 0;
            let mut x_negative_count = 0;

            let mut y_positive_count = 0;
            let mut y_negative_count = 0;

            for i in 0..len {
                //point1 and point2 stand for a line
                let point1 = &side.points[i];
                let point2 = &side.points[(i + 1) % len];
                //println!("h: {} {} {:?} {:?} vs {:?}", i, len, point1, point2, point);

                //x direction
                if point1.x == point2.x {
                    let min_y = min(point1.y, point2.y);
                    let max_y = max(point1.y, point2.y);
                    if point.y >= min_y && point.y < max_y {
                        if point1.x > point.x {
                            x_positive_count = x_positive_count + 1;
                        } else {
                            x_negative_count = x_negative_count + 1;
                        }
                    }
                }

                //y direction
                if point1.y == point2.y {
                    let min_x = min(point1.x, point2.x);
                    let max_x = max(point1.x, point2.x);
                    if point.x >= min_x && point.x < max_x {
                        if point1.y > point.y {
                            y_positive_count = y_positive_count + 1;
                        } else {
                            y_negative_count = y_negative_count + 1;
                        }
                    }
                }
            }

            //println!("{} {} {} {}", x_positive_count, x_negative_count, y_positive_count, y_negative_count);

            if x_positive_count % 2 != 0 && x_negative_count % 2 != 0 && y_positive_count % 2 != 0 && y_negative_count % 2 != 0 {
                return true;
            }
        }
    }

    false
}

fn build_bulk(sides: Vec<Side>) -> Bulk {
    let (mut min_x, mut min_y, mut min_z, mut max_x, mut max_y, mut max_z)
        = (None, None, None, None, None, None);

    for side in &sides {
        for point in &side.points {
            min_x = match min_x {
                Some(min_x) => if point.x < min_x { Some(point.x) } else { Some(min_x) },
                None => Some(point.x),
            };
            min_y = match min_y {
                Some(min_y) => if point.y < min_y { Some(point.y) } else { Some(min_y) },
                None => Some(point.y),
            };
            min_z = match min_z {
                Some(min_z) => if point.z < min_z { Some(point.z) } else { Some(min_z) },
                None => Some(point.z),
            };
            max_x = match max_x {
                Some(max_x) => if point.x > max_x { Some(point.x) } else { Some(max_x) },
                None => Some(point.x),
            };
            max_y = match max_y {
                Some(max_y) => if point.y > max_y { Some(point.y) } else { Some(max_y) },
                None => Some(point.y),
            };
            max_z = match max_z {
                Some(max_z) => if point.z > max_z { Some(point.z) } else { Some(max_z) },
                None => Some(point.z),
            };
        }
    }

    let (min_x, min_y, min_z, max_x, max_y, max_z)
        = (min_x.unwrap(), min_y.unwrap(), min_z.unwrap(), max_x.unwrap(), max_y.unwrap(), max_z.unwrap());

    let mode = BulkMode {
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
    };

    Bulk {
        sides,
        mode,
    }
}

fn build_side(buf: &str) -> Side {
    let buf = buf.trim();

    let split = buf.split("  ");

    let mut point_count = 0;
    let mut points: Vec<Point> = Vec::new();

    let (mut min_x, mut min_y, mut min_z, mut max_x, mut max_y, mut max_z)
        = (None, None, None, None, None, None);

    for (i, a) in split.enumerate() {
        if i == 0 {
            point_count = a.parse().unwrap();
        } else {
            if i <= point_count {
                let point_str = a.split(" ");

                let point = {
                    let mut point = Point { x: 0, y: 0, z: 0 };
                    for (j, b) in point_str.enumerate() {
                        match j {
                            0 => point.x = b.parse().unwrap(),
                            1 => point.y = b.parse().unwrap(),
                            2 => point.z = b.parse().unwrap(),
                            _ => unreachable!(),
                        }
                    }
                    point
                };
                min_x = match min_x {
                    Some(min_x) => if point.x < min_x { Some(point.x) } else { Some(min_x) },
                    None => Some(point.x),
                };
                min_y = match min_y {
                    Some(min_y) => if point.y < min_y { Some(point.y) } else { Some(min_y) },
                    None => Some(point.y),
                };
                min_z = match min_z {
                    Some(min_z) => if point.z < min_z { Some(point.z) } else { Some(min_z) },
                    None => Some(point.z),
                };
                max_x = match max_x {
                    Some(max_x) => if point.x > max_x { Some(point.x) } else { Some(max_x) },
                    None => Some(point.x),
                };
                max_y = match max_y {
                    Some(max_y) => if point.y > max_y { Some(point.y) } else { Some(max_y) },
                    None => Some(point.y),
                };
                max_z = match max_z {
                    Some(max_z) => if point.z > max_z { Some(point.z) } else { Some(max_z) },
                    None => Some(point.z),
                };

                points.push(point);
            }
        }
    }

    let (min_x, min_y, min_z, max_x, max_y, max_z)
        = (min_x.unwrap(), min_y.unwrap(), min_z.unwrap(), max_x.unwrap(), max_y.unwrap(), max_z.unwrap());

    let mode = {
        if min_x == max_x {
            SideMode::X { x: min_x, min_y, min_z, max_y, max_z }
        } else if min_y == max_y {
            SideMode::Y { y: min_y, min_x, min_z, max_x, max_z }
        } else if min_z == max_z {
            SideMode::Z { z: min_z, min_x, min_y, max_x, max_y }
        } else {
            unreachable!()
        }
    };
    Side {
        points,
        mode,
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

    //test();
}

fn test() {
    let side = build_side("4  10 10 10  10 10 20  10 20 20  10 20 10");
    let r = get_point_side_relation(&Point { x: 10, y: 15, z: 15 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 10, y: 10, z: 15 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 10, y: 20, z: 15 }, &side);
    assert_eq!(PointSideRelation::Off, r);

    let r = get_point_side_relation(&Point { x: 11, y: 15, z: 15 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 9, y: 15, z: 15 }, &side);
    assert_eq!(PointSideRelation::Negative, r);


    let side = build_side("4  10 10 10  10 10 20  20 10 20  20 10 10");
    let r = get_point_side_relation(&Point { x: 15, y: 10, z: 15 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 10, y: 10, z: 15 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 20, y: 10, z: 15 }, &side);
    assert_eq!(PointSideRelation::Off, r);

    let r = get_point_side_relation(&Point { x: 15, y: 11, z: 15 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 15, y: 9, z: 15 }, &side);
    assert_eq!(PointSideRelation::Negative, r);


    let side = build_side("4  10 10 10  10 20 10  20 20 10  20 10 10");
    let r = get_point_side_relation(&Point { x: 15, y: 15, z: 10 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 10, y: 15, z: 10 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 20, y: 15, z: 10 }, &side);
    assert_eq!(PointSideRelation::Off, r);

    let r = get_point_side_relation(&Point { x: 15, y: 15, z: 11 }, &side);
    assert_eq!(PointSideRelation::Positive, r);

    let r = get_point_side_relation(&Point { x: 15, y: 15, z: 9 }, &side);
    assert_eq!(PointSideRelation::Negative, r);


    let bulk = build_bulk(vec![
        build_side("4  10 10 10  10 10 20  10 20 20  10 20 10"),
        build_side("4  20 10 10  20 10 20  20 20 20  20 20 10"),
        build_side("4  10 10 10  10 10 20  20 10 20  20 10 10"),
        build_side("4  10 20 10  10 20 20  20 20 20  20 20 10"),
        build_side("4  10 10 10  10 20 10  20 20 10  20 10 10"),
        build_side("5  10 10 20  10 20 20  20 20 20  20 15 20  20 10 20"),
        build_side("4  14 14 14  14 14 16  14 16 16  14 16 14"),
        build_side("4  16 14 14  16 14 16  16 16 16  16 16 14"),
        build_side("4  14 14 14  14 14 16  16 14 16  16 14 14"),
        build_side("4  14 16 14  14 16 16  16 16 16  16 16 14"),
        build_side("4  14 14 14  14 16 14  16 16 14  16 14 14"),
        build_side("4  14 14 16  14 16 16  16 16 16  16 14 16"),
    ]);

    let in_bulk = is_in_bulk(&Point { x: 11, y: 11, z: 11 }, &bulk);
    assert_eq!(true, in_bulk);

    let in_bulk = is_in_bulk(&Point { x: 10, y: 10, z: 10 }, &bulk);
    assert_eq!(true, in_bulk);

    let in_bulk = is_in_bulk(&Point { x: 20, y: 20, z: 20 }, &bulk);
    assert_eq!(false, in_bulk);

    let in_bulk = is_in_bulk(&Point { x: 15, y: 15, z: 15 }, &bulk);
    assert_eq!(false, in_bulk);
}