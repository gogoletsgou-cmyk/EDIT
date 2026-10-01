use macroquad::prelude::*;
use std::cell::Cell;

use super::super::pages::article_page::ArticleId;

/// リンク文字列をクリックしたときの移動先。
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LinkTarget {
    Home,
    Article(ArticleId),
}

/// どのページでもリンクとして扱う文字列と、その移動先の一覧。
const LINKS: &[(&str, LinkTarget)] = &[
    ("KnowPedia", LinkTarget::Home),
    ("アオツノモリネズミ", LinkTarget::Article(ArticleId::AotsunoMorinezumi)),
    ("シロモヤ山林", LinkTarget::Article(ArticleId::ShiromoyaSanrin)),
];

/// カーソルを重ねたときのリンク文字色。
pub const LINK_HOVER: Color = Color::new(0.0, 0.47, 0.90, 1.0);

thread_local! {
    static CLICKED: Cell<Option<LinkTarget>> = const { Cell::new(None) };
    static LINK_AREA: Cell<Option<(f32, f32)>> = const { Cell::new(None) };
    static CURRENT: Cell<Option<LinkTarget>> = const { Cell::new(None) };
}

/// このフレームでクリックされたリンクの移動先を取り出す。
pub fn take_clicked_link() -> Option<LinkTarget> {
    CLICKED.take()
}

/// 表示中のページを設定する。このページへのリンクは普通の文字として扱う。
pub fn set_current_page(target: LinkTarget) {
    CURRENT.set(Some(target));
}

/// `draw` の中で描いたリンクは、画面の縦方向 top..bottom の範囲でだけ反応する。
/// スクロールしてヘッダーの裏に隠れた文字が反応しないようにするために使う。
pub fn with_link_area<R>(top: f32, bottom: f32, draw: impl FnOnce() -> R) -> R {
    let previous = LINK_AREA.replace(Some((top, bottom)));
    let result = draw();
    LINK_AREA.set(previous);
    result
}

/// 折り返し済みの行を描画する。行をまたいだリンク文字列も一続きとして判定し、
/// カーソルを重ねたリンクだけ色を変える。それ以外は通常の文字と同じに描く。
pub fn draw_lines<S: AsRef<str>>(
    font: &Font,
    lines: &[S],
    x: f32,
    first_baseline: f32,
    size: u16,
    line_height: f32,
    color: Color,
) {
    let hovered = hovered_link(font, lines, x, first_baseline, size, line_height);
    if let (Some((_, _, target)), true) = (hovered, is_mouse_button_pressed(MouseButton::Left)) {
        CLICKED.set(Some(target));
    }

    let mut line_start = 0;
    for (index, line) in lines.iter().enumerate() {
        let line = line.as_ref();
        let baseline = first_baseline + index as f32 * line_height;
        match hovered.and_then(|(start, end, _)| local_range(line_start, line, start, end)) {
            Some((start, end)) => {
                let mut segment_x = x;
                for (part, part_color) in [
                    (&line[..start], color),
                    (&line[start..end], LINK_HOVER),
                    (&line[end..], color),
                ] {
                    draw_segment(font, part, segment_x, baseline, size, part_color);
                    segment_x += measure_text(part, Some(font), size, 1.0).width;
                }
            }
            None => draw_segment(font, line, x, baseline, size, color),
        }
        line_start += line.len();
    }
}

fn draw_segment(font: &Font, value: &str, x: f32, y: f32, size: u16, color: Color) {
    if !value.is_empty() {
        draw_text_ex(value, x, y, TextParams { font: Some(font), font_size: size, color, ..Default::default() });
    }
}

/// カーソルの下にあるリンクを、行を連結した文字列でのバイト範囲として返す。
fn hovered_link<S: AsRef<str>>(
    font: &Font,
    lines: &[S],
    x: f32,
    first_baseline: f32,
    size: u16,
    line_height: f32,
) -> Option<(usize, usize, LinkTarget)> {
    let (mouse_x, mouse_y) = mouse_position();
    if let Some((top, bottom)) = LINK_AREA.get() {
        if mouse_y < top || mouse_y > bottom {
            return None;
        }
    }

    let joined: String = lines.iter().map(AsRef::as_ref).collect();
    let current = CURRENT.get();
    for (word, target) in LINKS.iter().filter(|(_, target)| Some(*target) != current) {
        for (start, _) in joined.match_indices(word) {
            let end = start + word.len();
            let mut line_start = 0;
            for (index, line) in lines.iter().enumerate() {
                let line = line.as_ref();
                if let Some((local_start, local_end)) = local_range(line_start, line, start, end) {
                    let baseline = first_baseline + index as f32 * line_height;
                    let left = x + measure_text(&line[..local_start], Some(font), size, 1.0).width;
                    let width = measure_text(&line[local_start..local_end], Some(font), size, 1.0).width;
                    if mouse_x >= left
                        && mouse_x <= left + width
                        && mouse_y >= baseline - size as f32
                        && mouse_y <= baseline + size as f32 * 0.25
                    {
                        return Some((start, end, *target));
                    }
                }
                line_start += line.len();
            }
        }
    }
    None
}

/// 連結文字列での範囲 start..end のうち、この行に含まれる部分を行内のバイト範囲で返す。
fn local_range(line_start: usize, line: &str, start: usize, end: usize) -> Option<(usize, usize)> {
    let line_end = line_start + line.len();
    let local_start = start.max(line_start);
    let local_end = end.min(line_end);
    (local_start < local_end).then(|| (local_start - line_start, local_end - line_start))
}
