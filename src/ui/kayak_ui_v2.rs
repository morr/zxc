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
    // rsx! {
    //     <KayakAppBundle>
    //         <TextWidgetBundle
    //             text={TextProps {
    //                 content: "Hello World".into(),
    //                 size: 20.0,
    //                 ..Default::default()
    //             }}
    //         />
    //     </KayakAppBundle>
    // };

    // Color::hex("181a1c").unwrap().set_a(0.5).into()
    // let a = Color::hex("181a1c").unwrap();
    // let b = Color::hex("181a1c").unwrap().set_a(0.5);
    //
    // let z = KStyle {
    //     // background_color: Color::rgba(1.0, 1.0, 1.0, 0.25).into(),
    //     background_color: (*Color::hex("181a1c").unwrap().set_a(0.5)).into(),
    //     height: Units::Pixels(100.0).into(),
    //     right: Units::Pixels(0.0).into(),
    //     top: Units::Pixels(0.0).into(),
    //     width: Units::Pixels(100.0).into(),
    //     offset:
    //     ..Default::default()
    // };

    rsx! {
        <KayakAppBundle>
            <BackgroundBundle
                styles={KStyle {
                    background_color: (*Color::hex("181a1c").unwrap().set_a(0.75)).into(),
                    height: Units::Pixels(100.0).into(),
                    left: Units::Stretch(1.0).into(),
                    padding_left: Units::Pixels(30.0).into(),
                    padding_top: Units::Pixels(60.0).into(),
                    width: Units::Pixels(272.0).into(),
                    ..Default::default()
                }}
            >
                <TextWidgetBundle
                    text={TextProps {
                        content: "Hello World".into(),
                        size: 20.0,
                        ..Default::default()
                    }}
                />
            </BackgroundBundle>
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}

// pub fn update_kayak_ui(time_scale: Res<TimeScale>, mut query: Query<&mut TextProps>) {
//     if let Ok(mut text_props) = query.get_single_mut() {
//         text_props.content = format!("Time Scale: {:.2}", time_scale.0);
//     }
// }
