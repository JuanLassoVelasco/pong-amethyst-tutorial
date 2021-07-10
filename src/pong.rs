pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;
pub const BALL_VELOCITY_X: f32 = 55.0;
pub const BALL_VELOCITY_Y: f32 = 30.0;
pub const BALL_RADIUS: f32 = 2.0;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    core::timing::Time,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ecs::{Entity},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
    input::{is_close_requested, VirtualKeyCode, is_key_down},
};

use crate::audio::initialize_audio;
use crate::pause_state::PauseState;

#[derive(PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Paddle {
        Paddle {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Pong {
    ball_spawn_timer: Option<f32>,
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

impl SimpleState for Pong {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // load spritesheet necessary for graphics
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        // set ball timer
        self.ball_spawn_timer.replace(1.0);

        initialize_camera(world);

        initialize_paddles(world, self.sprite_sheet_handle.clone().unwrap());

        initialize_scoreboard(world);

        initialize_audio(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {

        if let Some(mut timer) = self.ball_spawn_timer.take() {
            {
            let time = data.world.fetch::<Time>();
            timer -= time.delta_seconds();
            }
            if timer <= 0.0 {
                initialize_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                self.ball_spawn_timer.replace(timer);
            }
        }
        

        Trans::None
    }

    fn handle_event(&mut self, mut _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            } else if is_key_down(&event, VirtualKeyCode::P) {
                return Trans::Push(Box::new(PauseState::default()));
            }
        }

        Trans::None
    }

    fn on_resume(&mut self, mut _data: StateData<'_, GameData<'_, '_>>) {
        println!("Game Resumed");
    }
}

fn initialize_camera(world: &mut World) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world.create_entity().with(
        Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT)
    ).with(transform).build();

}

fn initialize_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    // create transform components
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // Assign sprites for the paddles
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    // set correct paddle positions
    let y = ARENA_HEIGHT / 2.0;
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - (PADDLE_WIDTH * 0.5), y, 0.0);

    // Create left paddle entity
    world.create_entity().with(
        sprite_render.clone()
    ).with(Paddle::new(Side::Left)).with(left_transform).build();

    // Create right paddle entity
    world.create_entity().with(
        sprite_render
    ).with(Paddle::new(Side::Right)).with(right_transform).build();
}

fn initialize_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {

    // create ball transform component
    let mut local_transform = Transform::default();

    // set ball sprite
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    // position ball in center
    local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    world.create_entity().with(sprite_render)
    .with(Ball {
        velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        radius: BALL_RADIUS,
    }).with(local_transform).build();
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )

}

fn initialize_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let p1_transform = UiTransform::new(
        "P1".to_string(), Anchor::TopMiddle, Anchor::TopMiddle, 
        -50., -50., 1., 200., 50.,
    );

    let p2_transform = UiTransform::new(
        "P2".to_string(), Anchor::TopMiddle, Anchor::TopMiddle, 
        50., -50., 1., 200., 50.,
    );

    let p1_score = world.create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.,1.,1.,1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let p2_score = world.create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1.,1.,1.,1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    world.insert(ScoreText {p1_score, p2_score});
}
