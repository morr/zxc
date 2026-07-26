//! Единственный законный glob-импорт внутри крейта.
//! Правило пополнения: символ добавляется, только если нужен ≥10 файлам.
//! Всё доменное (Pawn, Movable, ArcNavmesh, UI-хелперы) импортируется по месту.

pub use bevy::prelude::*;

pub use crate::{
    config::{
        ITEM_Z_INDEX, NIGHT_Z_INDEX, PAWN_Z_INDEX, PROP_Z_INDEX, STRUCTURE_Z_INDEX, TILE_Z_INDEX,
        config,
    },
    state::AppState,
};

// Макросы крейта: `#[macro_export]` кладёт их в корень, но после снятия
// плоских глобов (4.5) неквалифицированные вызовы перестанут резолвиться —
// поэтому реэкспорт здесь.
pub use crate::{
    continue_unless, ensure_state, exclusive_state_tags, log_event, log_state_change, return_unless,
};
