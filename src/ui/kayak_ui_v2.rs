use super::*;

pub fn setup_kayak_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<MainCamera>>,
) {
    let camera_entity = query.get_single().unwrap();
    commands.entity(camera_entity).insert(CameraUIKayak);

    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}
