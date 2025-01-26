use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::math::ops;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sin_translation)
        .run();
}

const X_EXTENT: f32 = 900.;
#[derive(Component)]
struct Ball;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Circle::new(2.0));
    let num_shapes = 128;
    for i in 0..num_shapes {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        commands.spawn((
            Mesh2d(shape.clone()),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                0.0,
                1.0,
            ),
            Ball,
        ));

    }
    let color = Color::hsl(0.0, 0.0, 1.0);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1000.0, 1.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(1.0, 400.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(
            0.0,
            0.0,
            0.0,
        ),
    ));

}

fn sin_translation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Ball>>
) {
    let elements = query.iter().len();
    for (num, mut transform) in (&mut query).into_iter().enumerate() {
        transform.translation.y = 150.0 * ops::sin(time.elapsed_secs() + 2.0 * PI / (elements as f32 + 1.0) * num as f32);
    }
}
