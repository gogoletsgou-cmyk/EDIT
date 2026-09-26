use macroquad::prelude::*;
use super::components::{self, INK, MUTED};
/// ロゴと検索欄を含む画面上部のヘッダーを描画する。
pub fn header(font: &Font) {
    let width = screen_width();
    components::panel(0.0, 0.0, width, 82.0);
    draw_circle(72.0, 41.0, 25.0, Color::new(0.94, 0.96, 0.97, 1.0));
    components::text(font, "Pe", 58.0, 51.0, 30, INK);
    components::text(font, "WIKIPEDIA GAME", 112.0, 37.0, 19, INK);
    components::text(font, "こんにちは！", 112.0, 59.0, 14, MUTED);
    let sx = width - 416.0;
    draw_rectangle(sx, 22.0, 360.0, 38.0, Color::new(0.97, 0.98, 0.98, 1.0));
    draw_rectangle_lines(sx, 22.0, 360.0, 38.0, 1.0, components::BORDER);
    components::text(font, "記事を検索", sx + 16.0, 46.0, 16, MUTED);
}
/// 画面中央に配置するコンテンツ領域の左端・上端・幅を返す。
pub fn contents_bounds() -> (f32, f32, f32) {
    let width = screen_width().min(1320.0);
    ((screen_width() - width) * 0.5, 124.0, width)
}
