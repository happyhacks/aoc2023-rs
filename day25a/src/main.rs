use std::collections::HashMap;

use petgraph::{
    dot::{Config, Dot},
    Graph,
};
use rustworkx_core::connectivity::stoer_wagner_min_cut;

fn main() {
    let h: HashMap<String, Vec<String>> = include_str!("/tmp/input.txt")
        .trim_end()
        .lines()
        .map(|l| {
            let (parent, children) = l.split_once(": ").unwrap();
            (
                parent.to_string(),
                children
                    .split_ascii_whitespace()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>(),
            )
        })
        .collect();
    // dbg!(&h);
    let mut g = Graph::new_undirected();
    let mut nodes = HashMap::new();
    for (k, children) in h.iter() {
        let src = *nodes.entry(k).or_insert_with(|| g.add_node(k));
        for child in children {
            let dst = *nodes.entry(child).or_insert_with(|| g.add_node(child));
            g.add_edge(src, dst, 1);
        }
    }
    if cfg!(debug_assertions) {
        // neato
        println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
    }
    // why code when dot works?
    // for cut in vec![("xxq", "hqq"), ("vkd", "qfb"), ("xzz", "kgl")] {
    //     g.remove_node(*nodes.get(&cut.0.to_string()).unwrap())
    //         .expect("check graph");
    //     g.remove_node(*nodes.get(&cut.1.to_string()).unwrap())
    //         .expect("check graph");
    // }
    // let n = g.node_count();
    // let mut c = 0;
    // let mut dfs = Dfs::new(&g, g.node_indices().next().unwrap());
    // while let Some(_) = dfs.next(&g) {
    //     c += 1;
    // }
    // println!("{}", (c + 3) * (n - c + 3));
    let min_cut_res: Result<Option<(usize, Vec<_>)>, _> =
        stoer_wagner_min_cut(&g, |_| Ok::<usize, usize>(1));
    if let Ok(Some((cut, part))) = min_cut_res {
        assert!(cut == 3);
        println!("{:?}", part.len() * (g.node_count() - part.len()));
    }
}
