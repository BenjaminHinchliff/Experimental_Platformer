use amethyst::{
    prelude::*,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct Gravity;

impl <'s> System<'s> for Gravity {
    type SystemData = (
        ReadStorage<'s, Transform>,
    );

    fn run(&mut self, (locals,): Self::SystemData) {
        for local in locals.join() {
            println!("{:?}", local);
        }
    }
}