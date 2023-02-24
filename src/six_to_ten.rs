use std::{cell::RefCell, collections::HashSet, fmt, rc::Rc};

use anyhow::{anyhow, Ok, Result};

use crate::common::read_lines;

#[allow(dead_code)]
pub fn day6() -> Result<()> {
    let input = &read_lines("6")?[0];
    let m = find_marker(input, 4);

    println!("{:?}", m);

    Ok(())
}

fn find_marker(input: &str, size: usize) -> Option<usize> {
    let mut window = Vec::<char>::with_capacity(size);
    for (i, ch) in input.chars().into_iter().enumerate() {
        if let Some(j) = window.iter().position(|&c| c == ch) {
            window.splice(0..j + 1, []);
        }
        window.push(ch);

        // full
        if window.len() == size {
            return Some(i + 1);
        }
    }

    None
}

#[test]
fn test_find_marker() {
    assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), Some(6));
    assert_eq!(
        find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4),
        Some(10)
    );
    assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), Some(11));
}

#[allow(dead_code)]
pub fn day6p2() -> Result<()> {
    let input = &read_lines("6")?[0];
    let m = find_marker(input, 14);

    println!("{:?}", m);

    Ok(())
}

#[allow(dead_code)]
pub fn day7() -> Result<()> {
    let input = read_lines("7")?;
    let fs = build_fs(input)?;
    update_dir_size(&fs);

    println!("{}", calc_small_dirs(&fs));

    Ok(())
}

fn calc_small_dirs(ent: &EntryRef) -> u32 {
    let mut total = 0;

    if ent.borrow().etype == EType::Dir && ent.borrow().size.unwrap() < 100000 {
        total = ent.borrow().size.unwrap();
    }

    for c in &ent.borrow().children {
        total += calc_small_dirs(c);
    }

    total
}

fn update_dir_size(ent: &EntryRef) -> u32 {
    let mut total = 0;
    let mut bm = ent.borrow_mut();

    for c in bm.children.iter() {
        let (etype, size) = {
            let cb = c.borrow();
            (cb.etype.clone(), cb.size.unwrap_or(0))
        };
        match etype {
            EType::Dir => {
                total += update_dir_size(&c);
            }
            EType::File => {
                total += size;
            }
        }
    }

    if bm.etype == EType::Dir {
        bm.size = Some(total);
    }

    total
}

fn build_fs(input: Vec<String>) -> Result<EntryRef> {
    let mut lines = input.into_iter();
    let root_line = lines.next().unwrap();
    let cd_root = parse_cmd(&root_line)?;
    assert_eq!(cd_root, Cmd::Cd("/".to_owned()));

    let root = Rc::new(RefCell::new(Entry {
        name: "/".to_owned(),
        etype: EType::Dir,
        size: None,
        children: vec![],
        parent: None,
    }));
    // let mut pwd = Rc::new(root);
    let mut pwd = root.clone();

    for l in lines {
        // println!("{}", l);
        // dbg!(&pwd);

        match l.chars().nth(0) {
            Some('$') => {
                // cd || ls
                match parse_cmd(&l)? {
                    Cmd::Cd(path) => {
                        match path.as_str() {
                            ".." => {
                                // go up
                                let parent = pwd.as_ref().borrow().parent.clone().unwrap();
                                pwd = parent;
                            }
                            _ => {
                                // go down
                                let child = pwd
                                    .borrow_mut()
                                    .children
                                    .iter()
                                    .find(|e| e.borrow_mut().name == path)
                                    .ok_or(anyhow!("not such dir"))?
                                    .clone();
                                pwd = child;
                            }
                        }
                    }
                    Cmd::Ls => {
                        // just continue
                    }
                };
            }
            Some(_) => {
                // let ent = parse_ls_result(&l, &pwd)?;
                // Rc::get_mut(&mut pwd).unwrap().children.push(ent);
                let ent = parse_ls_result(&l, &pwd)?;
                pwd.borrow_mut().children.push(Rc::new(RefCell::new(ent)));
            }
            None => return Err(anyhow!("empty line")),
        }
    }

    Ok(root)
}

fn parse_cmd(input: &str) -> Result<Cmd> {
    let sp: Vec<&str> = input.split(" ").collect();
    assert_eq!(sp[0], "$");
    match sp[1] {
        "cd" => Ok(Cmd::Cd(sp[2].to_string())),
        "ls" => Ok(Cmd::Ls),
        _ => Err(anyhow!("unknown cmd")),
    }
}

