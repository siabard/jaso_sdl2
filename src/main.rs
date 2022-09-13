use std::collections::HashMap;

use hangul_jaso::*;
use sdl2::image::LoadTexture;
use sdl2::image::*;
use sdl2::render::{Texture, WindowCanvas};

fn build_jaso_bul(t: &dyn ToString) -> (Jaso, Bul) {
    let code = utf8_to_ucs2(t).unwrap();
    let jaso = build_jaso(code).unwrap();
    let bul = build_bul(&jaso);

    (jaso, bul)
}

fn print_ascii(canvas: &mut WindowCanvas, texture: &Texture, x: i32, y: i32, i: usize, c: &char) {
    // 아스키 코드를 기준으로 row, col을 정한다.
    let row = (*c as i32) / 16;
    let col = (*c as i32) % 16;

    canvas
        .copy_ex(
            texture,
            sdl2::rect::Rect::new(col * 8, row * 16, 8, 16),
            sdl2::rect::Rect::new(x + i as i32 * 16, y, 8, 16),
            0.0,
            None,
            false,
            false,
        )
        .unwrap();
}

fn print_hangul(canvas: &mut WindowCanvas, texture: &Texture, x: i32, y: i32, i: usize, c: &char) {
    let (jaso, bul) = build_jaso_bul(c);

    // 초성 벌과 자소
    let cho_rect = sdl2::rect::Rect::new(
        16 * (jaso.cho as i32),
        16 * (bul.cho.unwrap() as i32),
        16,
        16,
    );
    // 중성 벌과 자소
    let mid_rect = sdl2::rect::Rect::new(
        16 * (jaso.mid as i32),
        16 * (bul.mid.unwrap() as i32 + 8),
        16,
        16,
    );
    // 종성 벌과 자소
    let jong_rect = match bul.jong {
        Some(jong) => {
            sdl2::rect::Rect::new(16 * (jaso.jong as i32), 16 * (jong as i32 + 12), 16, 16)
        }
        _ => sdl2::rect::Rect::new(0, 16 * 12, 16, 16),
    };

    canvas
        .copy_ex(
            texture,
            cho_rect,
            sdl2::rect::Rect::new(x + i as i32 * 16, y, 16, 16),
            0.0,
            None,
            false,
            false,
        )
        .unwrap();
    canvas
        .copy_ex(
            texture,
            mid_rect,
            sdl2::rect::Rect::new(x + i as i32 * 16, y, 16, 16),
            0.0,
            None,
            false,
            false,
        )
        .unwrap();
    canvas
        .copy_ex(
            texture,
            jong_rect,
            sdl2::rect::Rect::new(x + i as i32 * 16, y, 16, 16),
            0.0,
            None,
            false,
            false,
        )
        .unwrap();
}

fn print_string(
    canvas: &mut WindowCanvas,
    textures: &HashMap<Languages, &Texture>,
    x: i32,
    y: i32,
    text: &dyn ToString,
) {
    for (i, c) in text.to_string().chars().enumerate() {
        let code = utf8_to_ucs2(&c).unwrap();
        let lang = ucs2_language(code);
        let texture = (*textures).get(&lang).unwrap();
        match lang {
            Languages::Ascii => print_ascii(canvas, texture, x, y, i, &c),
            Languages::Hangul => print_hangul(canvas, texture, x, y, i, &c),
            _ => {}
        }
    }
}

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
        .load_texture("assets/ascii-light.png")
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
            &"다람쥐쳇바퀴돌았다 가나다 01234 ABCD".to_string(),
        );
        canvas.present();
    }
    Ok(())
}
