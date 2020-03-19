use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::platformer::{Ball, Tile};

#[derive(SystemDesc)]
pub struct MovePlayer;

impl <'s> System<'s> for MovePlayer {
    type SystemData = (
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Tile>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (balls, tiles, mut transforms): Self::SystemData) {
        let mut tile_change = 0.0;
        for (ball, transform) in (&balls, &mut transforms).join() {
            tile_change += ball.xvel;
            transform.prepend_translation_y(ball.yvel);
        }
        for (_, transform) in (&tiles, &mut transforms).join() {
            transform.prepend_translation_x(tile_change);
        }
    }
}