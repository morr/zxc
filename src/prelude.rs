pub use bevy::prelude::*;

pub use crate::assets::*;
pub use crate::camera::*;
pub use crate::input::*;
pub use crate::map::*;
pub use crate::movement::*;
pub use crate::navigation::*;
pub use crate::pawn::*;
pub use crate::settings::*;
pub use crate::story_time::*;
pub use crate::ui::*;
pub use crate::utils::*;

#[derive(Debug, Event)]
pub struct StateChangeEvent<T>(pub T);

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum WorldState {
    #[default]
    Loading,
    Playing,
}
