use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

#[derive(Component)]
struct Velocity(f32, f32);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Block;

#[derive(Component)]
struct Ball;

#[derive(Component)]
//Up, right, down, left
struct Collision(bool, bool, bool, bool);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        .add_system(ball_movement)
        .add_system(player_controller)
        .add_system(check_ball_collision)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
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
        Player,
    ));
}

fn check_ball_collision(
    mut ball_query: Query<(&Ball, &mut Transform, &mut Velocity, Without<Player>)>,
    player_query: Query<(&Player, &Transform, &Velocity, Without<Ball>)>,
) {
    let player_pos = player_query.single().1;
    let player_vel = player_query.single().2;
    for (_, ball_pos, mut ball_dir, _) in &mut ball_query {
        if (player_pos.translation.x - ball_pos.translation.x).abs() < 160.
            && (player_pos.translation.y - ball_pos.translation.y).abs() < 45.
        {
            ball_dir.0 = ball_dir.0 * -1.2 + player_vel.0;
            ball_dir.1 = ball_dir.1 * -1.1;
        }
    }
}

fn ball_movement(mut ball_query: Query<(&mut Transform, &mut Velocity, With<Ball>)>) {
    for (mut transform, mut curr_dir, _) in &mut ball_query {
        if curr_dir.0 == 0. && curr_dir.1 == 0. {
            curr_dir.0 = 0.7;
            curr_dir.1 = 1.5;
        } else if curr_dir.1 == 0. {
            curr_dir.1 = 0.2;
        }
        //si choca con una pared
        if transform.translation.x > 449. && curr_dir.0 > 0. {
            curr_dir.0 = curr_dir.0 * -0.9;
            //info!("x_speed: {}, y_speed:{}", curr_dir.0, curr_dir.1);
            //info!(
            //    "curr_pos: x{} y{}",
            //    transform.translation.x, transform.translation.y
            //);
        } else if transform.translation.x < -450. && curr_dir.0 < 0. {
            curr_dir.0 = 0. * -0.9;
            //info!("x_speed: {}, y_speed:{}", curr_dir.0, curr_dir.1);
            //info!(
            //    "curr_pos: x{} y{}",
            //    transform.translation.x, transform.translation.y
            //);
        } else if transform.translation.y > 349. && curr_dir.1 > 0. {
            curr_dir.1 = curr_dir.1 * -0.9;
            //info!("x_speed: {}, y_speed:{}", curr_dir.0, curr_dir.1);
            //info!(
            //    "curr_pos: x{} y{}",
            //    transform.translation.x, transform.translation.y
            //);
        }
        if transform.translation.y < -350. && curr_dir.1 < 0. {
            curr_dir.1 = curr_dir.1 * -0.9;
            //info!(transform.translation.y);
            //info!("x_speed: {}, y_speed:{}", curr_dir.0, curr_dir.1);
            ////info!(
            //    "curr_pos: x{} y{}",
            //    transform.translation.x, transform.translation.y
            //);
        } else {
            transform.translation.x += curr_dir.0;
            transform.translation.y += curr_dir.1;
        }
    }
}

fn player_controller(
    mut player_query: Query<(&mut Transform, &mut Velocity, &mut Collision, With<Player>)>,
    keyboard_input_event: Res<Input<KeyCode>>,
) {
    for (mut transform, mut curr_dir, mut collision, _) in &mut player_query {
        if keyboard_input_event.pressed(KeyCode::A) {
            if collision.1 == false && curr_dir.0 >= -1.5 {
                curr_dir.0 -= 0.03;
            }
        } else if keyboard_input_event.pressed(KeyCode::D) {
            if collision.3 == false && curr_dir.0 <= 1.5 {
                curr_dir.0 += 0.03;
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
