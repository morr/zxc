use super::*;

#[derive(Component, Default)]
pub struct UserSelectionMarker;

#[derive(Event, Debug, Default)]
pub struct UserSelectionChangeEvent;

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct CurrentUserSelection(pub Option<UserSelectionData>);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum UserSelectionKind {
    Pawn,
    Farm,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct UserSelectionData {
    pub id: Entity,
    pub kind: UserSelectionKind,
}
