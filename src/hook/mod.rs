/*
 * Created on Fri Jun 16 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

pub mod container;
pub mod state;
pub mod stream;

use std::{
    any::Any,
    ops::{Deref, DerefMut}, pin::Pin,
};

use crate::component::ComponentContext;

pub fn use_ref<'a, T: Any>(
    ctx: &mut ComponentContext<'a, '_>,
    initializer: impl FnOnce() -> T,
) -> Ref<'a, T> {
    Ref(ctx.next_ref(initializer))
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Ref<'a, T>(&'a mut T);

impl<T> Ref<'_, T> {
    pub fn pin_mut(this: &mut Self) -> Pin<&mut T> {
        // SAFETY: value is allocated on heap and has drop guarantee
        unsafe {
            Pin::new_unchecked(&mut *this)
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T> DerefMut for Ref<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

pub fn use_cleanup<C: Cleanup + Any>(ctx: &mut ComponentContext, cleanup: C) {
    ctx.next_ref(|| CleanupCell(cleanup));
}

pub fn use_once<C: Cleanup + Any>(ctx: &mut ComponentContext, initializer: impl FnOnce() -> C) {
    ctx.next_ref(|| CleanupCell(initializer()));
}

#[derive(Debug)]
#[repr(transparent)]
struct CleanupCell<T: Cleanup>(T);

impl<T: Cleanup> CleanupCell<T> {
    pub const fn new(cleanup: T) -> Self {
        Self(cleanup)
    }

    pub fn set(&mut self, cleanup: T) {
        self.0.cleanup();
        self.0 = cleanup;
    }
}

impl<T: Cleanup> Drop for CleanupCell<T> {
    fn drop(&mut self) {
        self.0.cleanup();
    }
}

pub trait Cleanup {
    fn cleanup(&mut self);
}

impl<T: Fn() -> R, R> Cleanup for T {
    fn cleanup(&mut self) {
        (self)();
    }
}

impl Cleanup for () {
    fn cleanup(&mut self) {}
}
