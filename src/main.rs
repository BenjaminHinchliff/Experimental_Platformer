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
};

mod platformer;

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

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )? 
        .with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");
    let _world = World::new();
    let mut game = Application::new(assets_dir, Platformer, game_data)?;
    game.run();

    Ok(())
}