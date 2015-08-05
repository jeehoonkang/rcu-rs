// Copyright 2015 Jeehoon Kang <jeehoon.kang@sf.snu.ac.kr>
// See LICENSE-APACHE and LICENSE-MIT file for more information.

use core::mem;
use core::nonzero::NonZero;
use std::collections::hash_set::HashSet;
use std::ptr;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};
use std::sync::atomic;

struct Node<'a, T: Sync + 'a> {
    item: T,
    next: AtomicPtr<Node<'a, T>>,
}

pub struct NodeIter<'a, T: Sync + 'a> {
    ptr: NonZero<*mut Node<'a, T>>,
}

impl<'a, T: Sync> Iterator for NodeIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let p = unsafe { (**(self.ptr)).next.load(Ordering::Acquire) };
        if p.is_null() {
            None
        }
        else {
            unsafe {
            self.ptr = NonZero::new(p);
                Some(&(**(self.ptr)).item)
            }
        }
    }
}

pub struct Reader<'a, T: Sync + 'a> {
    writer: NonZero<*mut Writer<'a, T>>,
    counter: AtomicUsize,
}

impl<'a, T: Sync + 'a> Clone for Reader<'a, T> {
    fn clone(&self) -> Self {
        unsafe { (**(self.writer)).create_reader() }
    }
}

impl<'a, T: Sync + 'a> Reader<'a, T> {
    pub fn quiescent_state(&mut self) {
        let wc = unsafe { (**(self.writer)).counter.load(Ordering::Acquire) };
        self.counter.store(wc, Ordering::Release);
        atomic::fence(Ordering::SeqCst);
    }

    pub fn iter(&self) -> NodeIter<'a, T> {
        NodeIter { ptr: unsafe { NonZero::new(&mut (**(self.writer)).head) } }
    }
}

pub struct Writer<'a, T: Sync + 'a> {
    head: Node<'a, T>,
    readers: Mutex<HashSet<NonZero<*const Reader<'a, T>>>>,
    counter: AtomicUsize,
}

impl<'a, T: Sync + 'a> Writer<'a, T> {
    fn create_reader(&mut self) -> Reader<'a, T> {
        let reader = {
            let self_ptr = unsafe { NonZero::new(self as *mut _) };
            let mut readers = self.readers.lock().ok().expect("RCU's internal mutex shall not be poisoned!");
            let reader = Reader {
                writer: self_ptr,
                counter: AtomicUsize::new(self.counter.load(Ordering::Acquire))
            };
            readers.insert(unsafe { NonZero::new(&reader) });
            reader
        };
        atomic::fence(Ordering::SeqCst);
        reader
    }

    pub fn synchronize(&mut self) {
        let wc = 1 + self.counter.load(Ordering::Acquire);
        self.counter.store(wc, Ordering::Release);
        let readers = self.readers.lock().ok().expect("RCU's internal mutex shall not be poisoned!");
        for reader in readers.iter() {
            while unsafe { (***reader).counter.load(Ordering::Acquire) } != wc {
                // TODO: "pause" instruction here?
            }
        }
        atomic::fence(Ordering::SeqCst);
    }

    pub fn iter(&'a mut self) -> NodeIter<T> {
        NodeIter { ptr: unsafe { NonZero::new(&mut self.head) } }
    }

    pub fn insert(&self, iter: NodeIter<T>, val:T) {
        unimplemented!()
    }

    pub fn update(&self, iter: NodeIter<T>, val:T) {
        unimplemented!()
    }

    pub fn delete(&self, iter: NodeIter<T>) {
        unimplemented!()
    }
}

pub fn create<'a, T: Sync + 'a>() -> Writer<'a, T> {
    Writer {
        head: Node {
            item: unsafe { mem::uninitialized() },
            next: AtomicPtr::new(ptr::null_mut()),
        },
        readers: Mutex::new(HashSet::new()),
        counter: AtomicUsize::new(1)
    }
}
