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
pub struct Component {
    last_id: usize,
    refs: RefStore,
}

impl Component {
    pub fn new() -> Self {
        Self {
            last_id: 0,
            refs: RefStore::new(),
        }
    }

    pub fn update<R>(
        &mut self,
        context: &mut Context,
        component_fn: &mut impl FnMut(&mut ComponentContext) -> R,
    ) -> R {
        let id = uid(component_fn);

        if self.last_id != id {
            self.refs = RefStore::new();
            self.last_id = id;
        }

        let mut ctx = ComponentContext {
            ref_accesser: self.refs.accesser(),
            context,
        };

        (component_fn)(&mut ctx)
    }
}

fn uid<F: FnMut(&mut ComponentContext) -> R, R>(_: &F) -> usize {
    fn helper<F: FnMut(&mut ComponentContext) -> R, R>(
        dummy: fn(&str),
        mut f: F,
        ctx: &mut ComponentContext,
    ) -> R {
        dummy(std::any::type_name::<F>());
        f(ctx)
    }

    helper::<F, R> as usize
}
