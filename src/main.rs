use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use generate::{AugmentedNoiseFn, AugmentedNoiseFnOperation, TerrainGenerator};

mod generate;

#[derive(Component)]
struct Terrain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, generate_terrain)
        .run();
}

const WIDTH: usize = 1024;
const HEIGHT: usize = 1024;
const AMPLITUDE: f64 = 200.0;
const X_FREQUENCY: f64 = 0.005;
const Y_FREQUENCY: f64 = 0.005;

fn generate_terrain(
    keys: Res<Input<KeyCode>>,
    query: Query<Entity, With<Terrain>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        // Delete the terrain entity if there is one already.
        if let Ok(terrain_entity) = query.get_single() {
            commands.entity(terrain_entity).despawn_recursive();
        }

        let generator = TerrainGenerator {
            noise_function: AugmentedNoiseFn::from([
                AMPLITUDE,
                X_FREQUENCY,
                Y_FREQUENCY,
                0.0,
                0.0,
                0.0,
            ]),
            operations: Some(vec![
                AugmentedNoiseFnOperation::AddNoiseFn(AugmentedNoiseFn::from([
                    AMPLITUDE / 10.0,
                    X_FREQUENCY * 5.0,
                    Y_FREQUENCY * 5.0,
                    0.0,
                    0.0,
                    0.0,
                ])),
                AugmentedNoiseFnOperation::AddNoiseFn(AugmentedNoiseFn::from([
                    AMPLITUDE / 100.0,
                    X_FREQUENCY * 50.0,
                    Y_FREQUENCY * 50.0,
                    0.0,
                    0.0,
                    0.0,
                ])),
            ]),
        };

        // Get positions of all verticies.
        let mut positions: Vec<[f32; 3]> = Vec::new();
        for i in 0..(WIDTH * HEIGHT) {
            let x_sample_point = (i / (WIDTH)) as f64;
            let z_sample_point = (i % (HEIGHT)) as f64;
            let y_sample_point = generator.sample(x_sample_point, z_sample_point);

            positions.push([
                x_sample_point as f32,
                y_sample_point as f32,
                z_sample_point as f32,
            ]);
        }

        // Generate indices that define the triangles for the mesh.
        let mut indices: Vec<u32> = Vec::new();
        for y in 0..(HEIGHT - 1) {
            for x in 0..(WIDTH - 1) {
                indices.push((y * WIDTH + x) as u32);
                indices.push((y * WIDTH + x + 1) as u32);
                indices.push(((y + 1) * WIDTH + x) as u32);

                indices.push((y * WIDTH + x + 1) as u32);
                indices.push(((y + 1) * WIDTH + x + 1) as u32);
                indices.push(((y + 1) * WIDTH + x) as u32);
            }
        }

        let mut terrain_mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_indices(Some(bevy::render::mesh::Indices::U32(indices)));

        // Generate normals automatically!
        terrain_mesh.duplicate_vertices();
        terrain_mesh.compute_flat_normals();

        let mesh_handle = meshes.add(terrain_mesh);

        commands.spawn((
            PbrBundle {
                mesh: mesh_handle,
                material: materials.add(StandardMaterial {
                    base_color: Color::GREEN,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            Terrain,
        ));
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.0,
            ..default()
        },
        transform: Transform {
            translation: Vec3::splat(0.0),
            rotation: Quat::from_euler(EulerRot::XYZ, -0.1, 0.0, 0.0),
            ..default()
        },
        ..default()
    });

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-20.0, 20.0, -20.0),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}
