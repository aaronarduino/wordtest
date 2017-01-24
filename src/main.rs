#[macro_use] extern crate conrod;

extern crate piston_window;
use piston_window::*;

fn main() {
    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 720;

    let mut window: PistonWindow = WindowSettings::new("Hello World!", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("failed to build window: {0}", e));

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    
    let ids = Ids::new(ui.widget_id_generator());
    
    while let Some(event) = window.next() {
        window.draw_2d(&event, |_c, g| {
            clear([0.5, 1.0, 0.5, 1.0], g);
        });
    }
}

widget_ids!{
    struct Ids {
        master,
        left_col,
        middle_col,
        right_col,
        left_text,
        middle_text,
        right_text,
    }
}
