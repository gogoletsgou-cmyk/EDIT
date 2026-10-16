//! 文字列リンク。
//!
//! `LINKS` に登録した文字列は、`components::text` / `components::text_lines` で描くと
//! どのページでも自動的にリンクになる。描画側でリンクかどうかを指定する必要はない。
//!
//! - リンク文字列は常に青文字で描き、カーソルを重ねると明るい青に変わる。
//! - 折り返しで行をまたいだ文字列も、一つのリンクとして判定する。
//! - 表示中のページへのリンク（`set_current_page` で設定）は普通の文字として描く。
//! - クリックされた移動先は `take_clicked_link` で 1 フレームに 1 回取り出し、ページを切り替える。
//! - スクロールする領域では `with_link_area` で反応する縦の範囲を絞る。
//! - リンク以外（ボタンやカードなど）からページを開くときは `open_page` を呼ぶ。
//!
//! 新しいリンクを追加するときは、`LINKS` に「文字列と移動先」の組を足すだけでよい。

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

/// 平常時のリンク文字色。
pub const LINK: Color = Color::new(0.02, 0.27, 0.68, 1.0);
/// カーソルを重ねたときのリンク文字色。
pub const LINK_HOVER: Color = Color::new(0.0, 0.47, 0.90, 1.0);

/// 行を連結した文字列の中で見つかったリンク一つ分。start..end はバイト範囲。
#[derive(Clone, Copy)]
struct LinkSpan {
    start: usize,
    end: usize,
    target: LinkTarget,
}

thread_local! {
    static CLICKED: Cell<Option<LinkTarget>> = const { Cell::new(None) };
    static LINK_AREA: Cell<Option<(f32, f32)>> = const { Cell::new(None) };
    static CURRENT: Cell<Option<LinkTarget>> = const { Cell::new(None) };
}

/// このフレームでクリックされたリンクの移動先を取り出す。
pub fn take_clicked_link() -> Option<LinkTarget> {
    CLICKED.take()
}

/// リンクをクリックしたときと同じように、指定したページへ移動する。どこからでも呼べる。
/// 実際の切り替えはフレームの最後に `take_clicked_link` で行われる。
pub fn open_page(target: LinkTarget) {
    CLICKED.set(Some(target));
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
/// リンクは青文字、カーソルを重ねたリンクは明るい青で描く。それ以外は `color` で描く。
pub fn draw_lines<S: AsRef<str>>(
    font: &Font,
    lines: &[S],
    x: f32,
    first_baseline: f32,
    size: u16,
    line_height: f32,
    color: Color,
) {
    let joined: String = lines.iter().map(AsRef::as_ref).collect();
    let spans = find_links(&joined);
    let hovered = hovered_link(font, lines, &spans, x, first_baseline, size, line_height);
    if let (Some(index), true) = (hovered, is_mouse_button_pressed(MouseButton::Left)) {
        CLICKED.set(Some(spans[index].target));
    }

    let mut line_start = 0;
    for (index, line) in lines.iter().enumerate() {
        let line = line.as_ref();
        let baseline = first_baseline + index as f32 * line_height;
        let mut segment_x = x;
        let mut cursor = 0;
        for (span_index, span) in spans.iter().enumerate() {
            let Some((start, end)) = local_range(line_start, line, span.start, span.end) else {
                continue;
            };
            let link_color = if hovered == Some(span_index) { LINK_HOVER } else { LINK };
            for (part, part_color) in [(&line[cursor..start], color), (&line[start..end], link_color)] {
                draw_segment(font, part, segment_x, baseline, size, part_color);
                segment_x += measure_text(part, Some(font), size, 1.0).width;
            }
            cursor = end;
        }
        draw_segment(font, &line[cursor..], segment_x, baseline, size, color);
        line_start += line.len();
    }
}

/// 連結文字列の中にあるリンクを先頭から順に探す。表示中のページへのリンクは除く。
/// 範囲が重なった場合は先に始まる方を優先する。
fn find_links(joined: &str) -> Vec<LinkSpan> {
    let current = CURRENT.get();
    let mut spans: Vec<LinkSpan> = LINKS
        .iter()
        .filter(|(_, target)| Some(*target) != current)
        .flat_map(|(word, target)| {
            joined
                .match_indices(word)
                .map(move |(start, _)| LinkSpan { start, end: start + word.len(), target: *target })
        })
        .collect();
    spans.sort_by_key(|span| span.start);
    let mut last_end = 0;
    spans.retain(|span| {
        let keep = span.start >= last_end;
        if keep {
            last_end = span.end;
        }
        keep
    });
    spans
}

fn draw_segment(font: &Font, value: &str, x: f32, y: f32, size: u16, color: Color) {
    if !value.is_empty() {
        draw_text_ex(value, x, y, TextParams { font: Some(font), font_size: size, color, ..Default::default() });
    }
}

/// カーソルの下にあるリンクを、`spans` 内の番号として返す。
fn hovered_link<S: AsRef<str>>(
    font: &Font,
    lines: &[S],
    spans: &[LinkSpan],
    x: f32,
    first_baseline: f32,
    size: u16,
    line_height: f32,
) -> Option<usize> {
    let (mouse_x, mouse_y) = mouse_position();
    if let Some((top, bottom)) = LINK_AREA.get() {
        if mouse_y < top || mouse_y > bottom {
            return None;
        }
    }

    for (span_index, span) in spans.iter().enumerate() {
        let mut line_start = 0;
        for (index, line) in lines.iter().enumerate() {
            let line = line.as_ref();
            if let Some((local_start, local_end)) = local_range(line_start, line, span.start, span.end) {
                let baseline = first_baseline + index as f32 * line_height;
                let left = x + measure_text(&line[..local_start], Some(font), size, 1.0).width;
                let width = measure_text(&line[local_start..local_end], Some(font), size, 1.0).width;
                if mouse_x >= left
                    && mouse_x <= left + width
                    && mouse_y >= baseline - size as f32
                    && mouse_y <= baseline + size as f32 * 0.25
                {
                    return Some(span_index);
                }
            }
            line_start += line.len();
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
