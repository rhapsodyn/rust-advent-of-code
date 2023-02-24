use std::{collections::HashSet, fmt::Display, vec};

use anyhow::Result;
use num_bigint::BigUint;

use crate::common::read_lines;

#[allow(dead_code)]
pub(crate) fn day11() -> Result<()> {
    let lines = read_lines("11")?;
    let monkeys = inspect(&lines, 20);

    let ans = get_d11_ans(&monkeys);
    println!("asn: {}", ans);

    Ok(())
}

fn get_d11_ans(monkeys: &Vec<Monkey>) -> u128 {
    let mut inspects: Vec<u128> = monkeys.iter().map(|m| m.inspect_times).collect();
    inspects.sort();
    inspects[inspects.len() - 1] * inspects[inspects.len() - 2]
}

type D11Op = Box<dyn Fn(&BigUint) -> BigUint>;

struct Monkey {
    items: Vec<BigUint>,
    op: D11Op,
    div_by: BigUint,
    inspect_times: u128,
    true_monkey: usize,
    false_monkey: usize,
}

impl Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "items: {:?} div: {} t: {} f: {}",
            self.items, self.div_by, self.true_monkey, self.false_monkey
        )
    }
}

fn parse_d11(lines: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let mut liter = lines.iter();

    loop {
        // id
        let mut sp = liter.next().unwrap().split(" ");
        assert_eq!(sp.next().unwrap(), "Monkey");
        let _id: u8 = sp.next().unwrap()[0..1].parse().unwrap();
        // id by index
        // println!("{}", id);

        // items
        sp = liter.next().unwrap().trim().split(" ");
        assert_eq!(sp.next().unwrap(), "Starting");
        assert_eq!(sp.next().unwrap(), "items:");
        let mut items = vec![];
        while let Some(digit) = sp.next() {
            let ds = if digit.ends_with(',') {
                String::from(digit)[0..digit.len() - 1].to_string()
            } else {
                String::from(digit)
            };
            items.push(ds.parse().unwrap());
        }
        // println!("{:?}", items);

        // op
        sp = liter.next().unwrap().trim().split(" ");
        assert_eq!(sp.next().unwrap(), "Operation:");
        assert_eq!(sp.next().unwrap(), "new");
        assert_eq!(sp.next().unwrap(), "=");
        assert_eq!(sp.next().unwrap(), "old");
        let math_op = sp.next().unwrap();
        let p2 = sp.next().unwrap();
        let op: D11Op = if p2 == "old" {
            match math_op {
                "+" => Box::new(|p| p + p),
                "-" => Box::new(|p| p - p),
                "*" => Box::new(|p| p * p),
                "/" => Box::new(|p| p / p),
                _ => panic!(),
            }
        } else {
            let cons = BigUint::from(p2.parse::<u32>().unwrap());
            match math_op {
                "+" => Box::new(move |p| p + &cons),
                "-" => Box::new(move |p| p - &cons),
                "*" => Box::new(move |p| p * &cons),
                "/" => Box::new(move |p| p / &cons),
                _ => panic!(),
            }
        };

        // test
        sp = liter.next().unwrap().trim().split(" ");
        assert_eq!(sp.next().unwrap(), "Test:");
        assert_eq!(sp.next().unwrap(), "divisible");
        assert_eq!(sp.next().unwrap(), "by");
        let div_by = sp.next().unwrap().parse().unwrap();

        // true
        sp = liter.next().unwrap().trim().split(" ");
        assert_eq!(sp.next().unwrap(), "If");
        assert_eq!(sp.next().unwrap(), "true:");
        assert_eq!(sp.next().unwrap(), "throw");
        assert_eq!(sp.next().unwrap(), "to");
        assert_eq!(sp.next().unwrap(), "monkey");
        let true_monkey: usize = sp.next().unwrap().parse().unwrap();

        // false
        sp = liter.next().unwrap().trim().split(" ");
        assert_eq!(sp.next().unwrap(), "If");
        assert_eq!(sp.next().unwrap(), "false:");
        assert_eq!(sp.next().unwrap(), "throw");
        assert_eq!(sp.next().unwrap(), "to");
        assert_eq!(sp.next().unwrap(), "monkey");
        let false_monkey: usize = sp.next().unwrap().parse().unwrap();

        let m = Monkey {
            items,
            op,
            div_by,
            true_monkey,
            false_monkey,
            inspect_times: 0,
        };
        // println!("{}", &m);
        monkeys.push(m);

        if liter.next() == None {
            break;
        }
    }

    monkeys
}

