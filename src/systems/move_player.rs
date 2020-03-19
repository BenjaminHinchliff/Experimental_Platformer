use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::platformer::Ball;

#[derive(SystemDesc)]
pub struct MovePlayer;

impl <'s> System<'s> for MovePlayer {
    type SystemData = (
        ReadStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (balls, mut transforms): Self::SystemData) {
        for (ball, transform) in (&balls, &mut transforms).join() {
            transform.prepend_translation_y(ball.yvel);
        }
    }
}