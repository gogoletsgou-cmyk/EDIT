use macroquad::prelude::*;
#[path = "uiS/mod.rs"]
mod ui_screen;

use ui_screen::components::{self, LinkTarget};
use ui_screen::pages::article_page::{self, ArticleId, ArticlePageState};

pub enum Page {
    Home,
    Article(ArticleId, ArticlePageState),
}

pub async fn load_font() -> Font {
    load_ttf_font("assets/fonts/PixelMplus12-Regular.ttf").await.expect("フォントを読み込めませんでした")
}
pub fn draw_ui(font: &Font, page: &mut Page) {
    components::set_current_page(match page {
        Page::Home => LinkTarget::Home,
        Page::Article(id, _) => LinkTarget::Article(*id),
    });

    match page {
        Page::Home => {
            if ui_screen::pages::wiki_page::draw(font) {
                *page = Page::Article(ArticleId::AotsunoMorinezumi, Default::default());
            }
        }
        Page::Article(id, state) => {
            if article_page::draw(font, *id, state) {
                *page = Page::Home;
            }
        }
    }

    // Follow a text link clicked anywhere on the page. Links to the current page never fire.
    match components::take_clicked_link() {
        Some(LinkTarget::Home) => *page = Page::Home,
        Some(LinkTarget::Article(id)) => *page = Page::Article(id, Default::default()),
        None => {}
    }
}