fn inspect(lines: &Vec<String>, round: usize) -> Vec<Monkey> {
    let mut monkeys = parse_d11(lines);

    for _ in 0..round {
        for i in 0..monkeys.len() {
            let mut ic = 0;
            while let Some(item) = monkeys[i].items.pop() {
                let new_val = (monkeys[i].op)(&item) / BigUint::from(3u32);
                let div = &monkeys[i].div_by;
                let to = if &new_val % div == BigUint::from(0u32) {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };
                monkeys[to].items.push(new_val);
                ic += 1;
            }
            monkeys[i].inspect_times += ic;
        }
    }

    monkeys
}

fn inspect2(lines: &Vec<String>, round: usize) -> Vec<Monkey> {
    let mut monkeys = parse_d11(lines);

    let magic: BigUint = monkeys.iter().map(|m| &m.div_by).product();

    for _ in 0..round {
        for i in 0..monkeys.len() {
            let mut ic = 0;
            while let Some(mut item) = monkeys[i].items.pop() {
                item %= &magic;
                let new_val = (monkeys[i].op)(&item);
                let div = &monkeys[i].div_by;
                let to = if &new_val % div == BigUint::from(0u32) {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };
                monkeys[to].items.push(new_val);
                ic += 1;
            }
            monkeys[i].inspect_times += ic;
        }
    }

    monkeys
}

#[test]
fn test_inspect() {
    let lines = read_lines("11-test").unwrap();
    let monkeys = inspect(&lines, 20);

    assert!(array_loose_equal(
        &monkeys[0].items,
        &[10u32, 12u32, 14u32, 26u32, 34u32]
            .map(|n| BigUint::from(n))
            .to_vec()
    ));
    assert!(array_loose_equal(
        &monkeys[1].items,
        &[245u32, 93u32, 53u32, 199u32, 115u32]
            .map(|n| BigUint::from(n))
            .to_vec()
    ));

    assert_eq!(monkeys[2].items, []);
    assert_eq!(monkeys[3].items, []);

    assert_eq!(monkeys[0].inspect_times, 101);
    assert_eq!(monkeys[1].inspect_times, 95);
    assert_eq!(monkeys[2].inspect_times, 7);
    assert_eq!(monkeys[3].inspect_times, 105);
}

#[allow(dead_code)]
fn array_loose_equal<T: PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    v1.len() == v2.len() && v1.iter().all(|x| v2.contains(x))
}

#[test]
fn test_inspect2() {
    let lines = read_lines("11-test").unwrap();
    let monkeys = inspect2(&lines, 10000);
    let ans = get_d11_ans(&monkeys);
    assert_eq!(ans, 2713310158);
}

