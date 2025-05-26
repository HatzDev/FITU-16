#[path = "core/scene.rs"] mod scene;
#[path = "core/data.rs"] mod data;
#[path = "core/draw.rs"] mod draw;
#[path = "core/fitu16.rs"] mod fitu16;
#[path = "core/lua_codec.rs"] mod lua_codec;

use std::{thread, time::Duration};

use fitu16::FITU16;
use macroquad::{miniquad::conf::Platform, prelude::*};

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 720;

fn window_conf() -> Conf {
    Conf {
        window_title: "FITU-16".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        platform: Platform {
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut fitu16 = FITU16::new();

    let target_frame_duration = 1.0 / 30.0; // 30 FPS
    let mut last_frame_time = get_time();
    
    loop {
        let now = get_time();
        let delta = now - last_frame_time;

        if delta < target_frame_duration {
            let sleep_time = target_frame_duration - delta;
            thread::sleep(Duration::from_secs_f64(sleep_time));
        }

        fitu16.update();
        fitu16.update();

        clear_background(BLACK);
        fitu16.draw();

        next_frame().await;

        last_frame_time = get_time();
    }
}