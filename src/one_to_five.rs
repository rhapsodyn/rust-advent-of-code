use std::{collections::HashSet, fs::read_to_string};

use anyhow::{anyhow, Result};

use crate::common::read_lines;

#[allow(dead_code)]
pub fn day1() -> Result<()> {
    let lines = read_lines("1")?;

    let mut biggest = 0u64;
    let mut temp = 0u64;
    for l in lines {
        match l.as_str() {
            "" => {
                if temp > biggest {
                    biggest = temp
                }
                temp = 0;
            }
            _ => {
                let num = l.parse::<u64>()?;
                temp += num;
            }
        }
    }

    println!("{}", biggest);

    Ok(())
}

#[allow(dead_code)]
pub fn day1e2() -> Result<()> {
    let content = read_to_string("input/d1.txt")?;
    let lines = content.split("\n");

    let mut biggest = vec![0u64, 0u64, 0u64];
    let mut temp = 0u64;
    for l in lines {
        match l {
            "" => {
                for i in biggest.as_mut_slice() {
                    if temp > *i {
                        *i = temp;
                        break;
                    }
                }
                temp = 0;
            }
            _ => {
                let num = l.parse::<u64>()?;
                temp += num;
            }
        }
    }

    println!("{}", biggest.iter().fold(0, |prev, curr| prev + curr));

    Ok(())
}

#[allow(dead_code)]
pub fn day2() -> Result<()> {
    let content = read_to_string("input/d2.txt")?;
    let lines = content.split("\n");

    let mut total = 0;
    for l in lines {
        total += one_round(l)?;
    }

    println!("{}", total);

    Ok(())
}

fn one_round(l: &str) -> Result<u64> {
    let mut cols = l.split(" ");
    let left = cols.next().unwrap();
    let right = cols.next().unwrap();

    let round = match right {
        // rock
        "X" => match left {
            "A" => 1 + 3,
            "B" => 1 + 0,
            "C" => 1 + 6,
            _ => return Err(anyhow!("unexpected input")),
        },
        // paper
        "Y" => match left {
            "A" => 2 + 6,
            "B" => 2 + 3,
            "C" => 2 + 0,
            _ => return Err(anyhow!("unexpected input")),
        },
        // scissor
        "Z" => match left {
            "A" => 3 + 0,
            "B" => 3 + 6,
            "C" => 3 + 3,
            _ => return Err(anyhow!("unexpected input")),
        },
        _ => return Err(anyhow!("unexpected input")),
    };

    Ok(round)
}

#[test]
fn test_one_round() {
    assert_eq!(one_round("A Y").unwrap(), 8);
    assert_eq!(one_round("B X").unwrap(), 1);
    assert_eq!(one_round("C Z").unwrap(), 6);
}

#[allow(dead_code)]
pub fn day2p2() -> Result<()> {
    let content = read_to_string("input/d2.txt")?;
    let lines = content.split("\n");

    let mut total = 0;
    for l in lines {
        total += one_round_p2(l)?;
    }

    println!("{}", total);

    Ok(())
}

fn one_round_p2(l: &str) -> Result<u64> {
    let mut cols = l.split(" ");
    let left = cols.next().unwrap();
    let right = cols.next().unwrap();

    let score = match left {
        // rock
        "A" => match right {
            "X" => 0 + 3,
            "Y" => 3 + 1,
            "Z" => 6 + 2,
            _ => return Err(anyhow!("unexpected input")),
        },
        // paper
        "B" => match right {
            "X" => 0 + 1,
            "Y" => 3 + 2,
            "Z" => 6 + 3,
            _ => return Err(anyhow!("unexpected input")),
        },
        // scissor
        "C" => match right {
            "X" => 0 + 2,
            "Y" => 3 + 3,
            "Z" => 6 + 1,
            _ => return Err(anyhow!("unexpected input")),
        },
        _ => return Err(anyhow!("unexpected input")),
    };

    Ok(score)
}

