use std::{
    fs,
    path,
};

use amethyst::{
    assets::{Handle, Loader, AssetStorage},
    ecs::prelude::*,
    prelude::*,
    core::transform::Transform,
    renderer::{Camera, SpriteSheet, Texture, ImageFormat, SpriteSheetFormat, SpriteRender},
    utils::application_root_dir,
};

pub const VIEWBOX_WIDTH: f32 = 640.0;
pub const VIEWBOX_HEIGHT: f32 = 480.0;

pub const BALL_WIDTH: f32 = 32.0;
pub const BALL_HEIGHT: f32 = 32.0;

pub const LEVEL_TILE_WIDTH: f32 = 32.0;
pub const LEVEL_TILE_HEIGHT: f32 = 32.0;

pub const GRAVITY_ACCELERATION: f32 = -1.0;
pub const JUMP_VELOCITY: f32 = 10.0;

pub struct Platformer;

impl SimpleState for Platformer {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let app_root = application_root_dir().expect("failed to get app root");
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);
        let level_path = app_root.join("assets").join("levels").join("test.lvl");

        // temp
        world.register::<Ball>();
        world.register::<Tile>();

        init_camera(world);
        init_ball(world, sprite_sheet_handle.clone());
        init_level(world, sprite_sheet_handle, level_path);
        
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage::<Texture>>();
        loader.load(
            "texture/platformer_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/platformer_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(VIEWBOX_WIDTH / 2.0, VIEWBOX_HEIGHT / 2.0, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(VIEWBOX_WIDTH, VIEWBOX_HEIGHT))
        .with(transform)
        .build();
}

pub struct Ball {
    pub width: f32,
    pub height: f32,
    pub xvel: f32,
    pub yvel: f32,
    pub on_ground: bool,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            width: BALL_WIDTH,
            height: BALL_HEIGHT,
            xvel: 0.0,
            yvel: 0.0,
            on_ground: false,
        }
    }
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

fn init_ball(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(VIEWBOX_WIDTH / 2.0,  VIEWBOX_HEIGHT / 2.0, 0.0);


    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: 0,
    };

    world.create_entity()
        .with(sprite_render)
        .with(Ball::new())
        .with(transform)
        .build();
}

#[derive(Clone)]
pub struct Tile {
    pub width: f32,
    pub height: f32,
}


impl Tile {
    pub fn new() -> Tile {
        Tile {
            width: LEVEL_TILE_WIDTH,
            height: LEVEL_TILE_HEIGHT,
        }
    }
}

impl Component for Tile {
    type Storage = VecStorage<Self>;
}

fn init_level(world: &mut World, sprite_sheet: Handle<SpriteSheet>, level_path: path::PathBuf) {
    let tile = Tile::new();

    let mut transform = Transform::default();
    transform.set_translation_xyz(tile.width / 2.0, VIEWBOX_HEIGHT - tile.height / 2.0, 0.0);

    let sprite_render =  SpriteRender {
        sprite_sheet,
        sprite_number: 1,
    };

    let level_data = fs::read_to_string(level_path)
        .expect("Failed to load level file")
        .split("\n")
        .map(|line| 
            line.chars().collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    for row in level_data {
        for col in row {
            if col == 'O' {
                world.create_entity()
                    .with(tile.clone())
                    .with(sprite_render.clone())
                    .with(transform.clone())
                    .build();
            }
            transform.prepend_translation_x(tile.width);
        }
        transform.set_translation_x(tile.width / 2.0);
        transform.prepend_translation_y(-tile.height);
    }
}