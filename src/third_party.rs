use bevy::app::{PluginGroup, PluginGroupBuilder};
use bevy_rapier3d::plugin::*;
use bevy_rapier3d::render::RapierDebugRenderPlugin;

pub struct ThirdPartyPlugins;

impl PluginGroup for ThirdPartyPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>();

        builder = builder.add(RapierPhysicsPlugin::<NoUserData>::default());

        #[cfg(feature = "debug")]
        {
            builder = builder.add(RapierDebugRenderPlugin::default());
        }

        builder
    }
}
