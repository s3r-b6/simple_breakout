//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
//use std::env;

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Lifes(i8);

struct LostLifeEvent();
struct LostGameEvent();

#[derive(Component)]
//Up, right, down, left
struct Collision(bool, bool, bool, bool);

#[derive(Component)]
struct ScoreBoard();

#[derive(Component)]
struct Score(i16);

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugin(LogDiagnosticsPlugin::default())
        //.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_event::<LostLifeEvent>()
        .add_event::<LostGameEvent>()
        .add_startup_system(setup)
        .add_system(update_scoreboard)
        .add_system(ball_movement)
        .add_system(player_controller)
        .add_system(check_ball_collision)
        .add_system(player_loses_life)
        .add_system(lost_game_handler)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        TextBundle::from_sections([
            //Score
            TextSection::new(
                "Score: ",
                TextStyle {
                    font: asset_server.load("font.ttf"),
                    font_size: 45.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("font.ttf"),
                font_size: 45.0,
                color: Color::WHITE,
            }),
            //Lifes
            TextSection::new(
                "\nLifes: ",
                TextStyle {
                    font: asset_server.load("font.ttf"),
                    font_size: 45.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("font.ttf"),
                font_size: 45.0,
                color: Color::WHITE,
            }),
        ])
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(5.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ScoreBoard(),
    ));
    commands.spawn((
        //ball
        SpriteBundle {
            texture: asset_server.load("circle.png"),
            transform: Transform {
                translation: Vec3::ZERO,
                scale: Vec3::new(0.05, 0.05, 1.),
                ..default()
            },
            ..default()
        },
        Velocity(0., 0.),
        Ball,
    ));
    // commands.spawn((
    //     SpriteBundle {
    //         texture: asset_server.load("a.png"),
    //         transform: Transform {
    //             translation: Vec3::new(0., -320., 1.),
    //             scale: Vec3::new(0.75, 0.3, 1.0),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Block,
    // ));
    commands.spawn((
        SpriteBundle {
            //player
            texture: asset_server.load("rect.png"),
            transform: Transform {
                translation: Vec3::new(0., -320., 1.),
                scale: Vec3::new(0.75, 0.3, 1.0),
                ..default()
            },
            ..default()
        },
        Velocity(0., 0.),
        Collision(false, false, false, false),
        Score(0),
        Lifes(3),
        Player,
    ));
}

fn check_ball_collision(
    mut ball_query: Query<(&Ball, &mut Transform, &mut Velocity), Without<Player>>,
    player_query: Query<(&Player, &Transform, &Velocity), Without<Ball>>,
) {
    let player_pos = player_query.single().1;
    let player_vel = player_query.single().2;
    for (_, ball_pos, mut ball_dir) in &mut ball_query {
        if (player_pos.translation.x - ball_pos.translation.x).abs() < 160.
            && (player_pos.translation.y - ball_pos.translation.y).abs() < 45.
        {
            ball_dir.0 = ball_dir.0 * -1.2 + player_vel.0;
            ball_dir.1 = ball_dir.1 * -1.1;
        }
    }
}

fn update_scoreboard(
    player_query: Query<(&Score, &Lifes), With<Player>>,
    mut scoreboard_query: Query<&mut Text, With<ScoreBoard>>,
) {
    let (score, lifes) = player_query.single();
    for mut text in &mut scoreboard_query {
        text.sections[1].value = format!("{:?}", score.0);
        text.sections[3].value = format!("{:?}", lifes.0);
    }
}

fn ball_movement(
    mut lost_life: EventWriter<LostLifeEvent>,
    mut ball_query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    for (mut transform, mut curr_dir) in &mut ball_query {
        if curr_dir.0 == 0. && curr_dir.1 == 0. {
            curr_dir.0 = 0.7;
            curr_dir.1 = 1.5;
        } else if curr_dir.1 == 0. {
            curr_dir.1 = 0.2;
        }
        //si choca con una pared
        if transform.translation.x > 449. && curr_dir.0 > 0. {
            curr_dir.0 = curr_dir.0 * -0.9;
        } else if transform.translation.x < -450. && curr_dir.0 < 0. {
            curr_dir.0 = 0. * -0.9;
        } else if transform.translation.y > 349. && curr_dir.1 > 0. {
            curr_dir.1 = curr_dir.1 * -0.9;
        }
        if transform.translation.y < -350. && curr_dir.1 < 0. {
            lost_life.send(LostLifeEvent());
            transform.translation = Vec3::ZERO;
            curr_dir.0 = 0.;
            curr_dir.1 = 0.;
        } else {
            transform.translation.x += curr_dir.0;
            transform.translation.y += curr_dir.1;
        }
    }
}

fn player_loses_life(
    mut lost_game: EventWriter<LostGameEvent>,
    mut ev_loselife: EventReader<LostLifeEvent>,
    mut player_query: Query<&mut Lifes, With<Player>>,
) {
    let mut player_lifes = player_query.single_mut();
    for _ev in ev_loselife.iter() {
        if player_lifes.0 >= 1 {
            player_lifes.0 -= 1;
            info!("Player lost life, current lifes: {}", player_lifes.0);
        } else {
            lost_game.send(LostGameEvent());
            info!("Game Lost!")
        }
    }
}

fn lost_game_handler(
    asset_server: Res<AssetServer>,
    ball_query: Query<Entity, With<Ball>>,
    mut ev_lostgame: EventReader<LostGameEvent>,
    mut commands: Commands,
) {
    for _ev in ev_lostgame.iter() {
        commands.spawn((TextBundle::from_section(
            "Game\nLost!",
            TextStyle {
                font: asset_server.load("font.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::TOP_CENTER)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),));
        info!("despawning entities");
        commands.entity(ball_query.single()).despawn();
    }
}

fn player_controller(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Collision), With<Player>>,
    keyboard_input_event: Res<Input<KeyCode>>,
) {
    for (mut transform, mut curr_dir, mut collision) in &mut player_query {
        if keyboard_input_event.pressed(KeyCode::A) {
            if collision.1 == false && curr_dir.0 >= -2.5 {
                curr_dir.0 -= 0.05;
            }
        } else if keyboard_input_event.pressed(KeyCode::D) {
            if collision.3 == false && curr_dir.0 <= 2.5 {
                curr_dir.0 += 0.05;
            }
        };

        if transform.translation.x > 150. && collision.1 == true {
            collision.1 = false;
        }
        if transform.translation.x < 350. && collision.3 == true {
            collision.3 = false;
        }
        if transform.translation.x > 419. && collision.3 == false {
            collision.3 = true;
            curr_dir.0 = 0.;
            //info!(transform.translation.x);
        } else if transform.translation.x < -420. && collision.1 == false {
            collision.1 = true;
            curr_dir.0 = 0.;
            //info!(transform.translation.x);
        } else {
            transform.translation.x += curr_dir.0;
        }
    }
}