#[test]
fn test_one_round_p2() {
    assert_eq!(one_round_p2("A Y").unwrap(), 4);
    assert_eq!(one_round_p2("B X").unwrap(), 1);
    assert_eq!(one_round_p2("C Z").unwrap(), 7);
}

#[allow(dead_code)]
pub fn day3() -> Result<()> {
    let lines = read_lines("3")?;
    let mut total = 0;
    for l in lines {
        let (left, right) = l.split_at(l.len() / 2);
        let mut uniq = HashSet::<char>::new();
        for c in left.chars() {
            uniq.insert(c);
        }

        for c in right.chars() {
            if uniq.contains(&c) {
                total += get_priority(c)?;
                break;
            }
        }
    }

    println!("{}", total);

    Ok(())
}

/// a => 1
/// A => 27
fn get_priority(ch: char) -> Result<u32> {
    let pri = ch as u32;
    // println!("{} {}", ch, pri);
    if pri > 96 {
        Ok(pri - 96)
    } else {
        Ok(pri - 64 + 26)
    }
}

#[test]
fn test_get_priority() {
    assert_eq!(get_priority('A').unwrap(), 27);
    assert_eq!(get_priority('Z').unwrap(), 52);
    assert_eq!(get_priority('a').unwrap(), 1);
    assert_eq!(get_priority('z').unwrap(), 26);
}

#[allow(dead_code)]
pub fn day3p2() -> Result<()> {
    let lines = read_lines("3")?;
    let mut total = 0;

    for i in 0..lines.len() {
        if (i + 1) % 3 == 0 {
            total += sum_three(&lines[i - 2], &lines[i - 1], &lines[i])?;
        }
    }

    println!("{}", total);

    Ok(())
}

fn sum_three(s1: &str, s2: &str, s3: &str) -> Result<u32> {
    let mut total = 0;
    let l1: Vec<char> = s1.chars().collect();
    let l2: Vec<char> = s2.chars().collect();
    for c in s3.chars() {
        if l1.contains(&c) && l2.contains(&c) {
            total += get_priority(c)?;
            break;
        }
    }

    Ok(total)
}

#[test]
fn test_sum_three() {
    assert_eq!(
        sum_three(
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg"
        )
        .unwrap(),
        18
    );
    assert_eq!(
        sum_three(
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        )
        .unwrap(),
        52
    );
}

#[allow(dead_code)]
pub fn day4() -> Result<()> {
    let lines = read_lines("4")?;
    let mut total = 0;
    for l in lines {
        let mut pairs = l.split(",");
        let left = pairs.next().expect("fuck no left");
        let right = pairs.next().expect("fuck no right");

        let left_sections = get_sections(left);
        let right_sections = get_sections(right);

        let left_bigger =
            left_sections.0 <= right_sections.0 && left_sections.1 >= right_sections.1;
        let right_bigger =
            left_sections.0 >= right_sections.0 && left_sections.1 <= right_sections.1;

        if left_bigger || right_bigger {
            total += 1;
        }
    }

    println!("{}", total);

    Ok(())
}

fn get_sections(input: &str) -> (u32, u32) {
    let mut ss = input.split("-");
    let l = ss.next().unwrap().parse().unwrap();
    let r = ss.next().unwrap().parse().unwrap();
    (l, r)
}

#[allow(dead_code)]
pub fn day4p2() -> Result<()> {
    let lines = read_lines("4")?;
    let mut total = 0;
    for l in lines {
        let mut pairs = l.split(",");
        let left = pairs.next().expect("fuck no left");
        let right = pairs.next().expect("fuck no right");

        let left_sections = get_sections(left);
        let right_sections = get_sections(right);

        let left_away = left_sections.1 < right_sections.0;
        let right_away = left_sections.0 > right_sections.1;

        if !(left_away || right_away) {
            total += 1;
        }
    }

    println!("{}", total);

    Ok(())
}

