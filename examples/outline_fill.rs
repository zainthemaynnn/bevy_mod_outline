use bevy::{prelude::*, render::mesh::VertexAttributeValues, window::close_on_esc};

use bevy_mod_outline::*;

#[bevy_main]
fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, OutlinePlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc, shift))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add cube with generated outline normals
    let mut sphere_mesh = Mesh::from(Sphere { radius: 0.5 });
    sphere_mesh.generate_outline_normals().unwrap();
    let n_verts = sphere_mesh.attributes().next().unwrap().1.len();
    sphere_mesh.insert_attribute(ATTRIBUTE_Y_CUTOFF, vec![1.0; n_verts]);
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(sphere_mesh),
            material: materials.add(StandardMaterial::from(Color::rgb(0.1, 0.1, 0.9))),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(OutlineBundle {
            outline: OutlineVolume {
                visible: true,
                colour: Color::rgba(0.0, 1.0, 0.0, 1.0),
                width: 25.0,
            },
            mode: OutlineMode::RealVertex,
            ..default()
        });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::new(Vec3::Y).mesh().size(5.0, 5.0).build()),
        material: materials.add(StandardMaterial::from(Color::rgb(0.3, 0.5, 0.3))),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn shift(time: Res<Time>, mut meshes: ResMut<Assets<Mesh>>) {
    for (_id, mesh) in meshes.iter_mut() {
        let Some(VertexAttributeValues::Float32(ref mut values)) =
            mesh.attribute_mut(ATTRIBUTE_Y_CUTOFF)
        else {
            continue;
        };

        values.fill(1.0 + time.elapsed_seconds_wrapped().sin() * 0.5);
    }
}
