use super::*;

pub static UI_COLOR: Lazy<Color> = Lazy::new(|| Color::hex("181a1c").unwrap());

#[derive(Component)]
pub struct PawnAgeTextUI {}

#[derive(Component)]
pub struct SimulationSpeedTextUI {}

#[derive(Component)]
pub struct SimulationDateTimeTextUI {}

#[derive(Component)]
pub struct DebugStatusTextUI {}

#[derive(Component)]
pub struct DebugHelpBlockUI {}

#[derive(Component)]
pub struct PawnStockTextUI {}

#[derive(Component)]
pub struct FoodStockTextUI {}
