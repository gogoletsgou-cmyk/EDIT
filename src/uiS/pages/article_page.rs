use macroquad::prelude::*;

use crate::ui::ui_screen::{components, layouts};
use components::{BLUE, INK, MUTED};

/// Content required to draw an article page.
pub struct Article<'a> {
    pub title: &'a str,
    pub category: &'a str,
    pub introduction: &'a str,
    pub sections: &'a [ArticleSection<'a>],
    pub infobox: ArticleInfobox<'a>,
}

/// A named body section in an article.
pub struct ArticleSection<'a> {
    pub heading: &'a str,
    pub body: &'a str,
}

/// A label and value shown in an article's infobox.
pub struct InfoboxFact<'a> {
    pub label: &'a str,
    pub value: &'a str,
}

/// Content and illustration callback for the right-hand infobox.
pub struct ArticleInfobox<'a> {
    pub title: &'a str,
    pub image_caption: &'a str,
    pub facts: &'a [InfoboxFact<'a>],
    pub draw_image: fn(f32, f32, f32, f32),
}

/// Per-page state that preserves the current vertical scroll position.
#[derive(Default)]
pub struct ArticlePageState {
    scroll_y: f32,
}

/// Draw the sample animal article used while the page is under development.
pub fn draw(font: &Font, state: &mut ArticlePageState) -> bool {
    let sections = [
        ArticleSection {
            heading: "特徴",
            body: "体長はおよそ18センチメートルで、落ち葉の下や倒木のすき間に巣を作る。耳の先には青灰色の短い毛が生えている。",
        },
        ArticleSection {
            heading: "生態",
            body: "夜間に活動し、木の実や小さな昆虫を食べる。危険を感じると、尾を地面につけて音を立てずに茂みへ隠れる。",
        },
        ArticleSection {
            heading: "観察",
            body: "人の気配を避けるため、野外で見つけるのは難しい。観察記録では、雨のあとに林床で活動する様子が多く報告されている。",
        },
        ArticleSection {
            heading:"宇おおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおお",
            body: "宇おおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおおお"
        }
    ];
    let facts = [
        InfoboxFact { label: "分類", value: "哺乳類・齧歯類" },
        InfoboxFact { label: "生息地", value: "霧の多い山林" },
        InfoboxFact { label: "体長", value: "約18 cm" },
        InfoboxFact { label: "活動時間", value: "夜行性" },
        InfoboxFact { label: "食べ物", value: "木の実・昆虫" },
        InfoboxFact { label: "ごめんなさい", value: "それはトムです"},
    ];
    let article = Article {
        title: "アオツノモリネズミ",
        category: "自然 / 動物",
        introduction: "アオツノモリネズミは、湿った山林に生息する小型の夜行性哺乳類である。長い尾と青みがかった耳が特徴とされる。この記事はレイアウト見本用の架空の動物を紹介している。",
        sections: &sections,
        infobox: ArticleInfobox {
            title: "アオツノモリネズミ",
            image_caption: "架空の動物・図版見本",
            facts: &facts,
            draw_image: draw_animal_placeholder,
        },
    };
    draw_article_page(font, &article, state)
}

/// Draw any article with a wrapped right-hand infobox. Returns true when back is clicked.
pub fn draw_article_page(font: &Font, article: &Article<'_>, state: &mut ArticlePageState) -> bool {
    clear_background(Color::new(0.97, 0.98, 0.98, 1.0));

    let (left, top, width) = layouts::contents_bounds();
    let panel_y = top + 38.0;
    let viewport_bottom = screen_height() - 20.0;
    let panel_height = article_panel_height(font, width, article);
    let max_scroll = (panel_height - (viewport_bottom - panel_y)).max(0.0);
    let (_, wheel_y) = mouse_wheel();
    let (_, mouse_y) = mouse_position();
    if mouse_y >= panel_y && mouse_y <= viewport_bottom {
        state.scroll_y = (state.scroll_y - wheel_y * 48.0).clamp(0.0, max_scroll);
    } else {
        state.scroll_y = state.scroll_y.clamp(0.0, max_scroll);
    }

    draw_article(font, left, panel_y - state.scroll_y, width, article);

    // Keep the header and navigation area fixed while article content moves beneath it.
    draw_rectangle(0.0, 82.0, screen_width(), panel_y - 82.0, Color::new(0.97, 0.98, 0.98, 1.0));
    layouts::header(font);
    draw_scrollbar(panel_y, viewport_bottom, panel_height, state.scroll_y, max_scroll);
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

    false
}

/// Draw the right-side scroll indicator only when the article exceeds the viewport.
fn draw_scrollbar(
    viewport_top: f32,
    viewport_bottom: f32,
    content_height: f32,
    scroll_y: f32,
    max_scroll: f32,
) {
    if max_scroll <= 0.0 {
        return;
    }

    let track_height = viewport_bottom - viewport_top;
    let thumb_height = (track_height * track_height / content_height)
        .clamp(36.0, track_height);
    let thumb_y = viewport_top + (scroll_y / max_scroll) * (track_height - thumb_height);
    let x = screen_width() - 34.0;
    let bar_width = 8.0;

    draw_rectangle(x, viewport_top, bar_width, track_height, Color::new(0.82, 0.85, 0.87, 0.55));
    draw_rectangle(x, thumb_y, bar_width, thumb_height, Color::new(0.32, 0.43, 0.51, 0.85));
}

