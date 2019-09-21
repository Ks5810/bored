/*******************************************************************************
Title           : directed_graph.rs
Author          : Keisuke Suzuki
Created on      : 9/21/19
Modification    :
*******************************************************************************/
use priority_queue::PriorityQueue;
use std::convert::{TryFrom, TryInto};
use crate::{vertices::V, edges::E, disjoint_set::DisjointSet};
use std::collections::VecDeque;

//directed graph
#[derive(Debug, Clone)]
pub struct DirectedGraph<T> {
    adj_list: Vec<Box<V<T>>>,
    edges: Vec<E>,
    size: usize
}

impl<T> DirectedGraph<T>
    where T: std::fmt::Debug,
          T: std::clone::Clone {
    fn construct_adj_list(_adj_list: Vec<Box<V<T>>>, _edges: &Vec<E>, _size:
    usize) -> Vec<Box<V<T>>> {
        let mut return_val: Vec<Box<V<T>>> = {
            _adj_list.iter().map(|adj| Box::new(V::with(
                adj.element.clone(),
                adj.in_degree,
                adj.num_visited,
                adj.adj.clone()
            ))).collect()
        };
        for edge in _edges.iter() {
            return_val[edge.src].adj.push_back(edge.dest);
            return_val[edge.dest].in_degree += 1;
        }
        return_val
    }
    fn construct_edges(edges: &Vec<E>) -> Vec<E> {
        edges.clone()
    }
    pub fn new(_adj: Vec<Box<V<T>>>, _edges: &Vec<E>, size: usize) -> Self {
        let adj_list = Self::construct_adj_list(_adj, _edges, size);
        let edges = Self::construct_edges(&_edges);
        Self { adj_list, edges, size }
    }
    
    pub fn print(self) {
        println!("{}", "Edge  :  Weight");
        for edge in self.edges.iter() {
            println!("{} - {} : {}", edge.src, edge.dest, edge.weight);
        }
    }
    
    pub fn check_cycle(&mut self) -> bool {
        //use queue
        let mut que: VecDeque<usize> = VecDeque::new();
        let mut count: usize = 0;
        //push indexes which has indegree of
        for (i, item) in self.adj_list.iter().enumerate().to_owned() {
            if item.in_degree == 0 {
                que.push_back(i);
            }
        }
        
        while !que.is_empty() {
            let item = *que.front().unwrap();
            que.pop_front();
            count += 1;
            self.adj_list[item].in_degree += 1;
            
            for adj in self.adj_list[item].adj.to_owned() {
                self.adj_list[adj].in_degree -= 1;
                if self.adj_list[adj].in_degree == 0 {
                    que.push_back(adj);
                }
            }
        }
        return count != self.size;
    }
    
    fn _dfs(&mut self, i: usize) {
        self.adj_list[i].num_visited += 1;
        println!("element {:?}", self.adj_list[i]);
        for it in self.adj_list[i].adj.to_owned() {
            if self.adj_list[it].num_visited == 0 {
                self._dfs(it);
            }
        }
    }
    pub fn dfs(&mut self) {
        for mut i in self.adj_list.to_owned() {
            i.num_visited = 0;
        }
        for (i, adj) in self.adj_list.to_owned().iter().enumerate() {
            if adj.num_visited == 0 {
                self._dfs(i);
            }
        }
    }
    pub fn kruskal(&mut self) {
        let mut dis_sets: DisjointSet = DisjointSet::new(self.size);
        let mut u_set: usize;
        let mut v_set: usize;
        let mut tmp_edges = vec![];
        let mut edges_accepted: usize = 0;
        let mut priority_queue =
        PriorityQueue::with_capacity(self.size);
        
        for edge in self.edges.iter() {
            priority_queue.push(edge, edge.weight);
        }
        
        while edges_accepted < (self.size - 1) && !priority_queue.is_empty() {
            let (edge, _p) = priority_queue.pop().unwrap();
            let src = isize::try_from(edge.src).unwrap();
            let weight = isize::try_from(edge.dest).unwrap();
            u_set = dis_sets.find(src).try_into().unwrap();
            v_set = dis_sets.find(weight).try_into().unwrap();
            
            if u_set != v_set {
                tmp_edges.push(*edge);
                edges_accepted += 1;
                dis_sets.union_set(u_set, v_set);
            }
        }
        self.edges = tmp_edges;
    }
    #[allow(dead_code)]
    fn reset(&mut self) {
        for mut adj in self.adj_list.to_owned() {
            adj.num_visited = 0;
        }
    }
}