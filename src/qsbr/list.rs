// Copyright 2015 Jeehoon Kang <jeehoon.kang@sf.snu.ac.kr>
// See LICENSE-APACHE and LICENSE-MIT file for more information.

use std::sync::atomic::{AtomicPtr, Ordering};
use core::marker::PhantomData;

struct Node<'a, T: Send + 'a> {
    item: T,
    next: AtomicPtr<Node<'a, T>>,
    _marker: PhantomData<&'a T>,
}

struct NodeIter<'a, T: Send + 'a> {
    node: &'a Node<'a, T>,
}

impl<'a, T: Send> Iterator for NodeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let ptr = self.node.next.load(Ordering::Acquire);
        if ptr.is_null() {
            None
        }
        else {
            let result = &self.node.item;
            self.node = unsafe { &*ptr };
            Some(result)
        }
    }
}

pub struct Reader<T: Send> {
    _marker: PhantomData<T> // TODO: remove
}

impl<T:Send> Clone for Reader<T> {
    fn clone(&self) -> Self {
        Reader { _marker: PhantomData }
    }
}

impl<T: Send> Reader<T> {
    fn quiescent_state() {
        unimplemented!()
    }

    fn iter(&self) -> NodeIter<'a, T> {
        unimplemented!()
    }
}

pub struct Writer<T: Send> {
    head: Node<'static, T>,
}

impl<T: Send> Writer<T> {
    fn sync() {
        unimplemented!()
    }

    fn iter(&self) -> NodeIter<'a, T> {
        unimplemented!()
    }

    fn insert(&self, iter: NodeIter<'a, T>, val:T) {
        unimplemented!()
    }

    fn update(&self, iter: NodeIter<'a, T>, val:T) {
        unimplemented!()
    }

    fn delete(&self, iter: NodeIter<'a, T>) {
        unimplemented!()
    }
}

pub fn create<T>() -> (Reader<T>, Writer<T>) {
    unimplemented!()
}
