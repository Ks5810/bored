/*******************************************************************************
Title           : main.rs
Author          : Keisuke Suzuki
Created on      : Aug 27, 2019
****************************************************************************/
use std::collections::VecDeque;
use std::collections::LinkedList;
use std::vec::Vec;
use std::convert::{TryInto, TryFrom};
use std::cmp::Ordering;
use priority_queue::PriorityQueue;
use std::iter::Iterator;


//vertices
#[derive(Debug,Clone)]
struct V<T>{
    element : T,
    in_degree: u32,
    num_visited: usize,
    adj: LinkedList<usize>
}

//edge
#[derive(Debug,Copy,Clone,Hash,Eq)]
struct E{
    src: usize,
    dest: usize,
    weight: i32
}

//edge impl
impl E{
    fn new()->E{ E{src:0,dest:0,weight:0} }
    fn with(src:usize,dest:usize,weight:i32)->E{ E{src,dest,weight } }
}
impl Ord for E{
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}
impl PartialOrd for E{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for E{
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}
#[derive(Debug,Clone)]
struct Types{
    edge:E,
    p:i32
}


//disjoint set
#[derive(Debug,Clone)]
struct DisjointSet{
    set:Vec<isize>,
    size:usize
}
//disjoint set impl
impl DisjointSet{
    fn new(_size:usize)->DisjointSet{
        DisjointSet{
            set:vec![-1;_size],
            size:_size
        }
    }
    fn union_set(&mut self, i:usize,j:usize){
        if i!=j{
            if self.set[i]<self.set[j]{
                self.set[i]+=self.set[j];
                self.set[j]= i as isize;
            }
            else {
                self.set[j]+=self.set[i];
                self.set[i]=j as isize;
            }
        }
    }
    fn find(&mut self, mut i:isize) ->isize{
        //set[i] is root of the tree
        let mut temp:isize=self.set[usize::try_from(i.clone()).unwrap()];
        while i>=0{
            temp=i.clone();
            i=self.set[usize::try_from(i).unwrap()];
            };
        return isize::try_from(temp).unwrap();
    }
}


//directed graph
#[derive(Debug,Clone)]
struct DirectedGraph<T>{
    adj_list :Vec<Box<V<T>>>,
    edges: Vec<E>,
    size:usize
}
impl<T> DirectedGraph<T>
    where T: std::fmt::Debug,
          T: std::clone::Clone {
    
    fn construct_adj_list(_adj_list: &Vec<Box<V<T>>>, _edges: &Vec<E>, _size:
    usize) -> Vec<Box<V<T>>> {
        let mut return_val: Vec<Box<V<T>>> = {
            _adj_list.iter().map(|adj| Box::new(V {
                element: adj.element.clone(),
                num_visited: adj.num_visited.clone(),
                in_degree: adj.in_degree.clone(),
                adj: Default::default()
            })).collect()
        };
        for edge in _edges.iter() {
            return_val[edge.src].adj.push_back(edge.dest);
            return_val[edge.dest].in_degree += 1;
        }
        return_val
    }
    fn construct_edges(edges: &Vec<E>) -> Vec<E> {
        edges.iter().map(|n| E {
            src: n.src,
            dest: n.dest,
            weight: n.weight
        }).collect()
    }
    fn new(_adj: &Vec<Box<V<T>>>, _edges: &Vec<E>, size: usize) -> Self {
        let adj_list = Self::construct_adj_list(_adj, _edges, size);
        let edges = Self::construct_edges(&_edges);
        Self { adj_list, edges, size }
    }
    
    fn print(self){
        println!("{}","Edge  :  Weight");
        for edge in self.edges.iter(){
            println!("{} - {} : {}",edge.src,edge.dest,edge.weight);
        }
    
    }
    
    fn check_cycle(&mut self) -> bool {
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
                self._dfs(it.clone());
            }
        }
    }
    fn dfs(&mut self) {
        for mut i in self.adj_list.to_owned() {
            i.num_visited = 0;
        }
        for (i, adj) in self.adj_list.to_owned().iter().enumerate(){
            if adj.num_visited == 0 {
                self._dfs(i);
            }
        }
    }
    fn kruskal(&mut self) {
        let mut dis_sets: DisjointSet = DisjointSet::new(self.size);
        let mut u_set: usize;
        let mut v_set: usize;
        let mut tmp_edges = vec![];
        let mut edges_accepted: usize = 0;
        let mut priority_queue =
            PriorityQueue::with_capacity(self.size);
        
        for edge in self.edges.iter(){
            priority_queue.push(edge, edge.weight);
        }
        
        while edges_accepted < (self.size - 1)&&!priority_queue.is_empty() {
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
        self.edges=tmp_edges;
    }
    fn reset(&mut self){
        for mut adj in self.adj_list.to_owned(){
            adj.num_visited=0;
        }
    }
}

fn main() {
    let node_elem = vec![23, 11, 15, 22, 7, 21];
    let vertices: Vec<Box<V<u32>>> = {
        node_elem.iter().map(|n| Box::new(V {
            element: *n,
            in_degree: 0,
            num_visited: 0,
            adj: LinkedList::new()
        })).collect()
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
    let mut graph:DirectedGraph<u32> = DirectedGraph::new(&vertices,&edges,size);
    
    println!("circle: {}", graph.check_cycle());
    graph.dfs();
    graph.to_owned().print();
    graph.kruskal();
    graph.print();
   
}