use super::*;

#[derive(Component, Default)]
pub struct UserSelectionMarker;

#[derive(Event, Debug, Default)]
pub struct UserSelectionEvent;

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct UserSelection(pub Option<UserSelectionData>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SelectionKind {
    Pawn,
    Farm,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct UserSelectionData {
    pub id: Entity,
    pub kind: SelectionKind,
}
