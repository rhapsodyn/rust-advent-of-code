use std::{collections::HashMap, str::Split};

use regex::Regex;

use crate::common::read_lines;

/// TODO some kinda `symbol` ??
type Name = String;

#[derive(Debug)]
struct Spec {
    name: Name,
    rate: u64,
    leads_to: Vec<String>,
    // on: bool,
}

type SpecGraph = HashMap<Name, Spec>;

type LenCache = HashMap<String, u64>;

type PressureCache = HashMap<String, u64>;

#[allow(dead_code)]
pub(crate) fn ans() {
    let lines = read_lines("16").unwrap();
    let a = _ans1_2(&lines);
    println!("{}", a)
}

///
/// JUST greedy ??
/// not ok :(
///
// fn _ans1(lines: &Vec<String>) -> u64 {
//     let mut graph = parse(lines);
//     // dbg!(graph);
//     let mut min = 30;
//     let mut pressure = 0;
//     let mut curr_name = "AA".to_string();
//     while min > 0 {
//         let (p, next) = tick(&mut graph, &mut min, &curr_name);
//         println!("p:{}, next:{}, min: {}", &p, &next, &min);
//         pressure += p;
//         curr_name = next;
//     }

//     pressure
// }

///
/// dynamic programming
///
fn _ans1_2(lines: &Vec<String>) -> u64 {
    let (graph, mut off_names) = parse(lines);
    // let mut cache = build_len_cache(&graph);
    let mut cache = LenCache::new();
    let mut p_cache = PressureCache::new();
    most_pressure(
        &graph,
        30,
        &"AA".to_string(),
        &mut off_names,
        &mut cache,
        &mut p_cache,
    )
}

///
/// too slow to pre-build
///
// fn build_len_cache(graph: &SpecGraph) -> LenCache {
//     let mut cache = LenCache::new();
//     for from in graph.keys() {
//         for to in graph.keys() {
//             let k = get_len_key(from, to);
//             if !cache.contains_key(&k) {
//                 cache.insert(k, shortest(graph, from, to));
//             }
//         }
//     }

//     cache
// }

///
/// ~~without pressure cache, it takes 10s to complete~~       
///
/// ~~get along with it~~
///
/// more complex data structure & param, lighter `clone`, makes 1s
///
fn most_pressure(
    graph: &SpecGraph,
    min_left: u64,
    curr_name: &Name,
    off_names: &mut Vec<Name>,
    l_cache: &mut LenCache,
    p_cache: &mut PressureCache,
) -> u64 {
    // let mut off_names: Vec<Name> = graph
    //     .values()
    //     .filter(|v| !v.on)
    //     .map(|v| v.name.to_string())
    //     .collect();

    off_names.sort();
    let p_cache_key = format!("{}-{}-{}", curr_name, min_left, off_names.join(""));
    if let Some(p) = p_cache.get(&p_cache_key) {
        return *p;
    }

    let mut targeting_paths: Vec<(Name, u64)> = vec![];
    for n in off_names.iter() {
        let len = shortest(&graph, curr_name, &n, l_cache);
        // let len = cache[&get_len_key(curr_name, &n)];
        // no time to move
        if len < min_left - 1 {
            targeting_paths.push((n.to_string(), len));
        }
    }

    let mut max = u64::MIN;
    if targeting_paths.len() == 0 {
        // no valve to turn on
        max = get_total_pressure(graph, &off_names, min_left)
    } else {
        for (next, l) in targeting_paths {
            // let mut g = graph.clone();
            // turning on waste 1min
            let so_far = get_total_pressure(graph, &off_names, l + 1);
            // g.get_mut(&next).unwrap().on = true;
            let mut new_off = off_names.clone();
            let mut remove_idx = None;
            for (i, n) in new_off.iter().enumerate() {
                if n == &next {
                    remove_idx = Some(i);
                    break;
                }
            }
            new_off.remove(remove_idx.unwrap());
            let p = so_far
                + most_pressure(
                    graph,
                    min_left - l - 1,
                    &next,
                    &mut new_off,
                    l_cache,
                    p_cache,
                );
            if p > max {
                // println!("{next}: {p}");
                max = p;
            }
        }
    }

    p_cache.insert(p_cache_key, max);
    max
}

///
/// lenof(from, to) = lenof(to, from)
///
fn join_key(from: &str, to: &str) -> String {
    if from > to {
        to.to_owned() + from
    } else {
        from.to_owned() + to
    }
}

