use bevy::prelude::*;

use crate::default;

pub struct Scene_Plugin;

impl Plugin for Scene_Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_scene)
            .add_systems(Update, update_scene);
    }
}

fn init_scene(mut commands: Commands) {
    // Light
    commands.spawn(default!(PointLightBundle {
        point_light: default!(PointLight {
            shadows_enabled: true,
        }),
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
    }));

    // Camera
    commands.spawn(default!(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    }));
}

fn update_scene() {}
