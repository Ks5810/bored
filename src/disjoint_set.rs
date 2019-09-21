/*******************************************************************************
Title           : disjoint_set.rs
Author          : Keisuke Suzuki
Created on      : 9/21/19
Modification    :
*******************************************************************************/
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct DisjointSet {
    set: Vec<isize>,
    size: usize
}

//disjoint set impl
impl DisjointSet {
    pub fn new(_size: usize) -> DisjointSet {
        DisjointSet {
            set: vec![-1; _size],
            size: _size
        }
    }
    pub fn union_set(&mut self, i: usize, j: usize) {
        if i != j {
            if self.set[i] < self.set[j] {
                self.set[i] += self.set[j];
                self.set[j] = i as isize;
            } else {
                self.set[j] += self.set[i];
                self.set[i] = j as isize;
            }
        }
    }
    pub fn find(&mut self, mut i: isize) -> isize {
        //set[i] is root of the tree
        let mut temp: isize = self.set[usize::try_from(i.clone()).unwrap()];
        while i >= 0 {
            temp = i.clone();
            i = self.set[usize::try_from(i).unwrap()];
        };
        return isize::try_from(temp).unwrap();
    }
}