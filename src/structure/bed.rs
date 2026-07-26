use crate::map::TileItem;
use crate::navigation::Navmesh;
use crate::prelude::*;
use crate::structure::BED_SIZE;
pub struct BedPlugin;

impl Plugin for BedPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AvailableBeds>()
            .add_observer(on_bed_claimed)
            .add_observer(on_bed_unclaimed);
    }
}

fn on_bed_claimed(_event: On<Add, BedOwnedBy>, mut available_beds: ResMut<AvailableBeds>) {
    available_beds.decrement();
}

fn on_bed_unclaimed(_event: On<Remove, BedOwnedBy>, mut available_beds: ResMut<AvailableBeds>) {
    available_beds.increment();
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
#[require(Name = Name::new("Bed"), Sprite, Transform)]
pub struct Bed {}

/// On a Bed: points to the Pawn that claimed it. Claim/unclaim by
/// inserting/removing this component; `AvailableBeds` is kept in sync by
/// the `Add`/`Remove` observers in `BedPlugin`, and despawning either side
/// cleans the relationship up automatically.
///
/// Claims go through `Commands`, so two pawns resting in the same command
/// flush can race for one bed; the later insert wins and the loser simply
/// re-claims on its next rest cycle.
#[derive(Component, Reflect)]
#[relationship(relationship_target = OwnedBed)]
pub struct BedOwnedBy(pub Entity);

/// On a Pawn: the bed claimed by it (at most one in practice).
#[derive(Component, Reflect, Debug)]
#[relationship_target(relationship = BedOwnedBy)]
pub struct OwnedBed(Vec<Entity>);

impl OwnedBed {
    pub fn bed(&self) -> Option<Entity> {
        self.0.first().copied()
    }
}

impl Bed {
    pub fn spawn(
        grid_tile: IVec2,
        commands: &mut Commands,
        texture: Handle<Image>,
        navmesh: &mut Navmesh,
        available_beds: &mut ResMut<AvailableBeds>,
    ) {
        // let size = IVec2::new(BED_SIZE, BED_SIZE);
        let tile_item = TileItem {
            grid_tile,
            width: BED_SIZE,
            height: BED_SIZE,
            aspect_ratio: 1.0,
            z_index: STRUCTURE_Z_INDEX,
            movement_cost: config().movement_cost.furniture,
        };

        let id = commands
            .spawn((
                Bed::default(),
                Sprite {
                    image: texture,
                    // custom_size: Some(size.grid_tile_edge_to_world()),
                    custom_size: Some(tile_item.sprite_size()),
                    ..default()
                },
                // Transform::from_translation(
                //     (grid_tile.grid_tile_edge_to_world() + size.grid_tile_edge_to_world() / 2.0)
                //         .extend(STRUCTURE_Z_INDEX),
                // ),
                tile_item.sprite_transform(None),
            ))
            // .insert(ShowAabbGizmo {
            //     colo: Some(Color::srgba(1.0, 1.0, 1.0, 0.25)),
            // })
            .id();

        // navmesh.update_cost(
        //     (grid_tile.x)..(grid_tile.x + size.x),
        //     (grid_tile.y)..(grid_tile.y + size.y),
        //     Navtile::config_cost_to_pathfinding_cost(config().movement_cost.furniture),
        // );
        // navmesh.add_occupant::<Bed>(&id, grid_tile.x, grid_tile.y);
        tile_item.sync_navmesh::<Bed>(id, navmesh);

        available_beds.increment();
    }
}
