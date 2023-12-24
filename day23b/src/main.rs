use std::collections::{HashMap, HashSet};

use pathfinding::{
    directed::bfs,
    matrix::{directions::DIRECTIONS_4, Matrix},
};
use petgraph::{algo::all_simple_paths, dot::Dot, Graph, Undirected};

fn main() {
    let grid = include_str!("/tmp/input.txt")
        .lines()
        .map(|l| l.bytes())
        .collect::<Matrix<u8>>();
    let start = (0, 1); // row, col
    assert!(*grid.get(start).unwrap() == b'.');
    let end = (grid.rows - 1, grid.columns - 2);
    assert!(*grid.get(end).unwrap() == b'.');
    let successors = |&p: &(usize, usize)| {
        let mut neigh = Vec::new();
        match grid.get(p) {
            Some(b'#') => (),
            Some(_) => {
                for d in DIRECTIONS_4 {
                    if let Some(np) = grid.move_in_direction(p, d) {
                        neigh.push(np);
                    }
                }
            }
            _ => panic!("Unexpected grid value"),
        };
        neigh
            .into_iter()
            .filter_map(|p| {
                if grid.get(p) != Some(&b'#') {
                    Some(p)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };
    let mut g: Graph<(usize, usize), i32, Undirected> = Graph::new_undirected();
    let mut h = HashMap::new();
    let mut nodes = Vec::new();
    for row in 0..grid.rows {
        for col in 0..grid.columns {
            let p = (row, col);
            if grid.get(p) != Some(&b'#') && (successors(&p).len() > 2 || p == start || p == end) {
                h.insert(p, g.add_node(p));
                nodes.push(p);
            }
        }
    }
    // compacted
    // println!("{}", g.node_count());
    for &n in nodes.iter() {
        let mut seen = HashSet::new();
        for _ in 0..1.max(successors(&n).len()) {
            let path = bfs::bfs(
                &n,
                |neigh| match seen.contains(neigh) {
                    true => vec![],
                    false => {
                        if h.contains_key(neigh)
                            && g.contains_edge(*h.get(&neigh).unwrap(), *h.get(&n).unwrap())
                        {
                            return vec![];
                        }
                        successors(neigh)
                    }
                },
                |neigh| {
                    *neigh != n
                        && h.contains_key(neigh)
                        && !seen.contains(neigh)
                        && !g.contains_edge(*h.get(&neigh).unwrap(), *h.get(&n).unwrap())
                },
            );
            if let Some(path) = path {
                let neigh = path.last().unwrap().clone();
                seen.insert(neigh);
                g.add_edge(
                    *h.get(&n).unwrap(),
                    *h.get(&neigh).unwrap(),
                    path.len() as i32 - 1,
                );
            }
        }
    }
    if cfg!(debug_assertions) {
        println!("{:?}", Dot::with_config(&g, &[]));
    }

    println!(
        "{:?}",
        all_simple_paths::<Vec<_>, _>(&g, *h.get(&start).unwrap(), *h.get(&end).unwrap(), 0, None)
            .into_iter()
            .map(|path| {
                path.windows(2)
                    .map(|w| g.edges_connecting(w[0], w[1]).next().unwrap().weight())
                    .sum::<i32>()
            })
            .max()
            .unwrap()
    );
}
