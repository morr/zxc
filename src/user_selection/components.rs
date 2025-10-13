use super::*;

#[derive(Component, Default)]
pub struct UserSelectionMarker;

#[derive(Event, Debug, Default)]
pub struct UserSelectionEvent;

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct CurrentUserSelection(pub Option<UserSelectionData>);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub enum UserSelectionKind {
    Pawn,
    Farm,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct UserSelectionData {
    pub entity: Entity,
    pub kind: UserSelectionKind,
}
