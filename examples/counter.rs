/*
 * Created on Sun Jun 11 2023
 *
 * Copyright (c) storycraft. Licensed under the Apache Licence 2.0.
 */
use futures::{pin_mut, StreamExt};

use fcs_compose::{
    app::App,
    component::ComponentContext,
    hook::{child::use_child, state::use_state, use_cleanup, use_once},
};

fn main() {
    let stream = App::new().run(app);
    pin_mut!(stream);

    // async friendly
    pollster::block_on(async {
        while let Some(expired) = stream.next().await {
            if expired {
                break;
            }
        }
    });
}

fn app(ctx: &mut ComponentContext) -> bool {
    // composition: compose counter components to parent component
    // inheritance: create new component function using closure to supply prop to counter component function
    let mut counter1 = use_child(ctx, |child_ctx| counter(child_ctx, "Counter 1", &0));
    let mut counter2 = use_child(ctx, |child_ctx| counter(child_ctx, "Counter 2", &1));

    // when mutably borrowed, signal async executor by abusing rust deref coercion
    let mut iter = use_state(ctx, || 0);
    *iter += 1;

    use_once(ctx, || {
        println!("once_hook: app initialized");

        || {
            println!("once_hook: app cleanup");
        }
    });

    // use rust Drop trait to cleanup resources
    use_cleanup(ctx, || {
        println!("App cleanup");
    });

    println!("App update");
    counter1.update(ctx.executor_context());
    counter2.update(ctx.executor_context());

    *iter >= 100
}

fn counter(ctx: &mut ComponentContext, name: &str, props: &i32) {
    let mut num = use_state(ctx, || *props);

    *num += 1;

    println!("{}: {}", name, *num);
}
