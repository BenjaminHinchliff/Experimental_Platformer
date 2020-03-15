use amethyst::{
    assets::{Handle, Loader, AssetStorage},
    ecs::prelude::*,
    prelude::*,
    core::transform::Transform,
    renderer::{Camera, SpriteSheet, Texture, ImageFormat, SpriteSheetFormat, SpriteRender},
};

pub const VIEWBOX_WIDTH: f32 = 640.0;
pub const VIEWBOX_HEIGHT: f32 = 480.0;

pub const BALL_WIDTH: f32 = 32.0;
pub const BALL_HEIGHT: f32 = 32.0;

pub struct Platformer;

impl SimpleState for Platformer {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        // temp
        world.register::<Ball>();

        init_camera(world);
        init_ball(world, sprite_sheet_handle);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(VIEWBOX_WIDTH / 2.0, VIEWBOX_HEIGHT / 2.0, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(VIEWBOX_WIDTH, VIEWBOX_HEIGHT))
        .with(transform)
        .build();
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

pub struct Ball {
    pub width: f32,
    pub height: f32,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            width: BALL_WIDTH,
            height: BALL_HEIGHT,
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
        sprite_number: 1,
    };

    world.create_entity()
        .with(sprite_render)
        .with(Ball::new())
        .with(transform)
        .build();
}