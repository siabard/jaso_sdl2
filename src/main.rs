use sdl2::image::LoadTexture;
use sdl2::image::*;
use sdl2::render::Texture;
use std::collections::HashMap;

use hangul_jaso::*;
use jaso_sdl2::*;

fn main() -> Result<(), String> {
    let context = sdl2::init()?;
    let video_subsystem = context.video()?;
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem
        .window("한글 출력", 800, 600)
        .build()
        .expect("Video System 설정불가");

    let mut canvas = window.into_canvas().build().expect("Canvas Setup 불가");
    let texture_creator = canvas.texture_creator();
    let mut textures: HashMap<Languages, &Texture> = HashMap::new();
    let texture = texture_creator
        .load_texture("assets/hangul-dkby-dinaru-2.png")
        .unwrap();

    let eng_texture = texture_creator
        .load_texture("assets/ascii-plain.png")
        .unwrap();

    textures.insert(Languages::Hangul, &texture);
    textures.insert(Languages::HangulJamo, &texture);
    textures.insert(Languages::Ascii, &eng_texture);

    let mut event_pump = context.event_pump().expect("Event setup 불가");

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => {
                    break 'running;
                }

                _ => {}
            }
        }

        canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        print_string(
            &mut canvas,
            &textures,
            0,
            0,
            220,
            300,
            &"다람쥐쳇바퀴돌았다 가나다 01234 ABCD()-+_!@#$%^&*".to_string(),
        );
        print_string(
            &mut canvas,
            &textures,
            0,
            120,
            150,
            90,
            &"동해물과 백두산이 마르고 닳도록".to_string(),
        );
        canvas.present();
    }
    Ok(())
}
