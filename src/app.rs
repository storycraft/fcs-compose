/*
 * Created on Thu Jun 15 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    ops::Deref,
    ptr::{self, addr_of_mut},
    task::Poll,
};

use async_stream::stream;
use bumpalo::{boxed::Box as BumpBox, Bump};
use futures::{future::poll_fn, Stream};
use rustc_hash::FxHashMap;
use scoped_tls::scoped_thread_local;

use crate::component::{Component, ComponentContext};

scoped_thread_local!(static APP_CONTEXT: App);

#[derive(Debug)]
pub struct App {
    system_map: RefCell<FxHashMap<TypeId, BumpBox<'static, dyn Any>>>,

    // Always drop after refs
    arena: Bump,
}

impl App {
    pub fn new() -> Self {
        Self {
            system_map: RefCell::new(FxHashMap::default()),
            arena: Bump::new(),
        }
    }

    pub fn run<R>(self, root_fn: impl FnMut(&mut ComponentContext) -> R) -> impl Stream<Item = R> {
        if APP_CONTEXT.is_set() {
            panic!("Cannot run another App inside App scope");
        }

        let mut root = Component::new(root_fn);

        stream! {
            loop {
                yield poll_fn(|context| {
                    Poll::Ready(APP_CONTEXT.set(&self, || {
                        root.update(context)
                    }))
                }).await;

                futures::pending!();
            }
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

pub fn use_system<'a, T: Any>(
    _ctx: &ComponentContext<'a, '_>,
    initializer: impl FnOnce() -> T + Any,
) -> System<'a, T> {
    if !APP_CONTEXT.is_set() {
        panic!("Cannot use outside of App scope");
    }

    APP_CONTEXT.with(|app| {
        let mut system_map = app.system_map.borrow_mut();

        let system = system_map.entry(initializer.type_id()).or_insert_with(|| {
            let value = app.arena.alloc(initializer()) as &mut dyn Any;

            // SAFETY: value is allocated in self-contained arena and has correct drop order
            unsafe { BumpBox::from_raw(addr_of_mut!(*value)) }
        });

        // SAFETY: system is allocated on heap and lifetime 'a is contained inside app scope
        System(unsafe { &*ptr::addr_of!(*system.downcast_ref::<T>().unwrap()) })
    })
}

#[derive(Debug)]
pub struct System<'a, T: ?Sized>(&'a T);

impl<T: ?Sized> Deref for System<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
