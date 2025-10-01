use super::*;

#[derive(Component, Reflect, Default)]
#[require(Name, Sprite, Transform)]
pub struct Warehouse {}

#[derive(Component, Reflect, Default)]
#[require(Name, Sprite, Transform)]
pub struct House {}

#[derive(Component, Reflect, Default)]
#[require(Name, Sprite, Transform)]
pub struct Well {}
