use crate::*;

use self::structure::spawn_base;

expose_submodules!(components, systems);

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntityStateChangeEvent<MovementState>>()
            .add_event::<EntityStateChangeEvent<PawnStatus>>()
            .add_systems(OnExit(WorldState::Loading), spawn_pawns.after(spawn_base))
            .add_systems(
                FixedUpdate,
                (
                    update_pawn_color,
                    update_pawn_status_text,
                    wander_idle_pawns,
                )
                    .chain()
                    .run_if(in_state(WorldState::Playing)),
            );

        // .add_systems(
        //     FixedUpdate,
        //     wander_pawns.run_if(in_state(TimeState::Running)),
        // );
    }
}
