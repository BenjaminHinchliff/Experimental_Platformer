use amethyst::{
    prelude::*,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct KeyEcho;

impl <'s> System<'s> for KeyEcho {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (input,): Self::SystemData) {
        if let Some(vel) = input.axis_value("horizontal_movement") {
            if vel != 0.0 {
                println!("{}", vel);
            }
        }
        if let Some(down) = input.action_is_down("jump") {
            if down {
                println!("{}", down)
            }
        }
    }
}