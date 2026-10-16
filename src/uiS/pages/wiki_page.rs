use macroquad::prelude::*;

use crate::ui::ui_screen::{components, layouts};
use super::article_page::ArticleId;
use components::{BLUE, INK, MUTED};

/// Wikipedia風のホーム画面と仮のアカウント情報を描画する。
pub fn draw(font: &Font) {
    clear_background(Color::new(0.97, 0.98, 0.98, 1.0));
    layouts::header(font);

    let (left, top, width) = layouts::contents_bounds();
    let gap = 28.0;
    let main_width = width * 0.66;
    let profile_x = left + main_width + gap;
    let profile_width = width - main_width - gap;
    let content_y = top + 16.0;

    draw_welcome(font, left, content_y, main_width);
    draw_account(font, profile_x, content_y, profile_width);
}

/// 記事の閲覧を中心に、編集もできることを紹介する本文欄を描画する。
fn draw_welcome(font: &Font, x: f32, y: f32, width: f32) {
    let height = 610.0;
    components::panel(x, y, width, height);
    let pad = 38.0;

    components::text(font, "KnowPediaへようこそ", x + pad, y + 58.0, 34, INK);
    components::text(font, "誰もが利用できるフリー百科事典", x + pad, y + 91.0, 18, BLUE);
    components::text(
        font,
        "知りたいことを調べたり、新しい知識を見つけたりできます。",
        x + pad,
        y + 137.0,
        17,
        INK,
    );
    components::text(
        font,
        "まずは記事を読んで、さまざまな分野の情報に触れてみましょう。",
        x + pad,
        y + 166.0,
        16,
        MUTED,
    );

    draw_line(x + pad, y + 202.0, x + width - pad, y + 202.0, 1.0, components::BORDER);

    components::text(font, "記事を読む", x + pad, y + 247.0, 25, INK);
    components::text(
        font,
        "記事を検索して、歴史・科学・文化など、気になるテーマを読んでみましょう。",
        x + pad,
        y + 282.0,
        16,
        MUTED,
    );

    let tile_y = y + 316.0;
    let tile_gap = 16.0;
    let tile_width = (width - pad * 2.0 - tile_gap * 2.0) / 3.0;
    let topics = [("自然", "生き物や宇宙"), ("歴史", "過去の出来事"), ("文化", "芸術や社会")];
    for (index, (title, description)) in topics.iter().enumerate() {
        let tile_x = x + pad + index as f32 * (tile_width + tile_gap);
        if draw_topic_tile(font, tile_x, tile_y, tile_width, title, description, index == 0) {
            ArticleId::AotsunoMorinezumi.open();
        }
    }

    draw_line(x + pad, y + 426.0, x + width - pad, y + 426.0, 1.0, components::BORDER);
    components::text(font, "記事を編集する", x + pad, y + 471.0, 23, INK);
    components::text(
        font,
        "誤りを見つけたときや情報を追加したいときは、記事を編集して共有できます。",
        x + pad,
        y + 506.0,
        16,
        MUTED,
    );
    components::text(
        font,
        "編集内容はほかの人にも確認でき、記事の改善に役立ちます。",
        x + pad,
        y + 534.0,
        16,
        MUTED,
    );
    components::text(font, "閲覧はいつでも自由に利用できます。", x + pad, y + 578.0, 14, BLUE);
}

/// 記事の例を並べ、閲覧が主な使い方だと伝える小さなカードを描画する。
fn draw_topic_tile(font: &Font, x: f32, y: f32, width: f32, title: &str, description: &str, clickable: bool) -> bool {
    let (mouse_x, mouse_y) = mouse_position();
    let hovered = clickable && mouse_x >= x && mouse_x <= x + width && mouse_y >= y && mouse_y <= y + 78.0;
    let background = if hovered {
        Color::new(0.93, 0.97, 1.0, 1.0)
    } else {
        Color::new(0.98, 0.99, 1.0, 1.0)
    };
    draw_rectangle(x, y, width, 78.0, background);
    draw_rectangle_lines(x, y, width, 78.0, 1.0, components::BORDER);
    components::text(font, title, x + 14.0, y + 29.0, 18, BLUE);
    components::text(font, description, x + 14.0, y + 55.0, 13, MUTED);
    hovered && is_mouse_button_pressed(MouseButton::Left)
}

/// 仮の初期アイコン、利用者名、支持者数、編集者の役職を表示する。
fn draw_account(font: &Font, x: f32, y: f32, width: f32) {
    let account = &crate::status::ACCOUNT;
    let height = 430.0;
    components::panel(x, y, width, height);
    components::text(font, "あなたのアカウント", x + 28.0, y + 43.0, 21, INK);

    let avatar_size = 112.0;
    let avatar_x = x + (width - avatar_size) * 0.5;
    let avatar_y = y + 70.0;
    draw_circle(
        avatar_x + avatar_size * 0.5,
        avatar_y + avatar_size * 0.5,
        avatar_size * 0.5,
        Color::new(0.91, 0.95, 0.97, 1.0),
    );
    draw_circle(
        avatar_x + avatar_size * 0.5,
        avatar_y + 39.0,
        19.0,
        Color::new(0.55, 0.65, 0.71, 1.0),
    );
    draw_ellipse(
        avatar_x + avatar_size * 0.5,
        avatar_y + 84.0,
        37.0,
        24.0,
        0.0,
        Color::new(0.55, 0.65, 0.71, 1.0),
    );

    let name = account.display_name;
    let name_width = measure_text(name, Some(font), 25, 1.0).width;
    components::text(font, name, x + (width - name_width) * 0.5, y + 222.0, 25, INK);

    let badge_width = 92.0;
    let badge_x = x + (width - badge_width) * 0.5;
    draw_rectangle(badge_x, y + 240.0, badge_width, 30.0, Color::new(0.91, 0.95, 0.99, 1.0));
    components::text(font, account.role, badge_x + 19.0, y + 261.0, 15, BLUE);

    draw_line(x + 28.0, y + 294.0, x + width - 28.0, y + 294.0, 1.0, components::BORDER);
    components::text(font, "支持者", x + 28.0, y + 331.0, 15, MUTED);
    components::text(font, &format!("{} 人", account.supporters), x + 28.0, y + 375.0, 27, INK);
    components::text(font, "編集への貢献を応援しています", x + 28.0, y + 405.0, 13, MUTED);
}
