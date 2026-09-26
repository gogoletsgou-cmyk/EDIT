use macroquad::prelude::*;
#[path = "uiS/mod.rs"]
mod ui_screen;

pub async fn load_font() -> Font {
    load_ttf_font("assets/fonts/PixelMplus12-Regular.ttf").await.expect("フォントを読み込めませんでした")
}
pub fn draw_ui(font: &Font) { ui_screen::pages::wiki_page::draw(font); }
