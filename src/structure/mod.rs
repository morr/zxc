use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Structure {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Bundle)]
pub struct StructureBundle {
    pub structure: Structure,
    pub name: Name,
}
