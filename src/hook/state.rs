/*
 * Created on Sun Jun 18 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{
    any::Any,
    ops::{Deref, DerefMut},
    task::Waker,
};

use crate::component::ComponentContext;

use super::{use_ref, Cleanup, CleanupCell};

pub fn use_state<'a, T: Any>(
    ctx: &mut ComponentContext<'a, '_>,
    initializer: impl FnOnce() -> T,
) -> StateRef<'a, T> {
    let waker = ctx.executor_context().waker();
    let cell = ctx.next_ref(move || StateCell {
        value: initializer(),
        waker: None,
    });

    if !cell
        .waker
        .as_ref()
        .is_some_and(|cell_waker| waker.will_wake(cell_waker))
    {
        cell.waker = Some(waker.clone());
    }
    StateRef(cell)
}

pub trait State {
    fn changed(&self) -> bool;
}

#[derive(Debug)]
#[repr(transparent)]
pub struct StateRef<'a, T>(&'a mut StateCell<T>);

impl<T> State for StateRef<'_, T> {
    fn changed(&self) -> bool {
        self.0.waker.is_none()
    }
}

impl<T> Deref for StateRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.value
    }
}

impl<T> DerefMut for StateRef<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Some(waker) = self.0.waker.take() {
            waker.wake();
        }

        &mut self.0.value
    }
}

#[derive(Debug)]
struct StateCell<T> {
    value: T,
    waker: Option<Waker>,
}

pub fn use_effect<'a, 'deps, C: Cleanup + Any>(
    ctx: &mut ComponentContext<'a, '_>,
    mut effect_fn: impl FnMut() -> C,
    deps: impl IntoIterator<Item = &'deps dyn State>,
) {
    let mut cell = use_ref(ctx, || CleanupCell::new(effect_fn()));

    if deps.into_iter().any(|state| state.changed()) {
        cell.set(effect_fn());
    }
}
