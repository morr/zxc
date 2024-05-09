use super::*;

pub fn render_belly_ui(mut commands: Commands) {
    // commands.spawn(Camera2dBundle::default());
    commands.add(StyleSheet::load("stylesheets/ui.ess"));
    let label = commands.spawn_empty().id();

    commands.add(eml! {
        <body>
            <div c:status-area-container>
                <div c:status-area>
                    <label {label}/>
                </div>
            </div>
        </body>
    });
}
