mod ui;
mod status;
#[path = "debugs"]
mod debugs;

use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "EDIT".to_owned(),
        window_width: 1920,
        window_height: 1080,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let font = ui::load_font().await;
    let mut page = ui::Page::Home;
    let mut debug_page = debugs::DebugPage::default();

    loop {
        ui::draw_ui(&font, &mut page);
        debug_page.update();
        debug_page.draw(&font);

        next_frame().await;
    }
}
