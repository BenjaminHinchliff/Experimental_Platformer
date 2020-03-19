use std::collections::HashMap;

use amethyst::{
    core::{Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage, Entities},
};

use crate::platformer::{Ball, Tile};

#[derive(SystemDesc)]
pub struct Collision;

impl <'s> System<'s> for Collision {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Tile>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (entities, tiles, mut balls, mut transforms): Self::SystemData) {
        let mut y_intersect_amount: HashMap<u32, f32> = HashMap::new();
        for (entity, ball, ball_transform) in (&*entities, &mut balls, &transforms).join() {
            ball.on_ground = false;
            let ball_x = ball_transform.translation().x;
            let ball_y = ball_transform.translation().y;
            let half_ball_width = ball.width / 2.0;
            let half_ball_height = ball.height / 2.0;
        for (tile, tiles_transform) in (&tiles, &transforms).join() {
                let tile_x = tiles_transform.translation().x;
                let tile_y = tiles_transform.translation().y;
                let half_tile_width = tile.width / 2.0;
                let half_tile_height = tile.height / 2.0;
                if tile_x + half_tile_width > ball_x - half_ball_width
                && tile_x - half_ball_width < ball_x + half_ball_width
                && tile_y - half_tile_height < ball_y + half_ball_height
                && tile_y + half_ball_height > ball_y - half_tile_height {
                    ball.yvel = 0.0;
                    y_intersect_amount.insert(entity.id(), (tile_y + half_tile_height) - (ball_y - half_ball_height));
                    ball.on_ground = true;
                }
            }
        }
        for (ent, _, ball_transform) in (&*entities, &balls, &mut transforms).join() {
            if let Some(amnt) = y_intersect_amount.get(&ent.id()) {
                if amnt < &20.0 {
                    ball_transform.prepend_translation_y(amnt - 1.0);
                }
            }
        }
    }
}