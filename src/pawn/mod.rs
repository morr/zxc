use crate::*;

expose_submodules!(components, systems); // , debug_components

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Pawn>()
            .add_message::<EntityStateChangeMessage<PawnState>>()
            // .add_message::<PawnBirthdayEvent>()
            .add_message::<PawnDeathMessage>()
            .add_systems(OnExit(AppState::Loading), spawn_pawns.after(spawn_base))
            .add_systems(
                FixedUpdate,
                (
                    // update_pawn_color,
                    update_pawn_state_text,
                    progress_pawn_daily,
                    progress_pawn_dying,
                    progress_pawn_death,
                )
                    .chain()
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
