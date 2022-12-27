//! 한글 자소 문자를 SDL2로 화면에 출력하는 유틸리티

use hangul_jaso::*;
use image::DynamicImage;
use image::GenericImageView;
use sdl2::gfx::primitives;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render;
use sdl2::render::{Texture, WindowCanvas};
use std::collections::HashMap;

/// 비트 값으로 저장된 (8x16) 일반 아스키 폰트
#[derive(Default)]
pub struct AsciiFonts {
    pub fonts: Vec<Vec<u32>>,
}

/// 비트 값으로 저장된 (16x16) 한글 자소 폰트
#[derive(Default)]
pub struct KoreanFonts {
    pub cho: Vec<Vec<u32>>,
    pub mid: Vec<Vec<u32>>,
    pub jong: Vec<Vec<u32>>,
}

pub enum Fonts {
    Ascii(AsciiFonts),
    Korean(KoreanFonts),
}

/// 아스키 문자를 출력하는 루틴
/// 해당 문자가 텍스쳐로 들어있는 경우 사용한다.
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

/// 한글 글자를 조합해 출력하는 루틴
/// 해당 문자가 텍스쳐에 들어있을 때 사용한다.
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

/// 문자열 출력
/// 텍스쳐에 글자의 비트맵이 있을 때 사용한다.
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

/// 읽어들인 이미지를 토대로 비트 폰트를 만들어 등록한다.
pub fn image2hex(img: &DynamicImage, x: u32, y: u32, w: u32, h: u32) -> Vec<u32> {
    let mut rows = vec![];
    for j in y..(y + h) {
        let mut cell: u32 = 0;
        for i in x..(x + w) {
            let digit: u32 = (w as i32 - i as i32 + x as i32) as u32 - 1;

            let v = if (*img).get_pixel(i, j).0[3] == 0 {
                0
            } else {
                2_u32.pow(digit)
            };
            cell += v;
        }
        rows.push(cell);
    }

    rows
}

/// 한글 자소 비트 폰트를 통해 한글 코드를 출력한다.
pub fn draw_kor_font(
    font: &KoreanFonts,
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    c: &char,
    fg: &dyn primitives::ToColor,
    bg: &dyn primitives::ToColor,
) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(sdl2::pixels::PixelFormatEnum::BGRA8888, 16, 16)
        .unwrap();

    let (jaso, bul) = build_jaso_bul(c);

    let cho_hex = &font.cho[(jaso.cho + bul.cho.unwrap() * 19) as usize];
    let mid_hex = &font.mid[(jaso.mid + bul.mid.unwrap() * 21) as usize];
    let jong_hex = match bul.jong {
        Some(jong) => &font.jong[(jaso.jong + jong * 28) as usize],
        _ => &font.jong[0],
    };
    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_blend_mode(render::BlendMode::Blend);
            texture_canvas.set_draw_color(Color::from(bg.as_rgba()));
            texture_canvas.clear();
            for j in 0..16_i16 {
                let cho = cho_hex[j as usize];
                let mid = mid_hex[j as usize];
                let jong = jong_hex[j as usize];
                for i in 0..16_i16 {
                    let vc = (cho << i) & 0x8000;
                    let vm = (mid << i) & 0x8000;
                    let vj = (jong << i) & 0x8000;

                    if vc > 0 {
                        texture_canvas.pixel(i, j, fg.as_u32()).unwrap();
                    }
                    if vm > 0 {
                        texture_canvas.pixel(i, j, fg.as_u32()).unwrap();
                    }
                    if vj > 0 {
                        texture_canvas.pixel(i, j, fg.as_u32()).unwrap();
                    }

                    if vc + vm + vj == 0 {
                        texture_canvas.pixel(i, j, bg.as_u32()).unwrap();
                    }
                }
            }
        })
        .unwrap();

    texture.set_blend_mode(render::BlendMode::Blend);
    canvas
        .copy_ex(
            &texture,
            Rect::new(0, 0, 16, 16),
            Rect::new(x, y, 16, 16),
            0.0,
            Point::new(0, 0),
            false,
            false,
        )
        .unwrap();
}

/// 영문 비트 폰트를 읽어들여 글자를 출력한다.
pub fn draw_ascii_font(
    font: &AsciiFonts,
    canvas: &mut WindowCanvas,
    x: i32,
    y: i32,
    contents: &char,
    fg: &dyn primitives::ToColor,
    bg: &dyn primitives::ToColor,
) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(sdl2::pixels::PixelFormatEnum::BGRA8888, 8, 16)
        .unwrap();

    canvas
        .with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_blend_mode(render::BlendMode::Blend);
            texture_canvas.set_draw_color(Color::from(bg.as_rgba()));
            texture_canvas.clear();

            for j in 0..16_i16 {
                let row = font.fonts[*contents as usize][j as usize];
                for i in 0..8_i16 {
                    let v = (row << i) & 0x80;
                    if v > 0 {
                        texture_canvas.pixel(i, j, fg.as_u32()).unwrap();
                    } else {
                        texture_canvas.pixel(i, j, bg.as_u32()).unwrap();
                    }
                }
            }
        })
        .unwrap();

    texture.set_blend_mode(render::BlendMode::Blend);
    canvas
        .copy_ex(
            &texture,
            Rect::new(0, 0, 8, 16),
            Rect::new(x, y, 8, 16),
            0.0,
            Point::new(0, 0),
            false,
            false,
        )
        .unwrap();
}
