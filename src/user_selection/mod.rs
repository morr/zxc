use crate::*;

expose_submodules!(components, systems);

pub struct UserSelectPlugin;

impl Plugin for UserSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UserSelectionEvent>()
            .init_resource::<UserSelection>()
            .add_systems(Update, find_new_selection_on_click.run_if(in_state(AppState::Playing)));
    }
}
