mod rgb;
mod colors;

use image;
use std::process;
use std::io;

fn main() {
    // 1色目を受け取る
    println!("市松模様に利用する1色目を選択（black, white, red, blue, green）");
    let color_1: String = input_string();
    let color_1: Color = match validate(color_1) {
        Result::Ok(color) => color,
        Result::Err(message) => {
            println!("failure: {}", message);
            process::exit(0);
        },
    };

    // 2色目を受け取る
    println!("市松模様に利用する2色目を選択（black, white, red, blue, green）");
    let color_2: String = input_string();
    let color_2: Color = match validate(color_2) {
        Result::Ok(color) => color,
        Result::Err(message) => {
            println!("failure: {}", message);
            process::exit(0);
        },
    };

    // 同じ色では市松模様にならないので弾く
    if color_1 == color_2 {
        println!("failure: 同じ色は選択できません！");
        process::exit(0);
    }

    println!("生成する画像のファイル名（拡張子なし）を入力");
    let file_name: String = input_string();

    println!("Creating a pattern...");

    // 1ブロックのピクセル数
    let cel_width: i32 = 32;

    // 画像を新規作成
    let mut image = image::RgbImage::new(512, 512);

    // 市松模様の描画
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if ((x / cel_width as u32) + (y / cel_width as u32)) % 2 == 0 {
            *pixel = set_color_to_image(&color_1);
        } else {
            *pixel = set_color_to_image(&color_2);
        }
    }

    // 画像を保存
    image.save(format!("{}.png", file_name)).unwrap();
}

fn input_string() -> String {
    let mut color = String::new();
    io::stdin()
        .read_line(&mut color)
        .expect("failure: 行の読み込みに失敗しました");

    color.trim().to_string()
}

#[derive(PartialEq)]
enum Color {
    Black,
    White,
    Red,
    Blue,
    Green,
}

fn validate(color: String) -> Result<Color, &'static str> {
    match &*color {
        "black" => Ok(Color::Black),
        "white" => Ok(Color::White),
        "red" => Ok(Color::Red),
        "blue" => Ok(Color::Blue),
        "green" => Ok(Color::Green),
        _ => Err("failure: 不正な色です"),
    }
}

fn set_color_to_image(color: &Color) -> image::Rgb<u8> {
    match color {
        Color::Black => image::Rgb([colors::black::get().r, colors::black::get().g, colors::black::get().b]),
        Color::White => image::Rgb([colors::white::get().r, colors::white::get().g, colors::white::get().b]),
        Color::Red => image::Rgb([colors::red::get().r, colors::red::get().g, colors::red::get().b]),
        Color::Green => image::Rgb([colors::green::get().r, colors::green::get().g, colors::green::get().b]),
        Color::Blue => image::Rgb([colors::blue::get().r, colors::blue::get().g, colors::blue::get().b]),
    }
}
