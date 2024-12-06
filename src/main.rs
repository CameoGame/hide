#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod game;
mod misc;

use bevy::prelude::*;
use bevy::window::WindowResolution;

fn main() {
    let mut app = App::new();
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Game".into(),
            resolution: WindowResolution::new(1600.0, 900.0),
            resizable: true,
            ..bevy::prelude::default()
        }),
        ..bevy::prelude::default()
    };

    #[cfg(feature = "debug")]
    {
        use bevy::log::{Level, LogPlugin};
        app.add_plugins(
            DefaultPlugins
                .set(LogPlugin {
                    filter: "wgpu=error,naga=warn,hide=debug".to_string(),
                    level: Level::INFO,
                    custom_layer: |_| None,
                })
                .set(window_plugin),
        );
    }

    #[cfg(not(feature = "debug"))]
    {
        app.add_plugins(DefaultPlugins.set(window_plugin));
    }

    // game
    app.add_plugins((misc::MiscPlugin, game::GamePlugin));

    app.run();
}
