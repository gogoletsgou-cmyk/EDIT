use macroquad::prelude::*;

use crate::ui::ui_screen::{components, layouts};
use components::{BLUE, INK, MUTED};

/// Draw a sample animal article with sections sized to their wrapped text.
pub fn draw(font: &Font) -> bool {
    clear_background(Color::new(0.97, 0.98, 0.98, 1.0));
    layouts::header(font);

    let (left, top, width) = layouts::contents_bounds();
    let back_y = top + 20.0;
    let (mouse_x, mouse_y) = mouse_position();
    let back_hovered = mouse_x >= left && mouse_x <= left + 190.0
        && mouse_y >= back_y - 24.0 && mouse_y <= back_y + 8.0;
    components::text(
        font,
        "← ホームへ戻る",
        left,
        back_y,
        16,
        if back_hovered { INK } else { BLUE },
    );
    if back_hovered && is_mouse_button_pressed(MouseButton::Left) {
        return true;
    }

    let panel_y = top + 38.0;
    draw_article(font, left, panel_y, width);
    false
}

/// Wrap article paragraphs, calculate the required panel height, then draw each section.
fn draw_article(font: &Font, x: f32, y: f32, width: f32) {
    let pad = 34.0;
    let text_width = width - pad * 2.0;
    let info_width = (width * 0.34).max(220.0).min(width - pad * 2.0);
    let info_x = x + width - pad - info_width;
    let info_y = y + 104.0;
    let title = "アオツノモリネズミ";
    let introduction = "アオツノモリネズミは、湿った山林に生息する小型の夜行性哺乳類である。長い尾と青みがかった耳が特徴とされる。この記事はレイアウト見本用の架空の動物を紹介している。";
    let sections = [
        (
            "特徴",
            "体長はおよそ18センチメートルで、落ち葉の下や倒木のすき間に巣を作る。耳の先には青灰色の短い毛が生えている。",
        ),
        (
            "生態",
            "夜間に活動し、木の実や小さな昆虫を食べる。危険を感じると、尾を地面につけて音を立てずに茂みへ隠れる。",
        ),
        (
            "観察",
            "人の気配を避けるため、野外で見つけるのは難しい。観察記録では、雨のあとに林床で活動する様子が多く報告されている。",
        ),
    ];
    // The lead paragraph uses the remaining left column beside the infobox.
    let info_height = information_box_height(font, info_width);
    let info_bottom = info_y + info_height;
    let narrow_width = text_width - info_width - 24.0;
    let intro_lines = wrap_text_around_infobox(
        font,
        introduction,
        narrow_width,
        text_width,
        y + 129.0,
        25.0,
        info_bottom,
        16,
    );
    // Continue sections beside the infobox, then use the full article width below it.
    let section_start_y = y + 129.0 + intro_lines.len() as f32 * 25.0 + 27.0;
    let mut section_y = section_start_y;
    let mut section_lines = Vec::new();
    for (heading, body) in &sections {
        let lines = wrap_text_around_infobox(
            font,
            body,
            narrow_width,
            text_width,
            section_y + 51.0,
            25.0,
            info_bottom,
            16,
        );
        section_lines.push((*heading, lines));
        section_y += 42.0 + section_lines.last().unwrap().1.len() as f32 * 25.0 + 20.0;
    }
    // The article panel owns both the text flow and the infobox.  Its bottom
    // must therefore cover whichever child extends farther down.
    let content_bottom = section_y.max(info_bottom);
    let panel_height = content_bottom - y + 12.0;
    components::panel(x, y, width, panel_height);
    components::text(font, title, x + pad, y + 57.0, 32, INK);
    components::text(font, "自然 / 動物", x + pad, y + 88.0, 15, BLUE);
    draw_wrapped_lines(font, &intro_lines, x + pad, y + 129.0, 16, 25.0, INK);
    draw_information_box(font, info_x, info_y, info_width);

    section_y = section_start_y;
    for (heading, lines) in &section_lines {
        let beside_infobox = section_y + 17.0 < info_bottom;
        let section_width = if beside_infobox { narrow_width } else { text_width };
        let section_x = x + pad;
        draw_line(section_x, section_y - 10.0, section_x + section_width, section_y - 10.0, 1.0, components::BORDER);
        components::text(font, heading, section_x, section_y + 17.0, 22, INK);
        draw_wrapped_lines(font, lines, section_x, section_y + 51.0, 16, 25.0, MUTED);
        section_y += 42.0 + lines.len() as f32 * 25.0 + 20.0;
    }
}

/// Draw the animal's sample illustration and dynamically sized fact rows.
fn information_facts() -> [(&'static str, &'static str); 5] {
    [
        ("分類", "哺乳類・齧歯類"),
        ("生息地", "霧の多い山林"),
        ("体長", "約18 cm"),
        ("活動時間", "夜行性"),
        ("食べ物", "木の実・昆虫"),
    ]
}

