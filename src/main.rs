use amethyst::{
    prelude::*,
    LoggerConfig,
    LogLevelFilter,
    StdoutLog,
    core::transform::TransformBundle,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    input::{
        InputBundle,
        StringBindings,
    }
};

mod platformer;
mod systems;

use systems::{KeyEcho, Gravity, Collision, MovePlayer, Jump, Scroll};
use platformer::Platformer;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        allow_env_override: false,
        level_filter: LogLevelFilter::Debug,
        log_file: None,
        log_gfx_backend_level: None,
        log_gfx_rendy_level: None,
        module_levels: Vec::default(),
        stdout: StdoutLog::Colored,
    });

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )? 
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(Gravity, "gravity", &[])
        .with(Collision, "collision", &["gravity"])
        .with(Jump, "jump", &["gravity", "collision"])
        .with(MovePlayer, "move_player", &["gravity", "collision", "jump"])
        .with(Scroll, "scroll", &[]);


    let assets_dir = app_root.join("assets");
    let _world = World::new();
    let mut game = Application::new(assets_dir, Platformer, game_data)?;
    game.run();

    Ok(())
}