/// Measure the article's outer panel so scrolling includes both text and infobox.
fn article_panel_height(font: &Font, width: f32, article: &Article<'_>) -> f32 {
    let pad = 34.0;
    let text_width = width - pad * 2.0;
    let info_width = (width * 0.34).max(220.0).min(width - pad * 2.0);
    let info_bottom = 104.0 + information_box_height(font, info_width, &article.infobox);
    let narrow_width = text_width - info_width - 24.0;
    let intro_lines = wrap_text_around_infobox(
        font,
        article.introduction,
        narrow_width,
        text_width,
        129.0,
        25.0,
        info_bottom,
        16,
    );
    let mut section_y = 129.0 + intro_lines.len() as f32 * 25.0 + 27.0;
    for section in article.sections {
        let lines = wrap_text_around_infobox(
            font,
            section.body,
            narrow_width,
            text_width,
            section_y + 51.0,
            25.0,
            info_bottom,
            16,
        );
        section_y += 42.0 + lines.len() as f32 * 25.0 + 20.0;
    }
    section_y.max(info_bottom) + 12.0
}

/// Wrap article paragraphs, calculate the required panel height, then draw each section.
fn draw_article(font: &Font, x: f32, y: f32, width: f32, article: &Article<'_>) {
    let pad = 34.0;
    let text_width = width - pad * 2.0;
    let info_width = (width * 0.34).max(220.0).min(width - pad * 2.0);
    let info_x = x + width - pad - info_width;
    let info_y = y + 104.0;
    // The lead paragraph uses the remaining left column beside the infobox.
    let info_height = information_box_height(font, info_width, &article.infobox);
    let info_bottom = info_y + info_height;
    let narrow_width = text_width - info_width - 24.0;
    let intro_lines = wrap_text_around_infobox(
        font,
        article.introduction,
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
    for section in article.sections {
        let lines = wrap_text_around_infobox(
            font,
            section.body,
            narrow_width,
            text_width,
            section_y + 51.0,
            25.0,
            info_bottom,
            16,
        );
        section_lines.push((section.heading, lines));
        section_y += 42.0 + section_lines.last().unwrap().1.len() as f32 * 25.0 + 20.0;
    }
    let panel_height = article_panel_height(font, width, article);
    components::panel(x, y, width, panel_height);
    components::text(font, article.title, x + pad, y + 57.0, 32, INK);
    components::text(font, article.category, x + pad, y + 88.0, 15, BLUE);
    draw_wrapped_lines(font, &intro_lines, x + pad, y + 129.0, 16, 25.0, INK);
    draw_information_box(font, info_x, info_y, info_width, &article.infobox);

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

/// Calculate the infobox height from its supplied title and facts.
fn information_box_height(font: &Font, width: f32, infobox: &ArticleInfobox<'_>) -> f32 {
    let inner_width = (width - 36.0).max(1.0);
    let title_lines = wrap_text(font, infobox.title, inner_width, 18);
    let image_y = 30.0 + title_lines.len() as f32 * 22.0;
    let fact_rows_height: f32 = infobox
        .facts
        .iter()
        .map(|fact| {
            let value_lines = wrap_text(font, fact.value, inner_width, 16);
            (54.0_f32).max(28.0 + value_lines.len() as f32 * 20.0)
        })
        .sum();
    image_y + 158.0 + 26.0 + fact_rows_height + 36.0
}

fn draw_information_box(font: &Font, x: f32, y: f32, width: f32, infobox: &ArticleInfobox<'_>) {
    let inner_width = (width - 36.0).max(1.0);
    let title_lines = wrap_text(font, infobox.title, inner_width, 18);
    let image_y_offset = 30.0 + title_lines.len() as f32 * 22.0;
    let height = information_box_height(font, width, infobox);
    components::panel(x, y, width, height);
    draw_wrapped_lines(font, &title_lines, x + 18.0, y + 34.0, 18, 22.0, INK);

    let image_x = x + 18.0;
    let image_y = y + image_y_offset;
    let image_width = width - 36.0;
    let image_height = 158.0;
    draw_rectangle(image_x, image_y, image_width, image_height, Color::new(0.94, 0.96, 0.94, 1.0));
    draw_rectangle_lines(image_x, image_y, image_width, image_height, 1.0, components::BORDER);
    (infobox.draw_image)(image_x, image_y, image_width, image_height);
    components::text(font, infobox.image_caption, image_x + 10.0, image_y + image_height - 10.0, 12, MUTED);

    let mut row_y = image_y + image_height + 26.0;
    for (index, fact) in infobox.facts.iter().enumerate() {
        let value_lines = wrap_text(font, fact.value, inner_width, 16);
        let row_height = 54.0_f32.max(28.0 + value_lines.len() as f32 * 20.0);
        components::text(font, fact.label, x + 18.0, row_y, 14, MUTED);
        draw_wrapped_lines(font, &value_lines, x + 18.0, row_y + 23.0, 16, 20.0, INK);
        row_y += row_height;
        if index + 1 < infobox.facts.len() {
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
