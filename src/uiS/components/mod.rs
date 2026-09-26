use macroquad::prelude::*;

pub const INK: Color = Color::new(0.12, 0.16, 0.20, 1.0);
pub const MUTED: Color = Color::new(0.40, 0.46, 0.51, 1.0);
pub const BORDER: Color = Color::new(0.84, 0.87, 0.89, 1.0);
pub const BLUE: Color = Color::new(0.20, 0.39, 0.62, 1.0);
pub const PAPER: Color = Color::new(1.0, 1.0, 1.0, 1.0);

/// 指定した位置・サイズ・色でテキストを描画する。
pub fn text(font: &Font, value: &str, x: f32, y: f32, size: u16, color: Color) {
    draw_text_ex(value, x, y, TextParams { font: Some(font), font_size: size, color, ..Default::default() });
}
/// 白い背景と枠線を持つパネルを描画する。
pub fn panel(x: f32, y: f32, width: f32, height: f32) {
    draw_rectangle(x, y, width, height, PAPER);
    draw_rectangle_lines(x, y, width, height, 1.0, BORDER);
}
/// ラベル付きの青いボタンを描画する。
pub fn button(font: &Font, label: &str, x: f32, y: f32, width: f32) {
    draw_rectangle(x, y, width, 48.0, BLUE);
    text(font, label, x + 20.0, y + 31.0, 18, PAPER);
}
/// 小さな丸角風ラベル（ピル）を描画する。
pub fn pill(font: &Font, label: &str, x: f32, y: f32) {
    draw_rectangle(x, y, 164.0, 30.0, Color::new(0.94, 0.96, 0.98, 1.0));
    text(font, label, x + 12.0, y + 20.0, 15, BLUE);
}
