#![allow(non_camel_case_types)]
#![feature(lazy_cell)]

mod door;
mod queue;
mod scene;

use bevy::{asset::AssetMetaCheck, prelude::*};
use wasm_bindgen::prelude::wasm_bindgen;

#[macro_export]
macro_rules! default {
    ($type:ident { $( $key:ident : $value:expr ),* $(,)? }) => {
        $type {
            $( $key : $value , )*
            ..Default::default()
        }
    };
}

#[wasm_bindgen]
pub fn start_app() {
    App::new()
        // -----------------------------------------------
        // Bevy
        .add_plugins(
            DefaultPlugins
                .set(default!(WindowPlugin {
                    primary_window: Some(default!(Window {
                        canvas: Some("#app-canvas".to_owned()),
                        prevent_default_event_handling: true,
                    })),
                }))
                .set(default!(AssetPlugin {
                    file_path: "/public/assets".to_owned(),
                })),
        )
        .insert_resource(AssetMetaCheck::Never)
        // -----------------------------------------------
        // Third Party
        // .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        // -----------------------------------------------
        // First Party
        // .insert_resource(Random::new())
        .add_plugins(scene::Scene_Plugin)
        .add_plugins(queue::Queue_Plugin)
        .add_plugins(door::Door_Plugin)
        // .add_plugins(picker::Picker_Plugin)
        // .add_plugins(inspect::Inspect_Plugin)
        .run();
}