fn parse_ls_result(input: &str, pe: &Rc<RefCell<Entry>>) -> Result<Entry> {
    let mut sp = input.split(" ");
    let (etype, size) = match sp.next() {
        Some("dir") => (EType::Dir, None),
        Some(len) => (EType::File, Some(len.parse::<u32>().unwrap())),
        None => return Err(anyhow!("empty")),
    };
    let name = sp.next().ok_or(anyhow!("no name"))?.to_owned();

    Ok(Entry {
        name,
        etype,
        size,
        children: vec![],
        parent: Some(pe.clone()),
    })
}

#[derive(Debug, PartialEq)]
enum Cmd {
    Cd(String),
    Ls,
}

#[test]
fn test_build_fs() {
    let eg = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
    let r = build_fs(eg.split("\n").map(|s| s.to_owned()).collect()).unwrap();
    let ent = r.as_ref().borrow();
    assert_eq!(ent.name, "/");
    assert_eq!(ent.children[0].as_ref().borrow().name, "a");
    assert_eq!(ent.children[0].as_ref().borrow().etype, EType::Dir);
    assert_eq!(ent.children[1].as_ref().borrow().name, "b.txt");
    assert_eq!(ent.children[1].as_ref().borrow().size, Some(14848514));
    assert_eq!(ent.children[2].as_ref().borrow().name, "c.dat");
    assert_eq!(ent.children[2].as_ref().borrow().size, Some(8504156));
    assert_eq!(ent.children[3].as_ref().borrow().name, "d");
    assert_eq!(ent.children[3].as_ref().borrow().etype, EType::Dir);
    assert_eq!(
        ent.children[0].as_ref().borrow().children[0]
            .as_ref()
            .borrow()
            .children[0]
            .as_ref()
            .borrow()
            .size,
        Some(584)
    );
}

#[derive(Debug, PartialEq, Clone)]
enum EType {
    Dir,
    File,
}

type EntryRef = Rc<RefCell<Entry>>;

#[derive(Debug, Clone)]
struct Entry {
    name: String,
    etype: EType,
    size: Option<u32>,
    parent: Option<EntryRef>,
    children: Vec<EntryRef>,
}

struct PrettyNode<'a>(&'a EntryRef);

impl<'a> fmt::Debug for PrettyNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = self.0.borrow();
        if this.etype == EType::Dir {
            writeln!(f, "(dir, size={})", this.size.unwrap_or(0))?;
        } else {
            writeln!(f, "(file, size={})", this.size.unwrap_or(0))?;
        }

        for c in &this.children {
            let name = &c.borrow().name;
            // not very efficient at all, but shrug
            for (index, line) in format!("{:?}", PrettyNode(c)).lines().enumerate() {
                if index == 0 {
                    writeln!(f, "{name} {line}")?;
                } else {
                    writeln!(f, "  {line}")?;
                }
            }
        }

        fmt::Result::Ok(())
    }
}

const WANNA_SIZE: u32 = 40000000;

#[allow(dead_code)]
pub fn day7p2() -> Result<()> {
    let input = read_lines("7")?;
    let root = build_fs(input)?;
    update_dir_size(&root);
    // println!("{:#?}", PrettyNode(&root));

    let threshold = root.borrow().size.unwrap() - WANNA_SIZE;
    let sizes = collect_big_enough(&root, threshold);
    // dbg!(&sizes);
    let mut min = u32::MAX;
    for s in sizes {
        if s < min {
            min = s;
        }
    }

    println!("min: {}", min);

    Ok(())
}

fn collect_big_enough(ent: &EntryRef, threshold: u32) -> Vec<u32> {
    let mut result = vec![];
    let br = ent.borrow();
    let s = br.size.unwrap_or(0);
    if s > threshold {
        result.push(s);
    }

    for c in br.children.iter() {
        if c.borrow().etype == EType::Dir {
            let mut more = collect_big_enough(&c, threshold);
            result.append(&mut more);
        }
    }

    result
}

#[allow(dead_code)]
pub(crate) fn day8() -> Result<()> {
    let lines = read_lines("8")?;
    let n = count_trees(lines)?;

    println!("{}", n);

    Ok(())
}

type Forest = Vec<Vec<Tree>>;

#[derive(Debug, Default)]
struct Tree {
    height: i8,
    view: View,
}

#[derive(Debug, Default)]
struct View {
    l: u32,
    r: u32,
    t: u32,
    b: u32,
}

impl View {
    fn score(&self) -> u32 {
        self.l * self.r * self.t * self.b
    }
}

