use macroquad::prelude::*;

#[macroquad::main("My First Game")]

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