use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    let cube_mesh = meshes.add(Cuboid::new(2.0, 2.0, 2.0).mesh());
    commands.spawn(PbrBundle {
        mesh: cube_mesh,
        ..Default::default()
    });
}