// fn tick(graph: &mut SpecGraph, min_left: &mut u64, curr_name: &Name) -> (u64, Name) {
//     let still_off_names: Vec<Name> = graph
//         .values()
//         .filter(|v| !v.on)
//         .map(|v| v.name.to_string())
//         .collect();

//     if still_off_names.len() == 0 {
//         let p = get_total_pressure(graph, *min_left);
//         *min_left = 0;
//         return (p, curr_name.to_owned());
//     }

//     let mut paths: Vec<(Name, u64)> = vec![];
//     for n in still_off_names {
//         let len = shortest(graph, curr_name, &n);
//         if len < *min_left - 1 {
//             paths.push((n, len));
//         }
//     }
//     // println!("{:?}", &paths);

//     let mut farest = u64::MIN;
//     for (_, l) in paths.iter() {
//         if *l > farest {
//             farest = *l;
//         }
//     }
//     // one minute to turn on
//     // one minute to work
//     farest += 1 + 1;

//     let mut max_pressure = 0;
//     let mut target = None;
//     for (n, l) in paths {
//         let p = (farest - l - 1) * graph[&n].rate;
//         if p > max_pressure {
//             max_pressure = p;
//             target = Some((n, l));
//         }
//     }

//     if let Some((n, l)) = target {
//         let p = get_total_pressure(graph, l);
//         graph.get_mut(&n).unwrap().on = true;
//         *min_left -= l + 1;
//         (p, n)
//     } else {
//         panic!("unreachable")
//     }
// }

fn get_total_pressure(graph: &SpecGraph, off_names: &Vec<Name>, min_duration: u64) -> u64 {
    // let per: u64 = graph.values().filter(|v| v.on).map(|v| v.rate).sum();
    let mut per = 0;
    for v in graph.values() {
        if !off_names.contains(&v.name) {
            per += v.rate;
        }
    }
    per * min_duration
}

fn shortest(graph: &SpecGraph, from: &Name, to: &Name, cache: &mut LenCache) -> u64 {
    let cache_key = join_key(from, to);
    if let Some(len) = cache.get(&cache_key) {
        return *len;
    }

    let mut depth = 1;
    let mut queue = graph[from].leads_to.to_vec();

    // go breadth-first
    loop {
        if depth > graph.len() {
            break;
        }

        let mut next_level = vec![];
        for t in queue {
            if &t == to {
                cache.insert(cache_key, depth as u64);
                return depth as u64;
            }

            let mut ts = graph[&t].leads_to.to_vec();
            next_level.append(&mut ts);
        }

        depth += 1;
        queue = next_level;
    }

    panic!("not found")
}

fn parse(lines: &Vec<String>) -> (SpecGraph, Vec<Name>) {
    let mut specs = HashMap::new();
    let mut off_names = vec![];

    let reg = Regex::new(r"rate=(\d+);").unwrap();
    for l in lines {
        let mut sp = l.split(" ");
        assert_const_str(&mut sp, "Valve");
        let name = sp.next().unwrap().to_string();
        assert_const_str(&mut sp, "has");
        assert_const_str(&mut sp, "flow");
        let rate_equal = sp.next().unwrap();
        let rate_str = &reg.captures_iter(rate_equal).next().unwrap()[1];
        let rate = rate_str.parse().unwrap();
        let t = sp.next().unwrap();
        assert!(t == "tunnel" || t == "tunnels");
        let t = sp.next().unwrap();
        assert!(t == "lead" || t == "leads");
        assert_const_str(&mut sp, "to");
        let t = sp.next().unwrap();
        assert!(t == "valve" || t == "valves");
        let mut leads_to = vec![];
        while let Some(to) = sp.next() {
            if to.ends_with(",") {
                leads_to.push(to[0..to.len() - 1].to_string());
            } else {
                leads_to.push(to.to_string());
            }
        }

        specs.insert(
            name.to_owned(),
            Spec {
                name: name.to_owned(),
                rate,
                leads_to,
                // no rate, no care
                // on: rate == 0,
            },
        );

        if rate > 0 {
            // all valves with pressure init to off
            off_names.push(name);
        }
    }

    (specs, off_names)
}

fn assert_const_str(sp: &mut Split<&str>, to_be: &str) {
    assert_eq!(sp.next().unwrap(), to_be);
}

// == Minute 2 ==
// No valves are open.
// You open valve DD.
// == Minute 5 ==
// Valve DD is open, releasing 20 pressure.
// You open valve BB.
// == Minute 9 ==
// Valves BB and DD are open, releasing 33 pressure.
// You open valve JJ.
// == Minute 17 ==
// Valves BB, DD, and JJ are open, releasing 54 pressure.
// You open valve HH.
// == Minute 21 ==
// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
// You open valve EE.
// == Minute 24 ==
// Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
// You open valve CC.
#[test]
fn test_ans1() {
    let lines = read_lines("16-test").unwrap();
    // let a = _ans1(&lines);
    let a = _ans1_2(&lines);
    assert_eq!(a, 1651);
}

