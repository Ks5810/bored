/*******************************************************************************
Title           : main.rs
Author          : Keisuke Suzuki
Created on      : Aug 27, 2019
****************************************************************************/

use std::collections::LinkedList;
use std::vec::Vec;

//vertices
struct V{
    element : u32,
    in_degree: u32,
    num_visited: i32,
    adj: LinkedList<u32>,
}

//edge
#[derive(Debug, Copy, Clone)]
struct E{
    src: u32,
    dest: u32,
    weight: u32
}
impl E{
    fn new(src:u32, dest:u32, weight:u32)->E{
        E{src,dest,weight}
    }
}

//PriorityQ
#[derive(Debug, Clone)]
struct PriorityQueue{
    heap: Vec<E>,
    size: usize,
}
impl PriorityQueue{
    fn new(empty:Vec<E>)->PriorityQueue {
        PriorityQueue {
            heap: empty,
            size: 0
        }
    }
    fn set(&mut self, edges:Vec<E>){
        self.heap.resize(edges.len()+1,E::new(0,0,0));
        for edge in edges{
            self.insert(edge);
        }
    }
    fn insert(&mut self, edge:E){
        self.size+=1;
        let mut hole=self.size;
        
        //percolate up while element is smaller than parent
        while hole>1&&edge.weight<self.heap[hole/2].weight {
            self.heap[hole]=self.heap[hole/2];
            hole=hole/2;
        }
        self.heap[hole]=edge;
    }
    fn percolate_down(&mut self, hole:usize){
        let mut child:usize;
        let mut hole=hole;
        let temp=self.heap[hole];
        
        while 2*hole<=self.size {
            child=hole*2;
            
            //there is right child and smaller than left child
            if child!=self.size&&self.heap[child+1].weight<self.heap[child].weight {
                child+=1;
            }
            //if current item is smaller than child
            if self.heap[child].weight < temp.weight {
                self.heap[hole]=self.heap[child];
            } else {
                break
            }
            hole=child;
        }
        self.heap[hole]=temp;
    }
    fn pop(&mut self){
        if self.size!=1{
            self.heap[1]=self.heap[self.size];
        }
        panic!("underflow!");
    }
    fn front(self)->E{
        self.heap[1]
    }
    fn print(self){
        for i in 1..self.size+1{
            print!("{}, ", self.heap[i].weight);
        }
    }
}

fn main() {
    println!("I'll implement this later!")
}