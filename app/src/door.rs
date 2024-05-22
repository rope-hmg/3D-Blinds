use std::f32::consts::FRAC_PI_8;

use bevy::prelude::*;

use crate::default;

pub struct Door_Plugin;

impl Plugin for Door_Plugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Debug, Clone, Copy)]
pub enum Hinge_Side {
    Left,
    Right,
}

#[derive(Component)]
pub struct Door {
    pub hinge: Hinge_Side,
    pub open: bool,
}

#[derive(Component)]
pub struct Louver {
    pub open: bool,
}

#[derive(Component)]
pub struct Frame;

#[derive(Component)]
pub struct Shutter;

pub struct Angle_Entry {
    entity: Entity,
    origin: Transform,
    target: Transform,
    angle: f32,
}

#[derive(Resource)]
pub struct Angle_Fold {
    pub transforms: Vec<Angle_Entry>,
}

pub fn spawn_doors(
    layout_code: &str,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // mut angle_fold: ResMut<Angle_Fold>,
) {
    let shutter_width = 0.5;
    let shutter_height = 1.0;
    let shutter_depth = 0.05;

    let shutters = layout_code
        .chars()
        .flat_map(|c| match c {
            'L' => Some(Hinge_Side::Left),
            'R' => Some(Hinge_Side::Right),
            _ => None,
        })
        .collect::<Vec<_>>();

    let scale = 3.0;
    let total_width = (shutter_width + shutter_depth) * scale;

    //           |Grand total width| half |  |Half of single |
    //           |                 |      |  |               |
    let offset = (shutters.len() as f32 * total_width) / 2.0 - total_width / 2.0;
    let material = materials.add(Color::WHITE);

    for i in 0..shutters.len() {
        commands
            .spawn((
                default!(SpatialBundle {
                    transform: Transform::from_xyz(i as f32 * total_width - offset, 0.0, 0.0)
                        .with_scale(Vec3::splat(3.0))
                }),
                Shutter,
            ))
            .with_children(|parent| {
                // Door
                parent
                    .spawn((
                        default!(SpatialBundle {}),
                        Door {
                            hinge: shutters[i],
                            open: false,
                        },
                    ))
                    .with_children(|parent| {
                        let louver_height = shutter_height / 10.0;
                        let louver_depth = shutter_depth / 2.0;
                        for j in 0..10 {
                            parent.spawn((
                                default!(PbrBundle {
                                    mesh: meshes.add(Cuboid::new(
                                        shutter_width,
                                        louver_height,
                                        louver_depth
                                    )),
                                    material: material.clone(),
                                    transform: Transform::from_xyz(
                                        0.0,
                                        j as f32 * louver_height - 0.45,
                                        0.0
                                    )
                                    .with_rotation(Quat::from_rotation_x(-FRAC_PI_8))
                                }),
                                Louver { open: false },
                            ));
                        }
                    });

                let offset_x = shutter_width / 2.0;
                let offset_y = shutter_height / 2.0;

                let frame = parent
                    .spawn((default!(SpatialBundle {}), Frame))
                    .with_children(|parent| {
                        // Left
                        parent.spawn(default!(PbrBundle {
                            mesh: meshes.add(Cuboid::new(
                                shutter_depth,
                                shutter_height + shutter_depth,
                                shutter_depth
                            )),
                            material: material.clone(),
                            transform: Transform::from_xyz(-offset_x, 0.0, 0.0),
                        }));
                        // Right
                        parent.spawn(default!(PbrBundle {
                            mesh: meshes.add(Cuboid::new(
                                shutter_depth,
                                shutter_height + shutter_depth,
                                shutter_depth
                            )),
                            material: material.clone(),
                            transform: Transform::from_xyz(offset_x, 0.0, 0.0),
                        }));
                        // Top
                        parent.spawn(default!(PbrBundle {
                            mesh: meshes.add(Cuboid::new(
                                shutter_width,
                                shutter_depth,
                                shutter_depth,
                            )),
                            material: material.clone(),
                            transform: Transform::from_xyz(0.0, offset_y, 0.0),
                        }));
                        // Bottom
                        parent.spawn(default!(PbrBundle {
                            mesh: meshes.add(Cuboid::new(
                                shutter_width,
                                shutter_depth,
                                shutter_depth,
                            )),
                            material: material.clone(),
                            transform: Transform::from_xyz(0.0, -offset_y, 0.0),
                        }));
                    })
                    .id();

                // angle_fold.transforms.push(Angle_Entry {
                //     entity: frame,
                //     origin: Transform::default(),
                //     target: Transform::default(),
                //     angle: 150.0,
                // });
            });
    }
}