#[allow(dead_code)]
pub(crate) fn ans2() {
    let lines = read_lines("16").unwrap();
    let a = _ans2(&lines);
    println!("{}", a)
}

#[test]
fn test_ans2() {
    let lines = read_lines("16-test").unwrap();
    let a = _ans2(&lines);
    assert_eq!(a, 1707);
}

fn _ans2(lines: &Vec<String>) -> u64 {
    let (graph, mut off_names) = parse(lines);
    let mut p_cache = PressureCache::new();
    let me_start = String::from("AA");
    let elephant_start = String::from("AA");
    let a = most_pressure_with_elephant(
        &graph,
        26,
        &me_start,
        &elephant_start,
        &mut off_names,
        // &mut cache,
        &mut p_cache,
        &[&me_start, &elephant_start],
    );
    // dbg!(&p_cache);
    a
}

///
/// tooooo slow, just give up
/// 
fn most_pressure_with_elephant(
    graph: &SpecGraph, 
    min_left: u64,     
    me_from: &Name,
    elephant_from: &Name,
    off_names: &mut Vec<Name>,   
    p_cache: &mut PressureCache, 
    prev_names: &[&Name; 2],
) -> u64 {
    if min_left == 0 {
        return 0;
    }

    // let this_min = get_total_pressure(graph, off_names, 1);
    off_names.sort();
    let key = format!(
        "{}-{}-{}",
        join_key(me_from, elephant_from),
        min_left,
        off_names.join("")
    );

    if p_cache.contains_key(&key) {
        p_cache[&key]
    } else {
        if off_names.len() == 0 {
            let max = get_total_pressure(graph, off_names, min_left);
            p_cache.insert(key, max);
            max
        } else {
            let me_to = &graph[me_from].leads_to;
            let elephant_to = &graph[elephant_from].leads_to;
            let mut me_off_idx = None;
            let mut elephant_off_idx = None;
            for (i, n) in off_names.iter().enumerate() {
                if n == me_from {
                    me_off_idx = Some(i);
                } else if n == elephant_from {
                    elephant_off_idx = Some(i);
                }
            }

            let mut after_max = 0;
            match (me_off_idx, elephant_off_idx) {
                (None, None) => {}
                (None, Some(e)) => {
                    // elephant turn on
                    let mut new_off = off_names.clone();
                    new_off.remove(e);

                    for m in me_to {
                        if !prev_names.contains(&m) {
                            let next = most_pressure_with_elephant(
                                graph,
                                min_left - 1,
                                m,
                                elephant_from, // keep still
                                &mut new_off,
                                p_cache,
                                &[me_from, elephant_from],
                            );
                            after_max = after_max.max(next);
                        }
                    }
                }
                (Some(m), None) => {
                    // me turn on
                    let mut new_off = off_names.clone();
                    new_off.remove(m);

                    for e in elephant_to {
                        if !prev_names.contains(&e) {
                            let next = most_pressure_with_elephant(
                                graph,
                                min_left - 1,
                                me_from, // keep still
                                e,
                                &mut new_off,
                                p_cache,
                                &[me_from, elephant_from],
                            );
                            after_max = after_max.max(next);
                        }
                    }
                }
                (Some(m), Some(e)) => {
                    // both turn on
                    let mut new_off = vec![];
                    for (i, n) in off_names.iter().enumerate() {
                        if i != m && i != e {
                            new_off.push(n.to_owned());
                        }
                    }

                    let next = most_pressure_with_elephant(
                        graph,
                        min_left - 1,
                        me_from,
                        elephant_from,
                        &mut new_off,
                        p_cache,
                        &[me_from, elephant_from],
                    );
                    after_max = after_max.max(next);
                }
            }

            // both not touching
            for m in me_to {
                for e in elephant_to {
                    if prev_names.contains(&m) || prev_names.contains(&e) {
                        continue;
                    }
                    let next = most_pressure_with_elephant(
                        graph,
                        min_left - 1,
                        m,
                        e,
                        off_names,
                        p_cache,
                        &[me_from, elephant_from],
                    );
                    after_max = after_max.max(next);
                }
            }

            let this_min = get_total_pressure(graph, off_names, 1);
            let max = this_min + after_max;
            p_cache.insert(key, max);
            max
        }
    }
}
