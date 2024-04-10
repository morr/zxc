use super::*;

pub const BASE_WIDTH: i32 = 8;
pub const BASE_HEIGHT: i32 = 14;

macro_rules! define_building_types {
    ($($name:ident),*) => {
        #[derive(Debug, Clone, Eq, PartialEq, Hash, States)]
        pub enum BuildingType {
            $($name),*
        }

        $(
            #[derive(Component)]
            pub struct $name;
        )*
    };
}

define_building_types!(Warehouse, Home, FarmTile);
