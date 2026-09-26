use macroquad::prelude::*;

// fn = 関数ですよ
// (n, m) = 引数がn, m
// -> k = 返り値がk
fn window_conf() -> Conf {
    Conf {
        window_title: "Wikipedia Game".to_owned(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}