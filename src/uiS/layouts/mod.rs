use macroquad::prelude::*;
use super::components::{self, INK, MUTED};
/// ロゴと検索欄を含む画面上部のヘッダーを描画する。
pub fn header(font: &Font) {
    let width = screen_width();
    components::panel(0.0, 0.0, width, 82.0);
    draw_circle(72.0, 41.0, 25.0, Color::new(0.94, 0.96, 0.97, 1.0));
    components::text(font, "KP", 58.0, 51.0, 30, INK);
    components::text(font, "KnowPedia", 112.0, 37.0, 19, INK);
    components::text(font, "フリー百科事典", 112.0, 59.0, 14, MUTED);
    let sx = width - 416.0;
    draw_rectangle(sx, 22.0, 360.0, 38.0, Color::new(0.97, 0.98, 0.98, 1.0));
    draw_rectangle_lines(sx, 22.0, 360.0, 38.0, 1.0, components::BORDER);
    components::text(font, "記事を検索", sx + 16.0, 46.0, 16, MUTED);

    draw_notification_bell(width - 28.0, 41.0);
}

/// Draw a small bell beside the search field and blink its notification badge.
fn draw_notification_bell(cx: f32, cy: f32) {
    let color = MUTED;
    let points = [
        (cx - 7.0, cy - 4.0),
        (cx - 6.0, cy - 9.0),
        (cx - 2.0, cy - 12.0),
        (cx + 2.0, cy - 12.0),
        (cx + 6.0, cy - 9.0),
        (cx + 7.0, cy - 4.0),
        (cx + 7.0, cy + 3.0),
        (cx + 10.0, cy + 7.0),
        (cx - 10.0, cy + 7.0),
        (cx - 7.0, cy + 3.0),
        (cx - 7.0, cy - 4.0),
    ];
    for pair in points.windows(2) {
        draw_line(pair[0].0, pair[0].1, pair[1].0, pair[1].1, 2.0, color);
    }
    draw_line(cx - 4.0, cy + 10.0, cx + 4.0, cy + 10.0, 2.0, color);

    // Show the red badge for half of each one-second cycle.
    if get_time() % 1.0 < 0.5 {
        draw_circle(cx + 8.0, cy - 10.0, 6.0, WHITE);
        draw_circle(cx + 8.0, cy - 10.0, 4.5, RED);
    }
}

/// 画面中央に配置するコンテンツ領域の左端・上端・幅を返す。
pub fn contents_bounds() -> (f32, f32, f32) {
    let width = screen_width().min(1320.0);
    ((screen_width() - width) * 0.5, 124.0, width)
}
