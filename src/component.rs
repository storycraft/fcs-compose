/*
 * Created on Sun Jun 11 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{any::Any, task::Context};

use crate::ref_store::{RefAccesser, RefStore};

#[derive(Debug)]
pub struct ComponentContext<'a, 'ctx> {
    ref_accesser: RefAccesser<'a>,
    context: &'a mut Context<'ctx>,
}

impl<'a, 'ctx> ComponentContext<'a, 'ctx> {
    pub fn executor_context(&mut self) -> &mut Context<'ctx> {
        self.context
    }

    pub fn next_ref<T: Any>(&mut self, initializer: impl FnOnce() -> T) -> &'a mut T {
        self.ref_accesser.next_ref(initializer)
    }
}

#[derive(Debug)]
pub struct Component<F> {
    component_fn: F,
    refs: RefStore,
}

impl<F: FnMut(&mut ComponentContext) -> R, R> Component<F> {
    pub fn new(component_fn: F) -> Self {
        Self {
            component_fn,
            refs: RefStore::new(),
        }
    }

    pub fn update(&mut self, context: &mut Context) -> R {
        let mut ctx = ComponentContext {
            ref_accesser: self.refs.accesser(),
            context,
        };

        (self.component_fn)(&mut ctx)
    }
}
