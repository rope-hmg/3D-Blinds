use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_6, PI},
    sync::{LazyLock, Mutex},
};

use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    default,
    door::{spawn_doors, Door, Hinge_Side, Louver, Shutter},
};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub enum Message {
    Toggle_Louvers,
    Toggle_Angles,
    Toggle_Doors,
    Top_View,
    Front_View,
}

pub static MESSAGE_QUEUE: LazyLock<Mutex<Vec<Message>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[wasm_bindgen]
pub fn push_message(message: Message) {
    MESSAGE_QUEUE.lock().unwrap().push(message);
}

#[wasm_bindgen]
pub enum Colour {
    White,
    Red,
    Green,
    Blue,
}

pub static COLOUR_QUEUE: LazyLock<Mutex<Option<Colour>>> =
    LazyLock::new(|| Mutex::new(Some(Colour::White)));

#[wasm_bindgen]
pub fn set_colour(colour: Colour) {
    COLOUR_QUEUE.lock().unwrap().replace(colour);
}

pub static LAYOUT_CODE: LazyLock<Mutex<Layout_Code>> = LazyLock::new(|| {
    Mutex::new(Layout_Code {
        code: "LBR".to_owned(),
        applied: false,
    })
});

#[wasm_bindgen]
pub fn set_layout_code(code: String) {
    let mut layout = LAYOUT_CODE.lock().unwrap();
    layout.code = code;
    layout.applied = false;
}

pub struct Queue_Plugin;

impl Plugin for Queue_Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Camera_Transition::new())
            .insert_resource(Layout_Code {
                code: "LBR".to_owned(),
                applied: false,
            })
            .insert_resource(Target_Door_State { open: false })
            .insert_resource(Target_Louver_State { open: false })
            .insert_resource(Shutter_Colour {
                colour: Colour::White,
            })
            .add_systems(
                Update,
                (
                    poll_queue,
                    apply_layout_code,
                    toggle_louvers,
                    toggle_doors,
                    transition_camera,
                    apply_shutter_colour,
                ),
            );
    }
}

#[derive(Resource, Clone)]
pub struct Layout_Code {
    pub code: String,
    pub applied: bool,
}

#[derive(Resource)]
pub struct Camera_Transition {
    pub target: Transform,
}

#[derive(Resource)]
pub struct Target_Door_State {
    pub open: bool,
}

#[derive(Resource)]
pub struct Target_Louver_State {
    pub open: bool,
}

#[derive(Resource)]
pub struct Shutter_Colour {
    pub colour: Colour,
}

impl Camera_Transition {
    pub fn new() -> Self {
        Self {
            target: Transform::from_xyz(0.0, 0.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        }
    }
}

fn poll_queue(
    mut camera_transition: ResMut<Camera_Transition>,
    mut door_state: ResMut<Target_Door_State>,
    mut louver_state: ResMut<Target_Louver_State>,
    mut layout_code: ResMut<Layout_Code>,
    mut shutter_colour: ResMut<Shutter_Colour>,
) {
    let mut queue = MESSAGE_QUEUE.lock().unwrap();

    for message in queue.drain(..) {
        match message {
            Message::Toggle_Louvers => {
                louver_state.open = !louver_state.open;
            }

            Message::Toggle_Angles => {}

            Message::Toggle_Doors => {
                door_state.open = !door_state.open;
            }

            Message::Top_View => {
                camera_transition.target = default!(Transform {
                    translation: Vec3::new(0.0, 9.0, 0.0),
                    rotation: Quat::from_rotation_x(-FRAC_PI_2),
                });
            }

            Message::Front_View => {
                camera_transition.target = default!(Transform {
                    translation: Vec3::new(0.0, 0.0, 9.0),
                });
            }
        }
    }

    if let Some(colour) = COLOUR_QUEUE.lock().unwrap().take() {
        shutter_colour.colour = colour;
    }

    let mut layout = LAYOUT_CODE.lock().unwrap();
    if !layout.applied {
        *layout_code = layout.clone();
        layout.applied = true;
    }
}

fn apply_layout_code(
    mut commands: Commands,
    shutters: Query<Entity, With<Shutter>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut layout_code: ResMut<Layout_Code>,
) {
    if !layout_code.applied {
        layout_code.applied = true;

        for shutter in shutters.iter() {
            commands.entity(shutter).despawn_recursive();
        }

        spawn_doors(
            &layout_code.code,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

fn toggle_louvers(
    mut louvers: Query<(&mut Louver, &mut Transform)>,
    louver_state: Res<Target_Louver_State>,
) {
    for (mut louver, mut transform) in louvers.iter_mut() {
        let angle = FRAC_PI_4;

        if louver_state.open && !louver.open {
            transform.rotate(Quat::from_rotation_x(-angle));
            louver.open = true;
        }

        if !louver_state.open && louver.open {
            transform.rotate(Quat::from_rotation_x(angle));
            louver.open = false
        }
    }
}

fn toggle_doors(mut doors: Query<(&mut Door, &mut Transform)>, door_state: Res<Target_Door_State>) {
    for (mut door, mut transform) in doors.iter_mut() {
        // TODO: Not magic numbers
        let right = transform.right() * 0.2;
        let front = transform.forward() * 0.025;

        let (rotation_point, angle) = match door.hinge {
            Hinge_Side::Left => (transform.translation - right + front, FRAC_PI_4),
            Hinge_Side::Right => (transform.translation + right + front, -FRAC_PI_4),
        };

        if door_state.open && !door.open {
            transform.rotate_around(rotation_point, Quat::from_rotation_y(-angle));
            door.open = true;
        }

        if !door_state.open && door.open {
            transform.rotate_around(rotation_point, Quat::from_rotation_y(angle));
            door.open = false
        }
    }
}

fn transition_camera(
    mut camera: Query<&mut Transform, With<Camera>>,
    camera_transition: Res<Camera_Transition>,
) {
    for mut transform in camera.iter_mut() {
        transform.translation = transform
            .translation
            .lerp(camera_transition.target.translation, 0.1);

        transform.rotation = transform
            .rotation
            .lerp(camera_transition.target.rotation, 0.1);
    }
}

fn apply_shutter_colour(
    material_handles: Query<&Handle<StandardMaterial>>,
    shutter_colour: Res<Shutter_Colour>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for handle in material_handles.iter() {
        if let Some(material) = materials.get_mut(handle) {
            let colour = match shutter_colour.colour {
                Colour::White => Color::WHITE,
                Colour::Red => Color::RED,
                Colour::Green => Color::GREEN,
                Colour::Blue => Color::BLUE,
            };

            material.base_color = colour;
        }
    }
}