fn count_trees(lines: Vec<String>) -> Result<usize> {
    let forest = plant(&lines)?;
    // dbg!(&forest[0]);
    let xl = forest[0].len();
    let yl = forest.len();
    let mut xys = HashSet::<(usize, usize)>::new();

    for y in 0..yl {
        // left to right
        let mut highest = -1;
        for x in 0..xl {
            if forest[y][x].height > highest {
                // left_right.push((y, x));
                xys.insert((x, y));
                highest = forest[y][x].height;
            }
        }
        // right to left
        highest = -1;
        for j in 0..xl {
            let x = xl - j - 1;
            if forest[y][x].height > highest {
                // left_right.push((x, x));
                xys.insert((x, y));
                highest = forest[y][x].height;
            }
        }
    }

    for x in 1..xl {
        // top to bottom
        let mut highest = -1;
        for y in 0..yl {
            if forest[y][x].height > highest {
                // top_bottom.push((y, x));
                xys.insert((x, y));
                highest = forest[y][x].height;
            }
        }
        // bottom to top
        highest = -1;
        for j in 0..yl {
            let y = yl - j - 1;
            if forest[y][x].height > highest {
                // top_bottom.push((x, y));
                xys.insert((x, y));
                highest = forest[y][x].height;
            }
        }
    }
    // let edge_count = (xl + yl) * 2 - 4;
    Ok(xys.len())
}

fn plant(lines: &Vec<String>) -> Result<Forest> {
    let mut f = vec![];
    for l in lines {
        let mut row = vec![];
        for c in l.chars() {
            let h: i8 = String::from(c).parse()?;
            row.push(Tree {
                height: h,
                view: Default::default(),
            });
        }
        f.push(row);
    }

    Ok(f)
}

#[test]
fn test_count_trees() {
    let lines = vec![
        String::from("30373"),
        String::from("25512"),
        String::from("65332"),
        String::from("33549"),
        String::from("35390"),
    ];
    let n = count_trees(lines).unwrap();
    assert_eq!(n, 21);
}

#[allow(dead_code)]
pub(crate) fn day8p2() -> Result<()> {
    let lines = read_lines("8")?;
    let n = calc_score(&lines)?;

    println!("{}", n);

    Ok(())
}

fn calc_score(lines: &Vec<String>) -> Result<u32> {
    let mut forest = plant(lines)?;
    let xl = forest[0].len();
    let yl = forest.len();

    for y in 0..yl {
        // left to right
        for x in 0..xl {
            let mut count = 0;
            for prev in (0..x).rev() {
                count += 1;
                if forest[y][prev].height >= forest[y][x].height {
                    break;
                }
            }
            forest[y][x].view.l = count;
        }
        // right to left
        for x in (0..xl).rev() {
            let mut count = 0;
            for prev in x + 1..xl {
                count += 1;
                if forest[y][prev].height >= forest[y][x].height {
                    break;
                }
            }
            forest[y][x].view.r = count;
        }
    }

    for x in 0..xl {
        // top to bottom
        for y in 0..yl {
            let mut count = 0;
            for prev in (0..y).rev() {
                count += 1;
                if forest[prev][x].height >= forest[y][x].height {
                    break;
                }
            }
            forest[y][x].view.t = count;
        }
        // bottom to top
        for y in (0..yl).rev() {
            let mut count = 0;
            for prev in y + 1..yl {
                count += 1;
                if forest[prev][x].height >= forest[y][x].height {
                    break;
                }
            }
            forest[y][x].view.b = count;
        }
    }

    // dbg!(&forest);

    let mut ans = 0;
    for y in 0..yl {
        for x in 0..xl {
            let s = forest[y][x].view.score();
            if s > ans {
                ans = s;
            }
        }
    }

    Ok(ans)
}

#[test]
fn test_calc_score() {
    let lines = vec![
        String::from("30373"),
        String::from("25512"),
        String::from("65332"),
        String::from("33549"),
        String::from("35390"),
    ];
    let n = calc_score(&lines).unwrap();
    assert_eq!(n, 8);
}

#[allow(dead_code)]
pub(crate) fn day9() -> Result<()> {
    let cmds = parse_d9_cmd(&read_lines("9")?)?;
    // println!("{:#?}", cmds[0]);
    let count = count_pos(&cmds)?;

    println!("{}", count);

    Ok(())
}

