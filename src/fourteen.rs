use std::collections::HashSet;

use anyhow::{Ok, Result};

use crate::common::read_lines;

type SandMap = Vec<Vec<bool>>;
type Coord = (usize, usize);

#[allow(dead_code)]
pub(crate) fn ans() -> Result<()> {
    let lines = read_lines("14")?;
    let a = _ans(&lines);
    println!("{}", a);
    Ok(())
}

#[allow(dead_code)]
pub(crate) fn ans2() -> Result<()> {
    let lines = read_lines("14")?;
    let a = _ans2(&lines);
    println!("{}", a);
    Ok(())
}

fn _ans2(lines: &Vec<String>) -> usize {
    let expand = 200;
    let (mut map, x_base) = parse_rocks(lines, expand);
    let xl = map[0].len();
    map.push(vec![false; xl]);
    map.push(vec![true; xl]);

    let mut unit = 0;
    let start = (500 - x_base + expand, 0usize);

    while !map[start.1][start.0] {
        fall(&mut map, &start);
        unit += 1;
    }

    unit
}

fn fall(map: &mut SandMap, coord: &Coord) {
    let down = (coord.0, coord.1 + 1);
    let down_left = (coord.0 - 1, coord.1 + 1);
    let down_right = (coord.0 + 1, coord.1 + 1);
    if !map[down.1][down.0] {
        fall(map, &down);
    } else if !map[down_left.1][down_left.0] {
        fall(map, &down_left);
    } else if !map[down_right.1][down_right.0] {
        fall(map, &down_right);
    } else {
        map[coord.1][coord.0] = true;
    }
}

fn _ans(lines: &Vec<String>) -> usize {
    let (mut map, x_base) = parse_rocks(lines, 0);
    let mut unit = 0;
    let start = (500 - x_base, 0usize);

    while fallable(&mut map, &start) {
        unit += 1;
    }

    unit
}

fn fallable(map: &mut SandMap, coord: &Coord) -> bool {
    // out of bound
    if coord.1 >= map.len() - 1 || coord.0 <= 0 || coord.0 >= map[0].len() - 1 {
        return false;
    }

    let down = (coord.0, coord.1 + 1);
    let down_left = (coord.0 - 1, coord.1 + 1);
    let down_right = (coord.0 + 1, coord.1 + 1);
    if !map[down.1][down.0] {
        fallable(map, &down)
    } else if !map[down_left.1][down_left.0] {
        fallable(map, &down_left)
    } else if !map[down_right.1][down_right.0] {
        fallable(map, &down_right)
    } else {
        map[coord.1][coord.0] = true;
        true
    }
}

fn new_map(xl: usize, yl: usize) -> SandMap {
    let mut sm = vec![];
    for _ in 0..yl {
        sm.push(vec![false; xl]);
    }
    sm
}

fn parse_rocks(lines: &Vec<String>, expand: usize) -> (SandMap, usize) {
    let mut rocks: Vec<HashSet<(usize, usize)>> = vec![];
    for l in lines {
        let mut sp = l.split("->");
        let mut rock = HashSet::new();
        let mut first_seg = sp.next().unwrap().trim().split(",");
        let x: usize = first_seg.next().unwrap().parse().unwrap();
        let y: usize = first_seg.next().unwrap().parse().unwrap();
        let mut prev_coord = (x, y);
        for seg in sp {
            let mut xy = seg.trim().split(",");
            let x: usize = xy.next().unwrap().parse().unwrap();
            let y: usize = xy.next().unwrap().parse().unwrap();
            let curr_coord = (x, y);
            if prev_coord.0 == curr_coord.0 {
                let y_base = if curr_coord.1 > prev_coord.1 {
                    prev_coord.1
                } else {
                    curr_coord.1
                };
                let x = curr_coord.0;
                for y_delta in 0..(curr_coord.1.abs_diff(prev_coord.1) + 1) {
                    rock.insert((x, y_base + y_delta));
                }
            } else {
                let x_base = if curr_coord.0 > prev_coord.0 {
                    prev_coord.0
                } else {
                    curr_coord.0
                };
                let y = curr_coord.1;
                for x_delta in 0..(curr_coord.0.abs_diff(prev_coord.0) + 1) {
                    rock.insert((x_base + x_delta, y));
                }
            }
            prev_coord = curr_coord;
        }
        rocks.push(rock);
    }

    let mut x_base = usize::MAX;
    let mut xl = 0;
    let mut yl = 0;
    for r in rocks.iter() {
        for (x, y) in r {
            if x < &x_base {
                x_base = *x;
            }

            if x > &xl {
                xl = *x;
            }

            if y > &yl {
                yl = *y;
            }
        }
    }

    let mut map = new_map(xl - x_base + 1 + 2 * expand, yl + 1);
    for r in rocks {
        for (x, y) in r {
            map[y][x - x_base + expand] = true;
        }
    }

    // draw to test
    // print_rocks(&map);

    (map, x_base)
}

#[allow(dead_code)]
fn print_rocks(map: &SandMap) {
    for xs in map.iter() {
        for x in xs {
            if *x {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[test]
fn test_ans() {
    let lines = vec![
        "498,4 -> 498,6 -> 496,6".to_string(),
        "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
    ];
    let a = _ans(&lines);
    assert_eq!(a, 24);
}

#[test]
fn test_ans2() {
    let lines = vec![
        "498,4 -> 498,6 -> 496,6".to_string(),
        "503,4 -> 502,4 -> 502,9 -> 494,9".to_string(),
    ];
    let a = _ans2(&lines);
    assert_eq!(a, 93);
}
