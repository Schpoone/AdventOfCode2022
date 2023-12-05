use glam::u32::UVec2;
use pathfinding::directed::dijkstra::dijkstra;
use std::collections::hash_map::HashMap;

#[derive(Debug)]
struct HeightMap {
    start: UVec2,
    end: UVec2,
    map: HashMap<UVec2, u32>,
}

fn parse_map(text: String) -> HeightMap {
    let mut map = HeightMap {
        start: UVec2::ZERO,
        end: UVec2::ZERO,
        map: HashMap::new(),
    };
    for (y, line) in text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as u32;
            let y = y as u32;
            match c {
                'S' => {
                    map.start = UVec2 { x, y };
                    map.map.insert(UVec2 { x, y }, 0)
                }
                'E' => {
                    map.end = UVec2 { x, y };
                    map.map.insert(UVec2 { x, y }, 25)
                }
                c => map.map.insert(UVec2 { x, y }, c as u32 - 97),
            };
        }
    }
    map
}

pub fn part1(text: String) -> u32 {
    let map = parse_map(text);
    let result = dijkstra(
        &map.start,
        |p| {
            let height = *map.map.get(p).unwrap();
            let adj_nodes = [
                p.saturating_add(UVec2::new(1, 0)),
                p.saturating_add(UVec2::new(0, 1)),
                p.saturating_sub(UVec2::new(1, 0)),
                p.saturating_sub(UVec2::new(0, 1)),
            ];
            adj_nodes
                .into_iter()
                .filter_map(|adj_node| {
                    if &adj_node == p {
                        return None;
                    }
                    if !map.map.contains_key(&adj_node) {
                        return None;
                    }
                    if map.map.get(&adj_node).unwrap().saturating_sub(height) <= 1 {
                        Some((adj_node, 1))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(UVec2, u32)>>()
        },
        |p| p == &map.end,
    );
    result.expect("No path found").1
}

#[derive(Debug)]
struct HeightMap2 {
    start: Vec<UVec2>,
    end: UVec2,
    map: HashMap<UVec2, u32>,
}

fn parse_map2(text: String) -> HeightMap2 {
    let mut map = HeightMap2 {
        start: Vec::new(),
        end: UVec2::ZERO,
        map: HashMap::new(),
    };
    for (y, line) in text.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let x = x as u32;
            let y = y as u32;
            match c {
                'S' => {
                    map.start.push(UVec2 { x, y });
                    map.map.insert(UVec2 { x, y }, 0);
                }
                'E' => {
                    map.end = UVec2 { x, y };
                    map.map.insert(UVec2 { x, y }, 25);
                }
                c => {
                    let height = c as u32 - 97;
                    if height == 0 {
                        map.start.push(UVec2 { x, y });
                    }
                    map.map.insert(UVec2 { x, y }, c as u32 - 97);
                }
            };
        }
    }
    map
}

pub fn part2(text: String) -> u32 {
    let map = parse_map2(text);
    let mut min_steps = u32::MAX;
    for start in map.start.iter() {
        let result = dijkstra(
            start,
            |p| {
                let height = *map.map.get(p).unwrap();
                let adj_nodes = [
                    p.saturating_add(UVec2::new(1, 0)),
                    p.saturating_add(UVec2::new(0, 1)),
                    p.saturating_sub(UVec2::new(1, 0)),
                    p.saturating_sub(UVec2::new(0, 1)),
                ];
                adj_nodes
                    .into_iter()
                    .filter_map(|adj_node| {
                        if &adj_node == p {
                            return None;
                        }
                        if !map.map.contains_key(&adj_node) {
                            return None;
                        }
                        if map.map.get(&adj_node).unwrap().saturating_sub(height) <= 1 {
                            Some((adj_node, 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<(UVec2, u32)>>()
            },
            |p| p == &map.end,
        );
        let num_steps = result.or_else(|| Some((Vec::new(), u32::MAX))).unwrap().1;
        if num_steps < min_steps {
            min_steps = num_steps;
        }
    }
    min_steps
}
