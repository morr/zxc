use super::*;

#[derive(Component, Reflect, Default)]
#[require(Name = Name::new("Warehouse"), Sprite, Transform)]
pub struct Warehouse {}

#[derive(Component, Reflect, Default)]
#[require(Name = Name::new("House"), Sprite, Transform)]
pub struct House {}

#[derive(Component, Reflect, Default)]
#[require(Name = Name::new("Well"), Sprite, Transform)]
pub struct Well {}
