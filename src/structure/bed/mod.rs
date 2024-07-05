use super::*;

pub struct BedPlugin;

impl Plugin for BedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bed>().init_resource::<AvailableBeds>();
    }
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct AvailableBeds(pub u32);

impl AvailableBeds {
    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn decrement(&mut self) {
        self.0 -= 1;
    }
}

#[derive(Component, Reflect, Default)]
pub struct Bed {
    pub owner: Option<Entity>,
}

impl Bed {
    pub fn spawn(
        grid_tile: IVec2,
        commands: &mut Commands,
        texture: Handle<Image>,
        navmesh: &mut Navmesh,
        available_beds: &mut ResMut<AvailableBeds>,
    ) {
        let size = IVec2::new(BED_SIZE, BED_SIZE);

        let id = commands
            .spawn((
                Bed::default(),
                Name::new("Bed"),
                SpriteBundle {
                    texture,
                    sprite: Sprite {
                        custom_size: Some(size.grid_tile_edge_to_world()),
                        ..default()
                    },
                    transform: Transform::from_translation(
                        (grid_tile.grid_tile_edge_to_world()
                            + size.grid_tile_edge_to_world() / 2.0)
                            .extend(STRUCTURE_Z_INDEX),
                    ),
                    ..default()
                },
            ))
            // .insert(ShowAabbGizmo {
            //     colo: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
            // })
            .id();

        navmesh.update_cost(
            (grid_tile.x)..(grid_tile.x + size.x),
            (grid_tile.y)..(grid_tile.y + size.y),
            Navtile::config_cost_to_pathfinding_cost(config().movement_cost.furniture),
        );
        navmesh.add_occupant::<Bed>(&id, grid_tile.x, grid_tile.y);

        available_beds.increment();
    }

    pub fn claim_by(
        &mut self,
        bed_entity: Entity,
        pawn_entity: Entity,
        pawn: &mut Pawn,
        available_beds: &mut ResMut<AvailableBeds>,
    ) {
        self.owner = Some(pawn_entity);
        pawn.owned_bed = Some(bed_entity);
        available_beds.decrement();
    }

    pub fn unclaim_by(&mut self, pawn: &mut Pawn, available_beds: &mut ResMut<AvailableBeds>) {
        self.owner = None;
        pawn.owned_bed = None;
        available_beds.increment();
    }
}
