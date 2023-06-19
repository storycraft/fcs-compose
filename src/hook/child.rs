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
    use_ref(ctx, move || Component::new(component_fn))
}

pub type ChildrenRef<'a, F> = Ref<'a, Vec<Component<F>>>;

pub fn use_children<'a, F: FnMut(&mut ComponentContext) -> R + Any, R>(
    ctx: &mut ComponentContext<'a, '_>,
) -> ChildrenRef<'a, F> {
    use_ref(ctx, Vec::<Component<F>>::new)
}
