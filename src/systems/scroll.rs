use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::platformer::{Ball, MOVE_SPEED};

#[derive(SystemDesc)]
pub struct Scroll;

impl <'s> System<'s> for Scroll {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Ball>
    );

    fn run(&mut self, (input, mut balls): Self::SystemData) {
        if let Some(vel) = input.axis_value("horizontal_movement") {
            for ball in (&mut balls).join() {
                ball.xvel = vel * -MOVE_SPEED;
            }
        }
    }
}