fn parse_d9_cmd(lines: &Vec<String>) -> Result<Vec<D9Cmd>> {
    let mut result = vec![];

    for l in lines {
        let mut sp = l.split(" ");
        let letter = sp.next().ok_or(anyhow!("no one"))?;
        let digit = sp.next().ok_or(anyhow!("no two"))?;

        let dir = match letter {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(anyhow!("unknown cmd")),
        };
        let step = String::from(digit).parse()?;
        result.push(D9Cmd { dir, step });
    }

    Ok(result)
}

fn count_pos(cmds: &Vec<D9Cmd>) -> Result<usize> {
    let mut head: Pos2D = (0, 0);
    let mut _prev_head: Pos2D = (0, 0);
    let mut tail: Pos2D = (0, 0);
    let mut tails = HashSet::<Pos2D>::new();
    // let mut prev_dir: Option<Direction> = None;

    for cmd in cmds {
        let dir = &cmd.dir;
        let step = cmd.step;

        for _ in 0..step {
            _prev_head = head;

            match dir {
                Direction::Left => head = (head.0 - 1, head.1),
                Direction::Right => head = (head.0 + 1, head.1),
                Direction::Up => head = (head.0, head.1 + 1),
                Direction::Down => head = (head.0, head.1 - 1),
            };

            if far_enough(&head, &tail) {
                tail = _prev_head;
                tails.insert(tail);
            }
        }
    }

    // tails.insert(prev_head);

    // let mut v: Vec<Pos2D> = tails.clone().into_iter().collect();
    // v.sort();
    // dbg!(v);

    Ok(tails.len() + 1)
}

fn far_enough(p1: &Pos2D, p2: &Pos2D) -> bool {
    (p1.0 - p2.0).abs() > 1 || (p1.1 - p2.1).abs() > 1
}

#[test]
fn test_count_pos() {
    let lines = vec![
        "R 4".to_owned(),
        "U 4".to_owned(),
        "L 3".to_owned(),
        "D 1".to_owned(),
        "R 4".to_owned(),
        "D 1".to_owned(),
        "L 5".to_owned(),
        "R 2".to_owned(),
    ];
    let cmds = parse_d9_cmd(&lines).unwrap();
    let cnt = count_pos(&cmds).unwrap();
    assert_eq!(cnt, 13);
}