#[allow(dead_code)]
pub fn day5() -> Result<()> {
    let lines = read_lines("5")?;

    let mut sp = lines.split(|l| l == "");
    let board_input = sp.next().ok_or_else(|| anyhow!("fuck none"))?;
    let move_input = sp.next().ok_or_else(|| anyhow!("fuck none"))?;

    let mut board = parse_board(board_input);
    // dbg!(board);
    let ops = parse_ops(move_input)?;
    // dbg!(ops);

    for op in ops {
        // let from_vec = &mut board[op.from - 1];
        // let to_vec = board.get_mut(op.to - 1).unwrap();
        let count = usize::min(op.quantity, board[op.from - 1].len());

        for _ in 0..count {
            let tail = board[op.from - 1]
                .pop()
                .ok_or_else(|| anyhow!("fuck wrong count"))?;
            board[op.to - 1].push(tail);
        }
    }


    for col in board {
        print!("{}", col.last().unwrap_or(&' '));
    }
    println!();

    Ok(())
}

type Board = Vec<Vec<char>>;

fn parse_board(input: &[String]) -> Board {
    let mut idxes = vec![];
    let idx_line = &input[input.len() - 1];
    for i in 0..idx_line.len() {
        if idx_line.chars().nth(i).unwrap().is_ascii_digit() {
            idxes.push(i);
        }
    }
    // dbg!(&idxes);

    let mut board: Board = vec![vec![]; idxes.len()];
    for l in &input[0..input.len() - 1] {
        for (n, i) in idxes.iter().enumerate() {
            let ch = l.chars().nth(*i).unwrap();
            if ch.is_ascii_alphabetic() {
                board[n].insert(0, ch);
            }
        }
    }

    board
}

#[derive(Debug)]
struct Op {
    from: usize,
    to: usize,
    quantity: usize,
}

fn parse_ops(move_input: &[String]) -> Result<Vec<Op>> {
    let mut ops = Vec::<Op>::with_capacity(move_input.len());

    for line in move_input {
        let mut sp = line.split(" ");
        assert_eq!(sp.next(), Some("move"));
        let quantity: usize = match sp.next() {
            Some(dig) => dig.to_owned().parse()?,
            None => return Err(anyhow!("not digit")),
        };
        assert_eq!(sp.next(), Some("from"));
        let from: usize = match sp.next() {
            Some(dig) => dig.to_owned().parse()?,
            None => return Err(anyhow!("not digit")),
        };
        assert_eq!(sp.next(), Some("to"));
        let to: usize = match sp.next() {
            Some(dig) => dig.to_owned().parse()?,
            None => return Err(anyhow!("not digit")),
        };

        ops.push(Op { from, to, quantity })
    }

    Ok(ops)
}

#[allow(dead_code)]
pub fn day5p2() -> Result<()> {
    let lines = read_lines("5")?;

    let mut sp = lines.split(|l| l == "");
    let board_input = sp.next().ok_or_else(|| anyhow!("fuck none"))?;
    let move_input = sp.next().ok_or_else(|| anyhow!("fuck none"))?;

    let mut board = parse_board(board_input);
    // dbg!(board);
    let ops = parse_ops(move_input)?;
    // dbg!(ops);

    for op in ops {
        // let from_vec = &mut board[op.from - 1];
        // let to_vec = board.get_mut(op.to - 1).unwrap();
        let count = usize::min(op.quantity, board[op.from - 1].len());

		let last_to_idx = board[op.to - 1].len();
        for _ in 0..count {
            let tail = board[op.from - 1]
                .pop()
                .ok_or_else(|| anyhow!("fuck wrong count"))?;
            board[op.to - 1].insert(last_to_idx, tail);
        }
    }


    for col in board {
        print!("{}", col.last().unwrap_or(&' '));
    }
    println!();

    Ok(())
}