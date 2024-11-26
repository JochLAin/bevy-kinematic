use bevy::prelude::{Resource, Timer};

#[derive(Resource)]
pub struct EndTimer(pub Timer);