#[derive(Debug)]
struct D9Cmd {
    dir: Direction,
    step: u8,
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

type Pos2D = (i32, i32);

#[allow(dead_code)]
pub(crate) fn day9p2() -> Result<()> {
    let cmds = parse_d9_cmd(&read_lines("9")?)?;
    // println!("{:#?}", cmds[0]);
    let count = count_pos_p2(&cmds)?;

    println!("{}", count);

    Ok(())
}

fn count_pos_p2(cmds: &Vec<D9Cmd>) -> Result<usize> {
    // let mut head: Pos2D = (0, 0);
    let mut tails = HashSet::<Pos2D>::new();
    let mut snake: Vec<Pos2D> = vec![(0, 0); 10];
    // let mut prev_snake: Vec<Pos2D> = vec![(0, 0); 10];
    // let mut prev_dir: Option<Direction> = None;

    for cmd in cmds {
        let dir = &cmd.dir;
        let step = cmd.step;

        for _i in 0..step {
            // for i in 0..prev_snake.len() {
            //     prev_snake[i] = snake[i].clone();
            // }

            let head = snake.get_mut(0).unwrap();
            match dir {
                Direction::Left => *head = (head.0 - 1, head.1),
                Direction::Right => *head = (head.0 + 1, head.1),
                Direction::Up => *head = (head.0, head.1 + 1),
                Direction::Down => *head = (head.0, head.1 - 1),
            };

            for j in 1..snake.len() {
                if far_enough(&snake[j], &snake[j - 1]) {
                    snake[j] = move_towards(&snake[j], &snake[j - 1]);
                }
            }

            tails.insert(snake[9].clone());
        }

        dbg!(&snake);
    }

    Ok(tails.len())
}

fn move_towards(from: &Pos2D, to: &Pos2D) -> Pos2D {
    let from_x = from.0;
    let from_y = from.1;
    let to_x = to.0;
    let to_y = to.1;

    if from_x == to_x {
        // horizontal
        if to_y > from_y {
            (from_x, from_y + 1)
        } else {
            (from_x, from_y - 1)
        }
    } else if from_y == to_y {
        // vertical
        if to_x > from_x {
            (from_x + 1, from_y)
        } else {
            (from_x - 1, from_y)
        }
    } else {
        // diagonal
        let x = {
            if to_x > from_x {
                from_x + 1
            } else {
                from_x - 1
            }
        };
        let y = {
            if to_y > from_y {
                from_y + 1
            } else {
                from_y - 1
            }
        };
        (x, y)
    }
}

#[test]
fn test_count_pos_p2() {
    let lines = vec![
        "R 5".to_owned(),
        "U 8".to_owned(),
        "L 8".to_owned(),
        "D 3".to_owned(),
        "R 17".to_owned(),
        "D 10".to_owned(),
        "L 25".to_owned(),
        "U 20".to_owned(),
    ];
    let cmds = parse_d9_cmd(&lines).unwrap();
    let cnt = count_pos_p2(&cmds).unwrap();
    assert_eq!(cnt, 36);
}

#[test]
fn test_count_pos_p2_2() {
    let lines = vec![
        "R 4".to_owned(),
        "U 4".to_owned(),
        "L 3".to_owned(),
        "D 1".to_owned(),
        "R 4".to_owned(),
        "D 1".to_owned(),
        "L 5".to_owned(),
        "R 2".to_owned(),
    ];
    let cmds = parse_d9_cmd(&lines).unwrap();
    let cnt = count_pos_p2(&cmds).unwrap();
    assert_eq!(cnt, 1);
}

#[allow(dead_code)]
pub(crate) fn day10() -> Result<()> {
    let lines = read_lines("10")?;
    let ans = ans_day10(&lines)?;

    println!("{}", ans);

    Ok(())
}

fn get_circle_xs(lines: &Vec<String>) -> Result<Vec<i32>> {
    let cmds = day10_parser(lines)?;

    let mut xs = vec![1];
    let mut circle = 0;
    for cmd in cmds {
        match cmd {
            D10Cmd::Noop => {
                xs.push(xs[circle]);
                circle += 1;
            }
            D10Cmd::Addx(n) => {
                xs.push(xs[circle]);
                circle += 1;
                xs.push(xs[circle] + n);
                circle += 1;
            }
        }
    }

    Ok(xs)
}

#[test]
fn test_get_circle_xs() {
    let xs = get_circle_xs(&vec![
        "noop".to_owned(),
        "addx 3".to_owned(),
        "addx -5".to_owned(),
    ])
    .unwrap();
    assert_eq!(xs, vec![1, 1, 1, 4, 4, -1])
}

fn ans_day10(lines: &Vec<String>) -> Result<i32> {
    let xs = get_circle_xs(lines)?;
    let cs = vec![20, 60, 100, 140, 180, 220];
    let mut ans = 0;
    for c in cs {
        ans += c as i32 * xs[c - 1];
    }

    Ok(ans)
}

fn day10_parser(lines: &Vec<String>) -> Result<Vec<D10Cmd>> {
    let mut result = vec![];

    for l in lines {
        let mut sp = l.split(" ");
        let op = sp.next().ok_or(anyhow!("no op"))?;
        match op {
            "noop" => result.push(D10Cmd::Noop),
            "addx" => {
                let digit = sp.next().ok_or(anyhow!("no digit"))?;
                let num = String::from(digit).parse()?;
                result.push(D10Cmd::Addx(num));
            }
            _ => return Err(anyhow!("unknown op")),
        }
    }

    Ok(result)
}

#[derive(Debug)]
enum D10Cmd {
    Noop,
    Addx(i32),
}

#[test]
fn test_ans_day10() {
    let input = read_lines("10-test").unwrap();
    let xs = get_circle_xs(&input).unwrap();
    assert_eq!(xs[19], 21);
    assert_eq!(xs[59], 19);
    assert_eq!(xs[99], 18);
    assert_eq!(xs[139], 21);
    assert_eq!(xs[179], 16);
    assert_eq!(xs[219], 18);

    let ans = ans_day10(&input).unwrap();
    assert_eq!(ans, 13140);
}

#[allow(dead_code)]
pub(crate) fn day10p2() -> Result<()> {
    let lines = read_lines("10")?;
    let xs = get_circle_xs(&lines)?;

    println!("{}", xs.len());

    let mut screen = vec![String::new(); 6];
    for i in 0..240 {
        let sprite_pos = xs[i];
        let screen_pos = (i % 40) as i32;
        if screen_pos >= sprite_pos - 1 && screen_pos <= sprite_pos + 1 {
            screen[i / 40].push('#')
        } else {
            screen[i / 40].push('.')
        }
    }

    for row in screen {
        println!("{}", row)
    }

    Ok(())
}
