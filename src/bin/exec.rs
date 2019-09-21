/*******************************************************************************
Title           : exec.rs
Author          : Keisuke Suzuki
Created on      : 9/21/19
Description     : executable crate
*******************************************************************************/
extern crate graph_practice;
use graph_practice::*;
use graph_practice::{vertices::V,
                     edges::E,
                     directed_graph::DirectedGraph};

fn main() {
    let node_elem = vec![23, 11, 15, 22, 7, 21];
    let vertices: Vec<Box<V<u32>>> = {
        node_elem.iter().map(|n| Box::new(
            V::new(*n))).collect()
    };
    
    let edges = vec![
        E::with(0, 1, 4),
        E::with(0, 2, 4),
        E::with(1, 2, 2),
        E::with(1, 0, 4),
        E::with(2, 0, 4),
        E::with(2, 1, 2),
        E::with(2, 3, 3),
        E::with(2, 5, 2),
        E::with(2, 4, 4),
        E::with(3, 2, 3),
        E::with(3, 4, 3),
        E::with(4, 2, 4),
        E::with(4, 3, 3),
        E::with(5, 2, 2),
        E::with(5, 4, 3),
    ];
    
    let size: usize = node_elem.len();
    let mut graph:DirectedGraph<u32> = DirectedGraph::new(vertices,
                                                          &edges,size);
    
    println!("circle: {}", graph.check_cycle());
    graph.dfs();
    graph.to_owned().print();
    graph.kruskal();
    graph.print();
    
}