use macroquad::prelude::*;

use crate::ui::ui_screen::{components, layouts};
use components::{BLUE, INK, MUTED};

pub fn draw(font: &Font) {
    clear_background(Color::new(
        0.96, 
        0.97, 
        0.97, 
        1.0)
    );

    layouts::header(font);

    let (left, top, width) = layouts::contents_bounds();

    components::text(
        font, 
        "今日のチャレンジ", 
        left, 
        top, 
        16, 
        BLUE
    );
    components::text(
        font, 
        "リンクをたどって、目的の記事へ。", 
        left, 
        top + 43.0, 
        32, 
        INK
    );
    components::text(
        font, 
        "Wikipediaの記事にあるリンクだけを使ってゴールを目指そう。",
         left, 
         top + 73.0, 
         17, 
         MUTED
        );
    let gap = 24.0;

    let main_w = width * 0.66;

    let side_x = left + main_w + gap; 
    let side_w = width - main_w - gap;

    let y = top + 108.0;

    components::panel(
        left, 
        y, 
        main_w, 
        402.0
    ); 
    components::panel(
        side_x, 
        y, 
        side_w,
         402.0
    );
    components::text(
        font, 
        "今回のルート", 
        left + 30.0, 
        y + 42.0, 
        20, 
        INK
    );
    components::pill(
        font, 
        "制限時間  05:00", 
        left + main_w - 194.0, 
        y + 20.0
    );

    let nx = left + 52.0;
    let nw = main_w - 104.0;

    draw_rectangle(
        nx, 
        y + 82.0, 
        nw, 92.0, 
        Color::new(0.98, 0.985, 0.99, 1.0)
    );

    draw_rectangle_lines(
        nx, 
        y + 82.0, 
        nw, 92.0, 
        1.0, 
        components::BORDER
    );

    components::text(
        font, 
        "START  はじまりの記事", 
        nx + 20.0, 
        y + 111.0, 
        14, 
        MUTED
    );
    components::text(
        font, 
        "太陽系", 
        nx + 20.0, 
        y + 151.0, 
        27, 
        INK
    );
    draw_line(
        left + main_w * 0.5, 
        y + 174.0, 
        left + main_w * 0.5, 
        y + 214.0, 
        2.0, 
        components::BORDER
    );

    draw_circle(left + main_w * 0.5, 
        y + 194.0, 
        5.0, 
        BLUE
    );

    draw_rectangle(
        nx, 
        y + 214.0, 
        nw,
        92.0, 
        Color::new(0.98, 0.985, 0.99, 1.0)
    );

    draw_rectangle_lines(
        nx, 
        y + 214.0, 
        nw, 
        92.0, 
        1.0, 
        components::BORDER
    );
    components::text(
        font, 
        "GOAL  たどり着く記事", 
        nx + 20.0, 
        y + 243.0, 
        14, 
        MUTED
    );
    components::text(
        font, 
        "ルネサンス", 
        nx + 20.0, 
        y + 283.0, 
        27, 
        INK
    );
    components::button(
        font, 
        "ゲームを始める   →", 
        nx, 
        y + 328.0,
        nw
        );
    components::text(
        font, 
        "遊び方", 
        side_x + 26.0, 
        y + 42.0, 
        20, 
        INK
    );
    components::text(
        font, 
        "1   スタートの記事から開始", 
        side_x + 26.0, 
        y + 91.0, 
        15, 
        INK
    );
    components::text(
        font, 
        "2   本文中のリンクを選ぶ",
         side_x + 26.0, 
         y + 137.0, 
         15, 
         INK
    );
    components::text(
        font, 
        "3   ゴールの記事を目指そう", 
        side_x + 26.0, 
        y + 183.0, 
        15, 
        INK
    );
    draw_line(
        side_x + 26.0, 
        y + 222.0, 
        side_x + side_w - 26.0, 
        y + 222.0, 
        1.0, 
        components::BORDER
    );
    components::text(
        font, 
        "ベストタイム", 
        side_x + 26.0, 
        y + 258.0, 
        14, 
        MUTED
    );
    components::text(
        font, 
        "--:--", 
        side_x + 26.0, 
        y + 299.0, 
        26, 
        INK
    );
    components::text(
        font, 
        "今日のプレイ", 
        side_x + side_w * 0.53, 
        y + 258.0, 
        14, 
        MUTED
    );
    components::text(
        font, 
        "0 回", 
        side_x + side_w * 0.53, 
        y + 299.0, 
        26, 
        INK
    );
    components::text(
        font, 
        "Wikipediaの記事を読みながら、知識のつながりを見つけよう。", 
        left, 
        y + 436.0, 
        15, 
        MUTED
    );
}
