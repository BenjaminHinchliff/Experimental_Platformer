use amethyst::{
    prelude::*,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::platformer::{Ball, GRAVITY_ACCELERATION};

#[derive(SystemDesc)]
pub struct Gravity;

impl <'s> System<'s> for Gravity {
    type SystemData = (
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (mut balls, transforms): Self::SystemData) {
        for (ball, _transform) in (&mut balls, &transforms).join() {
            ball.yvel += GRAVITY_ACCELERATION;
        }
    }
}