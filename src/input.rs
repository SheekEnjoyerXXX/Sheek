use bevy::{prelude::*, transform::TransformSystem, log};
use crate::prelude::*;

#[derive(Component, Default)]
pub struct Player;
#[derive(Component, Default)]
pub struct CameraFollow(pub i32);

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, process_input.run_if(in_state(AppState::Gaming)))
            .add_systems(PostUpdate, camera_follow.run_if(in_state(AppState::Gaming)).after(TransformSystem::TransformPropagate));
    }
}

fn process_input(
    mut q_players: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
) {
    for mut transform in q_players.iter_mut() {
        let left = keys.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keys.any_pressed([KeyCode::D, KeyCode::Right]);

        let mut to_move = Vec3::ZERO;

        if left {
            to_move.x -= 1.;
        }
        if right {
            to_move.x += 1.;
        }

        let speed = if keys.pressed(KeyCode::ShiftLeft) {
            4.
        } else {
            1.
        };

        transform.translation += to_move * 200. * speed * time.delta_seconds();
    }
}

fn camera_follow(
    q_follow: Query<(&GlobalTransform, &CameraFollow)>,
    mut q_camera: Query<&mut Transform, (With<PlayerCamera>, Without<CameraFollow>)>,
) {
    let Some((transform, _)) =
        q_follow.iter().max_by_key(|(_, follow)| follow.0)
    else { return; };

    let Some(mut camera) = q_camera.iter_mut().next() else {
        return;
    };

    camera.translation = transform.translation();
}