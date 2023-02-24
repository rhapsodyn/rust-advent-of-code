use std::{cmp::Ordering, vec};

use anyhow::{Ok, Result};

use crate::common::read_lines;

type Pairs = Vec<Pair>;
type Pair = (List, List);
type List = Vec<Item>;
#[derive(Debug, PartialEq, Clone)]
enum Item {
    Single(u8),
    Multi(List),
}

#[allow(dead_code)]
pub fn ans() -> Result<()> {
    let lines = read_lines("13").unwrap();
    let pairs = parse(&lines);
    let a = _ans(&pairs);
    let total: usize = a.iter().sum();

    println!("{:?}, {}", &a, total);

    Ok(())
}

fn parse(lines: &Vec<String>) -> Pairs {
    let mut pairs = vec![];
    let mut temp_pair = (vec![], vec![]);
    let mut is_left = true;
    for l in lines {
        if l == "" {
            pairs.push(temp_pair);
            temp_pair = (vec![], vec![]);
            is_left = true;
        } else {
            match is_left {
                true => {
                    temp_pair.0 = parse_line(l);
                    is_left = false;
                }
                false => temp_pair.1 = parse_line(l),
            }
        }
    }

    pairs.push(temp_pair);

    pairs
}

fn parse_line(line: &String) -> List {
    let mut stack: Vec<List> = vec![];
    let mut number_start = 0;
    for i in 0..line.len() {
        let ch = line.chars().nth(i).unwrap();
        match ch {
            '[' => {
                stack.push(vec![]);
                number_start = i + 1;
            }
            ']' => {
                if number_start != i {
                    let num: u8 = line[number_start..i].parse().unwrap();
                    stack.last_mut().unwrap().push(Item::Single(num));
                }

                if i != line.len() - 1 {
                    let last = stack.pop().unwrap();
                    number_start = i + 1;
                    assert!(stack.len() > 0);
                    stack.last_mut().unwrap().push(Item::Multi(last));
                }
            }
            ',' => {
                if number_start != i {
                    let num: u8 = match line[number_start..i].parse() {
                        core::result::Result::Ok(n) => n,
                        Err(_) => {
                            panic!("{}:{}-{}", line, number_start, i);
                        }
                    };
                    stack.last_mut().unwrap().push(Item::Single(num));
                }
                number_start = i + 1;
            }
            _ => {
                // number cont
            }
        }
    }

    // assert_eq!(stack.len(), 0);

    stack.pop().unwrap()
}

fn _ans(pairs: &Pairs) -> Vec<usize> {
    let mut result = vec![];
    for (i, p) in pairs.iter().enumerate() {
        if order_right(&p.0, &p.1) != Ordering::Greater {
            result.push(i + 1);
        }
    }

    result
}

// #[derive(Debug, PartialEq)]
// enum Order {
//     Right,
//     Wrong,
//     Equal,
// }

fn order_right(left: &List, right: &List) -> Ordering {
    if left.len() == 0 && right.len() == 0 {
        return Ordering::Equal;
    }

    if left.len() == 0 {
        return Ordering::Less;
    }

    if right.len() == 0 {
        return Ordering::Greater;
    }

    let mut l = left.clone().into_iter();
    let mut r = right.clone().into_iter();

    let l_first = l.next().unwrap();
    let r_first = r.next().unwrap();

    let order = match (l_first, r_first) {
        (Item::Single(l_num), Item::Single(r_num)) => {
            if l_num < r_num {
                Ordering::Less
            } else if l_num > r_num {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        (left, right) => {
            let l = match left {
                Item::Single(num) => vec![Item::Single(num)],
                Item::Multi(l) => l,
            };
            let r = match right {
                Item::Single(num) => vec![Item::Single(num)],
                Item::Multi(r) => r,
            };

            order_right(&l, &r)
        }
    };

    if order == Ordering::Equal {
        order_right(&l.collect(), &r.collect())
    } else {
        order
    }
}

#[test]
fn test_ans_13() {
    let lines = read_lines("13-test").unwrap();
    let pairs = parse(&lines);
    let a = _ans(&pairs);
    assert_eq!(a, vec![1, 2, 4, 6]);
}

#[allow(dead_code)]
pub(crate) fn ans2() -> Result<()> {
    let lines = read_lines("13").unwrap();
	let idx = _ans2(&lines);
	println!("{}", idx[0] * idx[1]);
    Ok(())
}

fn _ans2(lines: &Vec<String>) -> [usize; 2] {
    let mut idxes = [0usize; 2];
    let pairs = parse(&lines);

    let mut all = vec![];
    for orig in pairs {
        all.push(orig.0);
        all.push(orig.1);
    }
    let d1 = vec![Item::Multi(vec![Item::Single(2)])];
    all.push(d1.clone());
    let d2 = vec![Item::Multi(vec![Item::Single(6)])];
    all.push(d2.clone());
    all.sort_by(order_right);

	for (i, item) in all.iter().enumerate() {
		if item == &d1 {
			idxes[0] = i + 1;
		}
		if item == &d2 {
			idxes[1] = i + 1;
		}
	}

    idxes
}

#[test]
fn test_ans2_13() {
    let lines = read_lines("13-test").unwrap();
	let idx = _ans2(&lines);
	assert_eq!(idx[0], 10);
	assert_eq!(idx[1], 14);
}
