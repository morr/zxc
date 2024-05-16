use super::*;

#[derive(Component, Default)]
pub struct UserSelectionMarker;

#[derive(Event, Debug, Default)]
pub struct UserSelectionEvent;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct UserSelection(pub Option<UserSelectionData>);

pub enum SelectionKind {
    Pawn,
    Farm,
}

pub struct UserSelectionData {
    pub id: Entity,
    pub kind: SelectionKind,
}
