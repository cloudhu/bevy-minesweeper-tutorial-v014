use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
        // Window setup
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mine Sweeper!".to_string().into(),
                resolution: (1149., 645.).into(),
                ..default()
            }),
            ..default()
        }),
        #[cfg(feature = "debug")]
        // Debug hierarchy inspector
        WorldInspectorPlugin::new(),
    ))
        .add_systems(Startup,camera_setup)
        .run();
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn(Camera2dBundle::default());
}