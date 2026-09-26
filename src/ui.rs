use macroquad::prelude::*;
#[path = "uiS/mod.rs"]
mod ui_screen;

pub enum Page {
    Home,
    Article,
}

pub async fn load_font() -> Font {
    load_ttf_font("assets/fonts/PixelMplus12-Regular.ttf").await.expect("フォントを読み込めませんでした")
}
pub fn draw_ui(font: &Font, page: &mut Page) {
    match page {
        Page::Home => {
            if ui_screen::pages::wiki_page::draw(font) {
                *page = Page::Article;
            }
        }
        Page::Article => {
            if ui_screen::pages::article_page::draw(font) {
                *page = Page::Home;
            }
        }
    }
}
