//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
//use std::env;

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Brick;

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
struct Wall(WallDirection);

enum WallDirection {
    Top,
    Left,
    Right,
}

struct BreakBrickEvent(Entity);

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
        .add_event::<BreakBrickEvent>()
        .add_startup_system(setup)
        .add_startup_system(setup_scoreboard)
        .add_startup_system(setup_bricks)
        .add_system(change_colors)
        .add_system(break_event_handler)
        .add_system(update_scoreboard)
        .add_system(ball_movement)
        .add_system(player_controller)
        .add_system(check_ball_collision)
        .add_system(player_loses_life)
        .add_system(lost_game_handler)
        .run();
}

//x > 449 Left
//x < -450 Right
//aprox. 900
//y > 349 Top

fn setup_scoreboard(mut commands: Commands, asset_server: Res<AssetServer>) {
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
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        //wall
        SpriteBundle {
            texture: asset_server.load("rect.png"),
            transform: Transform {
                translation: Vec3::new(-650., 0., 1.),
                rotation: Quat::from_axis_angle(Vec3::Y, 90.),
                scale: Vec3::new(0.4, 10., 1.0),
                ..default()
            },
            ..default()
        },
        Wall(WallDirection::Left),
    ));
    commands.spawn((
        //wall
        SpriteBundle {
            texture: asset_server.load("rect.png"),
            transform: Transform {
                translation: Vec3::new(620., 0., 1.),
                rotation: Quat::from_axis_angle(Vec3::Y, 90.),
                scale: Vec3::new(0.4, 10., 1.0),
                ..default()
            },
            ..default()
        },
        Wall(WallDirection::Right),
    ));
    commands.spawn((
        //wall
        SpriteBundle {
            texture: asset_server.load("rect.png"),
            transform: Transform {
                translation: Vec3::new(0., 430., 1.),
                scale: Vec3::new(4., 1., 1.0),
                ..default()
            },
            ..default()
        },
        Wall(WallDirection::Top),
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

fn setup_bricks(mut commands: Commands, asset_server: Res<AssetServer>) {
    for h in 0..3 {
        for i in 0..5 {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("rect.png"),
                    transform: Transform {
                        translation: Vec3::new(-455. + 200. * i as f32, 270. - 75. * h as f32, 1.),
                        scale: Vec3::new(0.45, 0.3, 1.0),
                        ..default()
                    },
                    ..default()
                },
                Brick,
            ));
        }
    }
}

fn check_ball_collision(
    mut broke_brick: EventWriter<BreakBrickEvent>,
    mut ball_query: Query<(&Ball, &mut Transform, &mut Velocity), Without<Player>>,
    player_query: Query<(&Player, &Transform, &Velocity), Without<Ball>>,
    brick_query: Query<(&Brick, &Transform, Entity), Without<Ball>>,
) {
    let (_, player_pos, player_velocity) = player_query.single();
    for (_, ball_pos, mut ball_dir) in &mut ball_query {
        if (player_pos.translation.x - ball_pos.translation.x).abs() < 160.
            && (player_pos.translation.y - ball_pos.translation.y).abs() < 45.
        {
            ball_dir.0 = ball_dir.0 * -1.2 + player_velocity.0;
            ball_dir.1 = ball_dir.1 * -1.1;
        }
        for (_, brick_pos, curr_brick) in brick_query.iter() {
            if (ball_pos.translation.x - brick_pos.translation.x).abs() < 80.
                && (brick_pos.translation.y - ball_pos.translation.y).abs() < 45.
            {
                broke_brick.send(BreakBrickEvent(curr_brick));
                ball_dir.0 = ball_dir.0 * -1.2;
                ball_dir.1 = ball_dir.1 * -1.1;
            }
        }
    }
}

fn break_event_handler(
    mut commands: Commands,
    mut ev_broke_brick: EventReader<BreakBrickEvent>,
    mut score_query: Query<&mut Score, With<Player>>,
) {
    for ev in ev_broke_brick.iter() {
        commands.entity(ev.0).despawn();
        score_query.single_mut().0 += 1;
    }
}

fn change_colors(
    mut player_query: Query<(&Player, &mut Sprite), (Without<Wall>, Without<Brick>)>,
    mut wall_query: Query<(&Wall, &mut Sprite), Without<Brick>>,
    mut bricks_query: Query<(&Brick, &mut Sprite), Without<Wall>>,
) {
    let mut player_sprite = player_query.single_mut().1;
    player_sprite.color = Color::rgb(0., 52., 89.);
    for (_, mut wall_sprite) in wall_query.iter_mut() {
        wall_sprite.color = Color::rgb(0., 23., 31.);
    }
    let mut i = 0;
    for (_, mut brick_sprite) in bricks_query.iter_mut() {
        if i % 2 == 0 {
            brick_sprite.color = Color::rgb(0., 126., 167.);
        } else {
            brick_sprite.color = Color::rgb(0., 168., 232.);
        }
        i += 1;
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
        if transform.translation.x > 550. && curr_dir.0 > 0. {
            curr_dir.0 = curr_dir.0 * -0.9;
        } else if transform.translation.x < -560. && curr_dir.0 < 0. {
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
        } else {
            lost_game.send(LostGameEvent());
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

        if transform.translation.x > -380. && collision.1 == true {
            collision.1 = false;
        }
        if transform.translation.x < 380. && collision.3 == true {
            collision.3 = false;
        }
        if transform.translation.x > 460. && collision.3 == false {
            collision.3 = true;
            curr_dir.0 = 0.;
            //info!(transform.translation.x);
        } else if transform.translation.x < -480. && collision.1 == false {
            collision.1 = true;
            curr_dir.0 = 0.;
            //info!(transform.translation.x);
        } else {
            transform.translation.x += curr_dir.0;
        }
    }
}
