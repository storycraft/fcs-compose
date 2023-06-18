/*
 * Created on Sun Jun 18 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::any::Any;

use crate::component::{Component, ComponentContext};

use super::{use_ref, Ref};

pub type ChildRef<'a, F> = Ref<'a, Component<F>>;

pub fn use_child<'a, F: FnMut(&mut ComponentContext) -> R + Any, R>(
    ctx: &mut ComponentContext<'a, '_>,
    component_fn: F,
) -> ChildRef<'a, F> {
    let mut child = use_ref(ctx, move || Component::new(component_fn));
    child.update(ctx.executor_context());

    child
}

pub type OptionChildRef<'a, F> = Ref<'a, Option<Component<F>>>;

pub fn use_child_option<'a, F: FnMut(&mut ComponentContext) -> R + Any, R>(
    ctx: &mut ComponentContext<'a, '_>,
    component_fn: Option<F>,
) -> OptionChildRef<'a, F> {
    let mut child = use_ref(ctx, move || component_fn.map(Component::new));
    if let Some(child) = child.as_mut() {
        child.update(ctx.executor_context());
    }

    child
}

pub type ChildrenRef<'a, F> = Ref<'a, Vec<Component<F>>>;

pub fn use_children<'a, F: FnMut(&mut ComponentContext) -> R + Any, R>(
    ctx: &mut ComponentContext<'a, '_>,
) -> ChildrenRef<'a, F> {
    let mut children = use_ref(ctx, Vec::<Component<F>>::new);
    for child in children.iter_mut() {
        child.update(ctx.executor_context());
    }

    children
}
