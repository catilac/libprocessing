//! A light in Processing
//!

use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Light {
    pub light_type: LightType,
    pub pos: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightType {
    Ambient,
    Directional,
    Point,
    Spot,
}

pub fn create(
    In((light_type, x, y, z)): In<(LightType, f32, f32, f32)>,
    mut commands: Commands,
) -> Entity {
    // let light = Light {
    //     light_type: light_type,
    //     pos: Vec3::new(x, y, z),
    // };
    // commands.spawn(light).id()

    match light_type {
        LightType::Directional => commands
            .spawn((DirectionalLight::default(), Transform::from_xyz(x, y, z)))
            .id(),
        _ => commands.spawn(AmbientLight::default()).id(),
    }
}