#[allow(dead_code)]
pub(crate) fn day11p2() -> Result<()> {
    let lines = read_lines("11")?;
    let monkeys = inspect2(&lines, 10000);

    let ans = get_d11_ans(&monkeys);
    println!("asn: {}", ans);

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn day12() -> Result<()> {
    let lines = read_lines("12")?;
    let ans = ans_d12(&lines);
    println!("{}", ans);
    Ok(())
}

type D12Pos = (usize, usize);

fn ans_d12(lines: &Vec<String>) -> usize {
    // parse
    let mut start: Option<D12Pos> = None;
    let mut end: Option<D12Pos> = None;
    let xl = lines[0].len();
    let yl = lines.len();
    let mut grid: Vec<Vec<i8>> = vec![];
    for y in 0..yl {
        let l = &lines[y];
        grid.push(vec![]);
        for x in 0..xl {
            let ch = l.chars().nth(x).unwrap();
            match ch {
                'S' => {
                    start = Some((x, y));
                    let mut start_height = [0];
                    'a'.encode_utf8(&mut start_height);
                    // a to start
                    grid[y].push(start_height[0] as i8);
                }
                'E' => {
                    end = Some((x, y));
                    let mut end_height = [0];
                    'z'.encode_utf8(&mut end_height);
                    // z to end
                    grid[y].push(end_height[0] as i8);
                }
                a => {
                    let mut dst = [0; 1];
                    a.encode_utf8(&mut dst);
                    grid[y].push(dst[0] as i8);
                }
            }
        }
    }
    // dbg!(&start);
    // dbg!(&end);
    // dbg!(&grid);

    // traverse
    let mut curr = HashSet::new();
    curr.insert(start.unwrap());
    let mut passed = HashSet::new();
    d12_step_furthur(curr, &mut passed, &grid, &end.unwrap(), 0)
}

fn d12_step_furthur(
    curr: HashSet<D12Pos>,
    passed: &mut HashSet<D12Pos>,
    grid: &Vec<Vec<i8>>,
    end: &D12Pos,
    depth: usize,
) -> usize {
    let mut next = HashSet::<D12Pos>::new();
    for cur in curr.iter() {
        let possible = find_possible(cur, grid);
        for p in possible {
            if &p == end {
                return depth + 1;
            }

            if !passed.contains(&p) {
                passed.insert(p);
                next.insert(p);
            }
        }
    }

    // for n in next.iter() {
    //     print!("{:?}", n);
    //     print!(":{:?}, ", grid[n.1][n.0]);
    // }
    // println!();

    d12_step_furthur(next, passed, grid, end, depth + 1)
}

fn find_possible(cur: &D12Pos, grid: &Vec<Vec<i8>>) -> Vec<D12Pos> {
    let xl = grid[0].len();
    let yl = grid.len();
    let mut four_sides = vec![];
    if cur.0 > 0 {
        // left
        four_sides.push((cur.0 - 1, cur.1));
    }
    if cur.0 < xl - 1 {
        // right
        four_sides.push((cur.0 + 1, cur.1));
    }
    if cur.1 > 0 {
        // up
        four_sides.push((cur.0, cur.1 - 1));
    }
    if cur.1 < yl - 1 {
        // down
        four_sides.push((cur.0, cur.1 + 1));
    }
    let possible_moves = four_sides
        .iter()
        .filter(|(x, y)| {
            grid[*y][*x] - grid[cur.1][cur.0] <= 1
        })
        .map(|p| p.clone())
        .collect::<Vec<D12Pos>>();
    possible_moves
}

#[test]
fn test_ans_d12() {
    let lines: Vec<String> = r"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
        .split("\n")
        .filter(|l| l.len() > 0)
        .map(|s| s.to_string())
        .collect();
    let a = ans_d12(&lines);
    assert_eq!(a, 31);
}

#[allow(dead_code)]
pub(crate) fn day12p2() -> Result<()> {
    let lines = read_lines("12")?;
    let ans = ans_d12_p2(&lines);
    println!("{}", ans);
    Ok(())
}

fn ans_d12_p2(lines: &Vec<String>) -> usize {
    // parse
    let mut curr = HashSet::new();
    let mut end: Option<D12Pos> = None;
    let xl = lines[0].len();
    let yl = lines.len();
    let mut grid: Vec<Vec<i8>> = vec![];
    for y in 0..yl {
        let l = &lines[y];
        grid.push(vec![]);
        for x in 0..xl {
            let ch = l.chars().nth(x).unwrap();
            match ch {
                'S' => {
                    // start = Some((x, y));
                    curr.insert((x,y));
                    let mut start_height = [0];
                    'a'.encode_utf8(&mut start_height);
                    // a to start
                    grid[y].push(start_height[0] as i8);
                }
                'E' => {
                    end = Some((x, y));
                    let mut end_height = [0];
                    'z'.encode_utf8(&mut end_height);
                    // z to end
                    grid[y].push(end_height[0] as i8);
                }
                a => {
                    let mut dst = [0; 1];
                    a.encode_utf8(&mut dst);
                    grid[y].push(dst[0] as i8);

                    if a == 'a' {
                        curr.insert((x, y));
                    }
                }
            }
        }
    }
    // dbg!(&start);
    // dbg!(&end);
    // dbg!(&grid);

    // traverse
    let mut passed = HashSet::new();
    d12_step_furthur(curr, &mut passed, &grid, &end.unwrap(), 0)
}