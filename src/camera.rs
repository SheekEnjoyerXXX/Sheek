use bevy::{prelude::*, window::WindowResized, render::camera::Viewport};
use crate::prelude::globals;

#[derive(Component, Default)]
pub struct PlayerCamera;


pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PostStartup, setup_camera)
        .add_systems(Update, update_viewport);
    }
}

fn update_viewport(
    q_window: Query<&Window>,
    resize_reader: EventReader<WindowResized>,
    mut q_main_camera: Query<&mut Camera, With<PlayerCamera>>,
) {
    if resize_reader.len() == 0 {
        return;
    }

    let mut camera = q_main_camera.single_mut();
    let window = q_window.single();

    let target_aspect = globals::TARGET_RATIO;
    let current_aspect = window.width() / window.height();
    let size = Vec2::new(
        window.resolution.width() as f32 * window.scale_factor() as f32,
        window.resolution.height() as f32 * window.scale_factor() as f32,
    );

    let (viewport_width, viewport_height) = if current_aspect > target_aspect {
        ((size.y * target_aspect), size.y)
    } else {
        ((size.x as f32), (size.x as f32 / target_aspect))
    };

    let black_box_width = size.x as f32 - viewport_width;
    let black_box_height = size.y as f32 - viewport_height;

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(black_box_width as u32 / 2, black_box_height as u32 / 2),
        physical_size: UVec2::new(viewport_width as u32, viewport_height as u32),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::Fixed {
                    width: globals::WIDTH,
                    height: globals::HEIGHT,
                },
                near: -1000.,
                far: 1000.,
                ..default()
            },
            ..default()
        },
        PlayerCamera,
        Name::new("PlayerCamera"),
    ));
}