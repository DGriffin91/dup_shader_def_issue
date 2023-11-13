use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use post_processing_1::PostProcessPlugin1;
use post_processing_2::PostProcessPlugin2;

mod post_processing_1;
mod post_processing_2;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PostProcessPlugin1, PostProcessPlugin2))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0))
            .looking_at(Vec3::default(), Vec3::Y),
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::WHITE),
            ..default()
        },
        ..default()
    },));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    },));
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
        ..default()
    });
}
