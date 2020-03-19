use amethyst::{
    prelude::*,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::platformer::{Ball, JUMP_VELOCITY};

#[derive(SystemDesc)]
pub struct Jump;

impl<'s> System<'s> for Jump {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Ball>,
    );

    fn run(&mut self, (input, mut balls): Self::SystemData) {
        for ball in (&mut balls).join() {
            if let Some(down) = input.action_is_down("jump") {
                if down && ball.on_ground {
                    ball.yvel = JUMP_VELOCITY;
                }
            }
        }
    }
}