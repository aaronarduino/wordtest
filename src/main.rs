#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use piston_window::*;
use conrod::backend::piston;

fn main() {
    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 720;

    let mut window: PistonWindow = WindowSettings::new("Hello World!", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("failed to build window: {0}", e));

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    
    let ids = Ids::new(ui.widget_id_generator());

    // Add a `Font` to the `Ui`'s `font::Map` from file.
    let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
    let noto_sans = assets.join("fonts/NotoSans");
    let regular = ui.fonts.insert_from_file(noto_sans.join("NotoSans-Regular.ttf")).unwrap();
    let italic = ui.fonts.insert_from_file(noto_sans.join("NotoSans-Italic.ttf")).unwrap();
    let bold = ui.fonts.insert_from_file(noto_sans.join("NotoSans-Bold.ttf")).unwrap();

    // Store our `font::Id`s in a list for easy access in the `set_ui` function.
    let font_ids = [regular, italic, bold];

    ui.theme.font_id = Some(regular);

    let mut text_texture_cache = conrod::backend::piston::gfx::GlyphCache::new(&mut window, WIDTH, HEIGHT);

    let image_map = conrod::image::Map::new();
    
    while let Some(event) = window.next() {

        event.update(|_| set_ui(ui.set_widgets(), &ids, &font_ids));

        window.draw_2d(&event, |c, g| {
            if let Some(primitives) = ui.draw_if_changed() {
                fn texture_from_image<T>(img: &T) -> &T {img}
                piston::window::draw(c, g, primitives,
                                     &mut text_texture_cache,
                                     &image_map,
                                     texture_from_image);
            }
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

fn set_ui(ref mut ui: conrod::UiCell, ids: &Ids, font_ids: &[conrod::text::font::Id; 3]) {
    use conrod::{color, widget, Colorable, Positionable, Scalar, Sizeable, Widget};

    // Our canvas tree
    widget::Canvas::new()
        .flow_right(&[
                    (ids.left_col, widget::Canvas::new().color(color::BLACK)),
                    (ids.middle_col, widget::Canvas::new().color(color::DARK_CHARCOAL)),
                    (ids.right_col, widget::Canvas::new().color(color::CHARCOAL)),
        ]).set(ids.master, ui);

    let demo_txt: &str = "HI how are you?";

    const PAD: Scalar = 20.0;

    const FONT_REGULAR: usize = 0;
    const FONT_ITALIC: usize = 1;
    const FONT_BOLD: usize = 2;

    widget::Text::new(demo_txt)
        .font_id(font_ids[FONT_REGULAR])
        .color(color::LIGHT_RED)
        .padded_w_of(ids.left_col, PAD)
        .mid_top_with_margin_on(ids.left_col, PAD)
        .align_text_left()
        .line_spacing(10.0)
        .set(ids.left_text, ui);

    widget::Text::new(demo_txt)
        .font_id(font_ids[FONT_ITALIC])
        .color(color::LIGHT_GREEN)
        .padded_w_of(ids.middle_col, PAD)
        .middle_of(ids.middle_col)
        .align_text_middle()
        .line_spacing(2.5)
        .set(ids.middle_text, ui);

    widget::Text::new(demo_txt)
        .font_id(font_ids[FONT_BOLD])
        .color(color::LIGHT_BLUE)
        .padded_w_of(ids.right_col, PAD)
        .mid_bottom_with_margin_on(ids.right_col, PAD)
        .align_text_right()
        .line_spacing(5.0)
        .set(ids.right_text, ui);
}
