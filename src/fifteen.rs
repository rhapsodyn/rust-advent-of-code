use regex::Regex;

use crate::common::read_lines;

#[allow(dead_code)]
pub(crate) fn ans() {
    let lines = read_lines("15").unwrap();
    let a = ans_p1(&lines, 2000000);
    println!("{}", a);
}

#[allow(dead_code)]
pub(crate) fn ans2() {
    let lines = read_lines("15").unwrap();
    let pos = ans_p2(
        &lines,
        &Range {
            start: 0,
            end: 400_0000,
        },
    );
    dbg!(&pos);
    let x = pos.x as u64 * 4000000 + pos.y as u64;
    println!("x: {}", x);
}

fn ans_p1(lines: &Vec<String>, y: i32) -> i32 {
    let clues = parse(lines);
    let (occupied_xs, final_ranges) = get_impossible_ranges(&clues, y);
    let mut a = 0;
    let mut beacon_count = 0;
    for r in final_ranges {
        let confirmed = occupied_xs
            .iter()
            .filter(|&&x| x >= r.start && x <= r.end)
            .count();
        beacon_count += confirmed;

        a += r.end - r.start + 1;
    }

    a - beacon_count as i32
}

fn get_impossible_ranges(clues: &Vec<Clue>, y: i32) -> (Vec<i32>, Vec<Range>) {
    let mut ranges = vec![];
    let mut comfirmed_beacon_xs = vec![];
    for c in clues {
        if c.beacon.y == y {
            let x = c.beacon.x;
            if !comfirmed_beacon_xs.contains(&x) {
                comfirmed_beacon_xs.push(x);
            }
        }
        if c.sensor.y == y {
            let x = c.sensor.x;
            if !comfirmed_beacon_xs.contains(&x) {
                comfirmed_beacon_xs.push(x);
            }
        }
        if let Some(r) = c.get_x_projection(y) {
            // all by fold ??
            ranges = union(ranges, &r);
        }
    }
    let final_ranges: Vec<Range> = ranges.iter().fold(vec![], union);
    // dbg!(&final_ranges);
    // dbg!(&occupied_xs);
    // println!();
    (comfirmed_beacon_xs, final_ranges)
}

// fn min(a: i32, b: i32, c: i32) -> i32 {
//     a.min(b.min(c))
// }

// fn max(a: i32, b: i32, c: i32) -> i32 {
//     a.max(b.max(c))
// }

fn manhattan_distance(p1: &Pos, p2: &Pos) -> u32 {
    p1.x.abs_diff(p2.x) + p1.y.abs_diff(p2.y)
}

#[test]
fn test_manhattan_distance() {
    let d = manhattan_distance(&Pos { x: 8, y: 7 }, &Pos { x: 2, y: 10 });
    assert_eq!(d, 9);
}

fn parse(lines: &Vec<String>) -> Vec<Clue> {
    let mut clues = vec![];

    for l in lines {
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        let mut sp = l.split(" ");
        assert_eq!(sp.next().unwrap(), String::from("Sensor"));
        assert_eq!(sp.next().unwrap(), String::from("at"));
        let mut xequal = sp.next().unwrap();
        let mut x = get_digit_out(xequal);
        let mut yequal = sp.next().unwrap();
        let mut y = get_digit_out(yequal);
        let sensor = Pos { x, y };
        assert_eq!(sp.next().unwrap(), String::from("closest"));
        assert_eq!(sp.next().unwrap(), String::from("beacon"));
        assert_eq!(sp.next().unwrap(), String::from("is"));
        assert_eq!(sp.next().unwrap(), String::from("at"));
        xequal = sp.next().unwrap();
        x = get_digit_out(xequal);
        yequal = sp.next().unwrap();
        y = get_digit_out(yequal);
        let beacon = Pos { x, y };
        clues.push(Clue {
            radius: manhattan_distance(&sensor, &beacon),
            sensor,
            beacon,
        });
    }

    // dbg!(&clues);
    clues
}

fn get_digit_out(s: &str) -> i32 {
    // lazy-static spell!
    let reg = Regex::new(r"[xy]=(-?\d+)").unwrap();
    let d = &reg.captures_iter(s).next().unwrap()[1];
    d.parse().unwrap()
}

#[derive(Debug, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Clue {
    sensor: Pos,
    beacon: Pos,
    radius: u32,
}

impl Clue {
    // fn could_not_be_a_beacon(&self, p: &Pos) -> bool {
    //     if p == &self.beacon {
    //         return false;
    //     }

    //     let p_dis = manhattan_distance(&self.sensor, p);
    //     let self_dis = manhattan_distance(&self.sensor, &self.beacon);

    //     println!(
    //         "self:{:?} p:{:?} p_dis:{:?} self_dis:{:?}",
    //         self, p, p_dis, self_dis
    //     );

    //     p_dis <= self_dis
    // }

    fn get_x_projection(&self, y: i32) -> Option<Range> {
        // let dis = manhattan_distance(&self.sensor, &self.beacon);
        let dis = self.radius;
        let x_delta = dis as i32 - self.sensor.y.abs_diff(y) as i32;
        if x_delta <= 0 {
            // too far
            None
        } else {
            let x_mid = self.sensor.x;
            Some(Range {
                start: x_mid - x_delta,
                end: x_mid + x_delta,
            })
        }
    }
}

///
/// the `std` one not so useful in this case
///
#[derive(Debug, Clone, PartialEq)]
struct Range {
    start: i32,
    end: i32,
}

fn union(ranges: Vec<Range>, range: &Range) -> Vec<Range> {
    if ranges.len() == 0 {
        return vec![range.clone()];
    }

    let mut new_ranges = ranges.clone();
    let mut need_to_expand = true;
    for r in new_ranges.iter_mut() {
        if r.start <= range.start && r.end >= range.end {
            // totally cover
            need_to_expand = false;
            break;
        } else if !(range.end < r.start || range.start > r.end) {
            // partially cover
            r.start = i32::min(r.start, range.start);
            r.end = i32::max(r.end, range.end);
            need_to_expand = false;
            break;
        }
    }

    if need_to_expand {
        new_ranges.push(range.clone());
    }

    new_ranges
}

#[test]
fn test_ans1() {
    let lines = read_lines("15-test").unwrap();
    let mut a = ans_p1(&lines, 9);
    assert_eq!(a, 25);
    a = ans_p1(&lines, 10);
    assert_eq!(a, 26);
    a = ans_p1(&lines, 11);
    assert_eq!(a, 27);
}

fn ans_p2(lines: &Vec<String>, bound: &Range) -> Pos {
    let clues = parse(lines);
    // let (occupied_xs, final_ranges) = get_impossible_ranges(clues, y);
    // todo!()

    for y in bound.start..bound.end + 1 {
        let (_, row) = get_impossible_ranges(&clues, y);
        if row.len() > 1 {
            for r in row {
                let x = r.end + 1;
                if x >= bound.start && x <= bound.end {
                    return Pos { x, y };
                }
            }
        }
    }

    panic!("not found");
}

#[test]
fn test_ans2() {
    let lines = read_lines("15-test").unwrap();
    let p = ans_p2(&lines, &Range { start: 0, end: 20 });
    assert_eq!(p, Pos { x: 14, y: 11 });
}
