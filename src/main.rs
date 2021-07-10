//! Pong Tutorial 1

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
extern crate amethyst;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::audio::AudioBundle;
use amethyst::audio::DjSystemDesc;
use crate::audio::Music;


mod pong;
mod systems;
mod audio;
mod pause_state;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {

    // start amethyst logger
    amethyst::start_logger(Default::default());

    // prepare display configurations
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // prepare input handler
    let binding_path = app_root.join("config").join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(
        binding_path
    )?;

    // application setup
    let game_data = GameDataBuilder::default()
    .with_bundle(
        RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?
            .with_clear(
                [0.0, 0.0, 0.0, 1.0]
            ),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default())
    )?
    .with_bundle(TransformBundle::new())?
    .with_bundle(input_bundle)?
    .with_bundle(UiBundle::<StringBindings>::new())?
    .with_bundle(AudioBundle::default())?
    .with_system_desc(
        DjSystemDesc::new(|music: &mut Music| music.music.next()), 
        "dj_system", 
        &[],
    )
    .with(
        systems::PaddleSystem, "paddle_system", &["input_system"]
    )
    .with(systems::MoveBallSystem, "ball_system", &[])
    .with(
        systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"]
    )
    .with(
            systems::WinnerSystem, "winner_system", &["collision_system"]
    );

    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();

    Ok(())
}
