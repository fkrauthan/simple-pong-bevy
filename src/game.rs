use crate::audio::initialise_audio;
use crate::utils::{ScaleType, VirtualScreen};
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::core::{Time, Timer};
use bevy::ecs::{Commands, Query, Res, ResMut};
use bevy::math::Size;
use bevy::prelude::{
    BuildChildren, Camera2dBundle, CameraUiBundle, Entity, HorizontalAlign, NodeBundle, TextBundle,
    Transform, Vec2, Vec3, VerticalAlign,
};
use bevy::render::color::Color;
use bevy::sprite::entity::SpriteSheetBundle;
use bevy::sprite::{ColorMaterial, TextureAtlas, TextureAtlasSprite};
use bevy::text::{TextAlignment, TextStyle};
use bevy::ui::widget::Text;
use bevy::ui::{AlignItems, JustifyContent, Style, Val};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PADDLE_HEIGHT: f32 = 16.0;
pub const PADDLE_WIDTH: f32 = 4.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

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

pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

pub struct SpriteSheet {
    pub handle: Handle<TextureAtlas>,
}

pub struct StartTimer {
    pub timer: Timer,
}
impl StartTimer {
    fn new(seconds_to_start: f32) -> StartTimer {
        StartTimer {
            timer: Timer::from_seconds(seconds_to_start, false),
        }
    }
}

pub fn init_game(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_atlas_handle = load_sprite_sheet(&asset_server, &mut texture_atlases);
    commands.insert_resource(SpriteSheet {
        handle: texture_atlas_handle.clone(),
    });

    commands.insert_resource(ScoreBoard {
        score_left: 0,
        score_right: 0,
    });

    commands.spawn((StartTimer::new(1.0),));

    initialise_camera(commands);
    initialise_paddles(commands, &texture_atlas_handle);

    initialise_scoreboard(commands, &asset_server, &mut materials);
    initialise_audio(commands, &asset_server);
}

pub fn start_game_system(
    commands: &mut Commands,
    time: Res<Time>,
    sprite_sheet: Res<SpriteSheet>,
    mut query: Query<(Entity, &mut StartTimer)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        if timer.timer.tick(time.delta_seconds()).just_finished() {
            commands.despawn(entity);
            initialise_ball(commands, &sprite_sheet.handle);
        }
    }
}

fn load_sprite_sheet(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    // Add Sprite sheet
    let texture_handle = asset_server.load("texture/pong_spritesheet.png");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(8.0, 16.0));

    texture_atlas.textures.push(bevy::sprite::Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(4.0, 16.0),
    });
    texture_atlas.textures.push(bevy::sprite::Rect {
        min: Vec2::new(4.0, 0.0),
        max: Vec2::new(8.0, 4.0),
    });
    return texture_atlases.add(texture_atlas);
}

fn initialise_camera(commands: &mut Commands) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    commands
        .spawn(Camera2dBundle {
            transform: Transform {
                translation: Vec3::new(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with(VirtualScreen {
            width: ARENA_WIDTH,
            height: ARENA_HEIGHT,
            scale_type: ScaleType::Stretch,
        })
        .spawn(CameraUiBundle::default());
}

fn initialise_paddles(commands: &mut Commands, texture_atlas_handle: &Handle<TextureAtlas>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    let y = ARENA_HEIGHT / 2.0;
    left_transform.translation = Vec3::new(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.translation = Vec3::new(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    let sprite_render_left = SpriteSheetBundle {
        texture_atlas: (*texture_atlas_handle).clone(),
        ..Default::default()
    };
    let sprite_render_right = SpriteSheetBundle {
        texture_atlas: (*texture_atlas_handle).clone(),
        ..Default::default()
    };

    // Create a left plank entity.
    commands
        .spawn(sprite_render_left)
        .with(left_transform)
        .with(Paddle::new(Side::Left));

    commands
        .spawn(sprite_render_right)
        .with(right_transform)
        .with(Paddle::new(Side::Right));
}

fn initialise_ball(commands: &mut Commands, texture_atlas_handle: &Handle<TextureAtlas>) {
    // Create the translation.
    let mut local_transform = Transform::default();
    local_transform.translation = Vec3::new(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the ball. The ball is the second sprite in the sheet.
    let sprite_render = SpriteSheetBundle {
        texture_atlas: (*texture_atlas_handle).clone(),
        sprite: TextureAtlasSprite {
            index: 1,
            ..Default::default()
        },
        ..Default::default()
    };

    commands
        .spawn(sprite_render)
        .with(local_transform)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        });
}

fn initialise_scoreboard(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let font = asset_server.load("font/square.ttf");

    let mut p1_score = None;
    let mut p2_score = None;
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::FlexEnd,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            p1_score = parent
                .spawn(TextBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "0".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .current_entity();

            p2_score = parent
                .spawn(TextBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                        ..Default::default()
                    },
                    text: Text {
                        value: "0".to_string(),
                        font: font.clone(),
                        style: TextStyle {
                            font_size: 50.0,
                            color: Color::rgb(1.0, 1.0, 1.0),
                            alignment: TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Left,
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .current_entity();
        });

    commands.insert_resource(ScoreText {
        p1_score: p1_score.unwrap(),
        p2_score: p2_score.unwrap(),
    });
}
