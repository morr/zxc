use bevy::{gizmos::gizmos::Gizmos, prelude::*, render::color::Color};

pub fn render_grid(mut gizmos: Gizmos) {
    let from = -99;
    let to = 99;

    for i in from..to {
        gizmos.line_2d(
            Vec2::new(from as f32, i as f32),
            Vec2::new(to as f32, i as f32),
            if i == 0 {
                Color::rgb(0.4, 0.4, 0.4)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            },
        );
        gizmos.line_2d(
            Vec2::new(i as f32, from as f32),
            Vec2::new(i as f32, to as f32),
            if i == 0 {
                Color::rgb(0.4, 0.4, 0.4)
            } else {
                Color::rgb(0.2, 0.2, 0.2)
            },
        );
    }
}
