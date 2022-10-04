#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Capitan Arturo".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    //* add camera
    commands.spawn_bundle(Camera2dBundle::default());
    //* add rect
//    commands.spawn_bundle(SpriteBundle {
//        sprite: Sprite {
//            color: Color::rgb(0.25, 0.25, 0.75),
//            custom_size: Some(Vec2::new(150., 150.)),
//            ..Default::default()
//        },
//        ..Default::default()
//    });
    //* add sprite
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        ..Default::default()
    });
}