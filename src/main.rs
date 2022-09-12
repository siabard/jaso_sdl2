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

fn print_string(canvas: &mut WindowCanvas, texture: &Texture, x: i32, y: i32, text: &dyn ToString) {
    let mut i = 0;
    for c in text.to_string().chars() {
        let (jaso, bul) = build_jaso_bul(&c);

        // 초성 벌과 자소
        let cho_rect =
            sdl2::rect::Rect::new(16 * (jaso.cho as i32), 16 * (bul.cho as i32 - 1), 16, 16);
        // 중성 벌과 자소
        let mid_rect = sdl2::rect::Rect::new(
            16 * (jaso.mid as i32),
            16 * (bul.mid as i32 - 1 + 8),
            16,
            16,
        );
        // 종성 벌과 자소
        let jong_rect = if bul.jong > 0 {
            sdl2::rect::Rect::new(
                16 * (jaso.jong as i32),
                16 * (bul.jong as i32 - 1 + 12),
                16,
                16,
            )
        } else {
            sdl2::rect::Rect::new(0, 16 * 12, 16, 16)
        };

        canvas.copy_ex(
            &texture,
            cho_rect,
            sdl2::rect::Rect::new(x + i as i32 * 16, y, 16, 16),
            0.0,
            None,
            false,
            false,
        );
        canvas.copy_ex(
            &texture,
            mid_rect,
            sdl2::rect::Rect::new(x + i as i32 * 16, y, 16, 16),
            0.0,
            None,
            false,
            false,
        );
        canvas.copy_ex(
            &texture,
            jong_rect,
            sdl2::rect::Rect::new(x + i as i32 * 16, y, 16, 16),
            0.0,
            None,
            false,
            false,
        );
        i += 1;
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
    let mut texuture_creator = canvas.texture_creator();
    let texture = texuture_creator
        .load_texture("assets/hangul-dkby-dinaru-2.png")
        .unwrap();

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
            &texture,
            0,
            0,
            &"다람쥐쳇바퀴돌았다가나다".to_string(),
        );
        canvas.present();
    }
    Ok(())
}
