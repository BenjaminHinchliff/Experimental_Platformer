use amethyst::{
    prelude::*,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::platformer::{Tile, MOVE_SPEED};

#[derive(SystemDesc)]
pub struct Scroll;

impl <'s> System<'s> for Scroll {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Tile>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (input, tiles, mut transforms): Self::SystemData) {
        if let Some(vel) = input.axis_value("horizontal_movement") {
            for (tile, transforms) in (&tiles, &mut transforms).join() {
                transforms.prepend_translation_x(vel * -MOVE_SPEED);
            }
        }
    }
}