/*******************************************************************************
Title           : vertices.rs
Author          : Keisuke Suzuki
Created on      : 9/21/19
Modification    :
*******************************************************************************/
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub struct V<T> {
    pub(crate) element: T,
    pub(crate) in_degree: u32,
    pub(crate) num_visited: u32,
    pub(crate) adj: LinkedList<usize>
}

impl<T> V<T> {
    pub fn new(elem: T) -> V<T> {
        V {
            element: elem,
            in_degree: 0,
            num_visited: 0,
            adj: LinkedList::new()
        }
    }
    pub fn with(element: T, in_degree: u32, num_visited: u32, adj:
    LinkedList<usize>) -> V<T> {
        V { element, in_degree, num_visited, adj }
    }
}