/*
 * Created on Sun Jun 11 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{
    any::{Any, TypeId},
    ptr::addr_of_mut,
};

use bumpalo::{boxed::Box as BumpBox, Bump};

#[derive(Debug)]
pub struct RefStore {
    sealed: bool,
    refs: Vec<BumpBox<'static, dyn Any>>,

    // Always drop after refs
    arena: Bump,
}

impl RefStore {
    pub fn new() -> Self {
        Self {
            sealed: false,
            refs: Vec::new(),
            arena: Bump::new(),
        }
    }

    pub const fn sealed(&self) -> bool {
        self.sealed
    }

    pub fn len(&self) -> usize {
        self.refs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }

    pub fn accesser(&mut self) -> RefAccesser {
        let sealed = self.sealed;
        if !self.sealed {
            self.sealed = true;
        }

        RefAccesser {
            arena: &self.arena,
            refs: &mut self.refs,
            sealed,
            index: 0,
        }
    }
}

impl Default for RefStore {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct RefAccesser<'a> {
    arena: &'a Bump,
    refs: &'a mut Vec<BumpBox<'static, dyn Any>>,
    sealed: bool,
    index: usize,
}

impl<'a> RefAccesser<'a> {
    pub const fn next_index(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.refs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.refs.is_empty()
    }

    pub fn next_ref<T: Any>(&mut self, initializer: impl FnOnce() -> T) -> &'a mut T {
        if self.refs.len() <= self.index {
            if self.sealed {
                panic!("Ref count mismatch. Ref cannot be used conditionally");
            } else {
                self.refs.push({
                    let value = self.arena.alloc(initializer()) as &mut dyn Any;

                    // SAFETY: value is allocated in self-contained arena and has correct drop order
                    unsafe { BumpBox::from_raw(addr_of_mut!(*value)) }
                });
            }
        }

        let value_ref = match self.refs[self.index].downcast_mut::<T>() {
            // SAFETY: Each refs are only unique reference to [`RefAccessor`]
            Some(boxed) => unsafe { &mut *(boxed as *mut _) },

            _ => panic!(
                "Accessing with incorrect type. Actual type: {:?} requested type: {:?}",
                self.refs[self.index].type_id(),
                TypeId::of::<T>()
            ),
        };

        self.index += 1;

        value_ref
    }
}

impl Drop for RefAccesser<'_> {
    fn drop(&mut self) {
        if self.sealed && self.refs.len() != self.index {
            panic!(
                "Ref visited less than it should be. count: {} visited: {}",
                self.len(),
                self.index
            );
        }
    }
}
