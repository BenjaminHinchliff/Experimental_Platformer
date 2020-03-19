use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
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