use image;

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    println!("Creating Tangelo pattern...");

    // 1ブロックのピクセル数
    let cel_width: i32 = 32;

    // 緑の RBG 値
    let green = Color {
        r: 51,
        g: 153,
        b: 102,
    };

    // 黒の RBG 値
    let black = Color {
        r: 0,
        g: 0,
        b: 0,
    };

    // 画像を新規作成
    let mut image = image::RgbImage::new(512, 512);

    // 市松模様の描画
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        if ((x / cel_width as u32) + (y / cel_width as u32)) % 2 == 0 {
            *pixel = image::Rgb([black.r, black.g, black.b]);
        } else {
            *pixel = image::Rgb([green.r, green.g, green.b]);
        }
    }

    // 画像を保存
    image.save("result.png").unwrap();
}
