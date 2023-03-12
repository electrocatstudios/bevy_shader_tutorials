use bevy::{
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(rotate_block)
        .run();
}

#[derive(Component)]
pub struct Block {
    rot: f32
}

// setup runs once during the program start
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut std_materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a block
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
            material: std_materials.add(Color::rgba(1.0,0.5,1.0,1.0).into()),
            transform: Transform::from_xyz(0.0,0.0,0.0),
            ..Default::default()
        })
        .insert(Block{rot:1.0});

    // Setup a camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
}

const ROTATION_SPEED: f32 = 0.5;

// Rotate the block - called every frame
pub fn rotate_block(
    time: Res<Time>,
    mut blocks: Query<(&mut Transform, &mut Block)>,
) {
    for (mut transform, mut block) in &mut blocks {
        block.rot += ROTATION_SPEED * time.delta_seconds();
        if block.rot > std::f32::consts::PI * 2.0 {
            block.rot -= std::f32::consts::PI * 2.0;
        }
        transform.rotation = Quat::from_rotation_y(block.rot);
    }
}