use bevy::math::vec2;
use bevy::prelude::*;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct Speed(f32);

pub fn spawn_player(
    mut commands: Commands
) {
    commands
    .spawn_bundle(
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::splat(16.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .with_children(|builder| {
        builder.spawn_bundle(
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(vec2(4.0, 8.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(12.0 * Vec3::Y + Vec3::Z),
                ..Default::default()
            }
        );
    })
    .insert(Speed(250.0))
    .insert(PlayerCharacter);
}

pub fn move_player(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<PlayerCharacter>>
) {
    query.for_each_mut(|(mut transform, player_speed)| {
        let mut m = Vec3::ZERO;
        if keyboard.pressed(KeyCode::A) {
            m -= Vec3::X
        } 
        if keyboard.pressed(KeyCode::D) {
            m += Vec3::X
        }
        if keyboard.pressed(KeyCode::S) {
            m -= Vec3::Y
        }
        if keyboard.pressed(KeyCode::W) {
            m += Vec3::Y
        }
        transform.translation += time.delta_seconds() * player_speed.0 * m.normalize_or_zero();
    });
}

pub fn track_mouse_cursor(
    windows: Res<Windows>,
    mut query: Query<&mut Transform, With<PlayerCharacter>>
) {
    let cursor_world_position = 
        windows.get_primary().map(|window| {
            window.cursor_position().map(|cursor_position| {
                cursor_position - 0.5 * vec2(window.width(), window.height())
            })
        }).flatten();
    if let Some(cursor_world_position) = cursor_world_position {
        query.for_each_mut(|mut transform| {
            let displacement = cursor_world_position - transform.translation.truncate();
            if let Some(dir) = displacement.try_normalize() {
                transform.rotation = Quat::from_rotation_arc_2d(Vec2::Y, dir);
            }
        });
    }
}

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(|mut commands: Commands| { commands.spawn_bundle(OrthographicCameraBundle::new_2d()); })   
    .add_startup_system(spawn_player)
    .add_system(move_player)
    .add_system(track_mouse_cursor)
    .run();
}