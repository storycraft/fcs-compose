/*
 * Created on Sun Jun 18 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::any::Any;

use smallvec::SmallVec;

use crate::component::{Component, ComponentContext};

use super::{use_ref, Ref};

#[derive(Debug)]
pub struct Container<'a> {
    index: usize,
    components: Ref<'a, SmallVec<[Component; 1]>>,
}

impl Container<'_> {
    pub fn child<R>(
        &mut self,
        ctx: &mut ComponentContext,
        mut component_fn: impl FnMut(&mut ComponentContext) -> R + Any,
    ) -> &mut Self {
        let component = if self.components.len() <= self.index {
            self.components.push(Component::new());
            self.components.last_mut().unwrap()
        } else {
            &mut self.components[self.index]
        };

        self.index += 1;
        component.update(ctx.executor_context(), &mut component_fn);

        self
    }
}

impl Drop for Container<'_> {
    fn drop(&mut self) {
        if self.components.len() > self.index {
            self.components.resize_with(self.index, || unreachable!());
        }
    }
}

pub fn use_container<'a>(ctx: &mut ComponentContext<'a, '_>) -> Container<'a> {
    Container {
        index: 0,
        components: use_ref(ctx, SmallVec::new),
    }
}
