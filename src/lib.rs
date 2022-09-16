use hangul_jaso::*;
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;

pub fn build_jaso_bul(t: &dyn ToString) -> (Jaso, Bul) {
    let code = utf8_to_ucs2(t).unwrap();
    let jaso = build_jaso(code).unwrap();
    let bul = build_bul(&jaso);

    (jaso, bul)
}

pub fn print_ascii(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    x: i32,
    y: i32,
    c: &char,
) -> (i32, i32) {
    // 아스키 코드를 기준으로 row, col을 정한다.
    let row = (*c as i32) / 16;
    let col = (*c as i32) % 16;

    canvas
        .copy_ex(
            texture,
            sdl2::rect::Rect::new(col * 8, row * 16, 8, 16),
            sdl2::rect::Rect::new(x, y, 8, 16),
            0.0,
            None,
            false,
            false,
        )
        .unwrap();
    (x + 8, y)
}

pub fn print_hangul(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    x: i32,
    y: i32,
    c: &char,
) -> (i32, i32) {
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
            sdl2::rect::Rect::new(x, y, 16, 16),
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
            sdl2::rect::Rect::new(x, y, 16, 16),
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
            sdl2::rect::Rect::new(x, y, 16, 16),
            0.0,
            None,
            false,
            false,
        )
        .unwrap();
    (x + 16, y)
}

pub fn print_string(
    canvas: &mut WindowCanvas,
    textures: &HashMap<Languages, &Texture>,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    text: &dyn ToString,
) {
    let mut x_target = x;
    let mut y_target = y;
    for c in text.to_string().chars() {
        let code = utf8_to_ucs2(&c).unwrap();
        let lang = ucs2_language(code);
        let texture = (*textures).get(&lang).unwrap();

        if x_target > x + w as i32 {
            x_target = x;
            y_target += 16;
        }

        if y_target > y + h as i32 {
            break;
        }
        (x_target, y_target) = match lang {
            Languages::Ascii => print_ascii(canvas, texture, x_target, y_target, &c),
            Languages::Hangul => print_hangul(canvas, texture, x_target, y_target, &c),
            _ => (x_target, y_target),
        };
    }
}
