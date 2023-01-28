use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use roguelike::{
    components::{Player, Position, Renderable},
    resources::Map,
};

fn setup(
    mut commands: Commands,
    map: Res<Map>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawns a default 2d camera
    commands.spawn(Camera2dBundle::default());

    // Spawns a player entity
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(10.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PINK)),
            transform: Transform::from_translation(Vec3::new(50.0, 0.0, 0.0)),
            ..default()
        },
        Position { x: 50, y: 0 },
        Renderable,
        Player,
    ));

    for x in 0..map.width {
        for y in 0..map.height {
            if x == 0 || x == map.width - 1 || y == 0 || y == map.height - 1 {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Quad::new(Vec2::new(10.0, 10.0)).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::FUCHSIA)),
                        transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                        ..default()
                    },
                    Position {
                        x: x as i32,
                        y: y as i32,
                    },
                    Renderable,
                ));
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .insert_resource(Map {
            width: 40,
            height: 20,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run();
}
