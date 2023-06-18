/*
 * Created on Sun Jun 18 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */

use std::{any::Any, task::Poll};

use futures::Stream;

use crate::component::ComponentContext;

use super::{use_ref, Ref};

pub fn use_stream<'a, S: Any + Stream>(
    ctx: &mut ComponentContext<'a, '_>,
    initializer: impl FnOnce() -> S,
    mut consumer: impl FnMut(S::Item),
) {
    let mut stream = use_ref(ctx, initializer);

    while let Poll::Ready(Some(item)) = Ref::pin_mut(&mut stream).poll_next(ctx.executor_context())
    {
        (consumer)(item);
    }
}