fn information_box_height(font: &Font, width: f32) -> f32 {
    let inner_width = (width - 36.0).max(1.0);
    let title_lines = wrap_text(font, "アオツノモリネズミ", inner_width, 18);
    let image_y = 30.0 + title_lines.len() as f32 * 22.0;
    let fact_rows_height: f32 = information_facts()
        .iter()
        .map(|(_, value)| {
            let value_lines = wrap_text(font, value, inner_width, 16);
            (54.0_f32).max(28.0 + value_lines.len() as f32 * 20.0)
        })
        .sum();
    image_y + 158.0 + 26.0 + fact_rows_height + 36.0
}

fn draw_information_box(font: &Font, x: f32, y: f32, width: f32) {
    let facts = information_facts();
    let inner_width = (width - 36.0).max(1.0);
    let title_lines = wrap_text(font, "アオツノモリネズミ", inner_width, 18);
    let image_y_offset = 30.0 + title_lines.len() as f32 * 22.0;
    let height = information_box_height(font, width);
    components::panel(x, y, width, height);
    draw_wrapped_lines(font, &title_lines, x + 18.0, y + 34.0, 18, 22.0, INK);

    let image_x = x + 18.0;
    let image_y = y + image_y_offset;
    let image_width = width - 36.0;
    let image_height = 158.0;
    draw_rectangle(image_x, image_y, image_width, image_height, Color::new(0.94, 0.96, 0.94, 1.0));
    draw_rectangle_lines(image_x, image_y, image_width, image_height, 1.0, components::BORDER);
    draw_animal_placeholder(image_x, image_y, image_width, image_height);
    components::text(font, "架空の動物・図版見本", image_x + 10.0, image_y + image_height - 10.0, 12, MUTED);

    let mut row_y = image_y + image_height + 26.0;
    for (index, (label, value)) in facts.iter().enumerate() {
        let value_lines = wrap_text(font, value, inner_width, 16);
        let row_height = 54.0_f32.max(28.0 + value_lines.len() as f32 * 20.0);
        components::text(font, label, x + 18.0, row_y, 14, MUTED);
        draw_wrapped_lines(font, &value_lines, x + 18.0, row_y + 23.0, 16, 20.0, INK);
        row_y += row_height;
        if index + 1 < facts.len() {
            draw_line(x + 18.0, row_y - 11.0, x + width - 18.0, row_y - 11.0, 1.0, components::BORDER);
        }
    }
}

/// Draw a simple vector animal placeholder inside the article's figure box.
fn draw_animal_placeholder(x: f32, y: f32, width: f32, height: f32) {
    let center_x = x + width * 0.53;
    let center_y = y + height * 0.57;
    let fur = Color::new(0.56, 0.52, 0.45, 1.0);
    let blue = Color::new(0.31, 0.55, 0.69, 1.0);
    draw_ellipse(center_x, center_y, 108.0, 64.0, 0.0, fur);
    draw_circle(center_x + 38.0, center_y - 27.0, 29.0, fur);
    draw_circle(center_x + 22.0, center_y - 48.0, 12.0, blue);
    draw_circle(center_x + 52.0, center_y - 49.0, 12.0, blue);
    draw_circle(center_x + 49.0, center_y - 30.0, 3.0, BLACK);
    draw_circle(center_x + 66.0, center_y - 17.0, 4.0, Color::new(0.8, 0.5, 0.48, 1.0));
    draw_line(center_x - 46.0, center_y + 6.0, center_x - 76.0, center_y + 22.0, 3.0, fur);
    draw_line(center_x - 76.0, center_y + 22.0, center_x - 91.0, center_y + 17.0, 3.0, fur);
}

/// Split Japanese text at character boundaries so each line fits its available width.
fn wrap_text(font: &Font, text: &str, max_width: f32, size: u16) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for character in text.chars() {
        let mut candidate = current.clone();
        candidate.push(character);
        let candidate_width = measure_text(&candidate, Some(font), size, 1.0).width;
        if !current.is_empty() && candidate_width > max_width {
            lines.push(current);
            current = character.to_string();
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// Wrap text line by line, widening each line after it passes the infobox bottom.
fn wrap_text_around_infobox(
    font: &Font,
    text: &str,
    narrow_width: f32,
    full_width: f32,
    first_baseline: f32,
    line_height: f32,
    infobox_bottom: f32,
    size: u16,
) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();

    for character in text.chars() {
        let line_y = first_baseline + lines.len() as f32 * line_height;
        let max_width = if line_y < infobox_bottom { narrow_width } else { full_width };
        let mut candidate = current.clone();
        candidate.push(character);
        if !current.is_empty() && measure_text(&candidate, Some(font), size, 1.0).width > max_width {
            lines.push(current);
            current = character.to_string();
        } else {
            current = candidate;
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

/// Draw pre-wrapped lines and return their total height for the next layout item.
fn draw_wrapped_lines(
    font: &Font,
    lines: &[String],
    x: f32,
    first_baseline: f32,
    size: u16,
    line_height: f32,
    color: Color,
) -> f32 {
    for (index, line) in lines.iter().enumerate() {
        components::text(font, line, x, first_baseline + index as f32 * line_height, size, color);
    }
    lines.len() as f32 * line_height
}
