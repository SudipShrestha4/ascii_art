use image::GenericImageView;

fn get_str_ascii(intent: u8) -> &'static str {
    let index = intent / 32;
    let ascii = [" ", ".", ",", "-", "~", "+", "=", "@"];
    return ascii[index as usize];
}

fn get_image(dir: &str, scale: u32) {
    let img = image::open(dir).unwrap();
    let (width: u32, height: u32) = img.dimensions();
    for y: u32 in 0..height {
        for x: u32 in 0..width {
            if y % (scale * 2) == 0 && x % scale == 0 {
                let pix = img.get_pixel(x, y);
                let mut intent = pix[0] / 3 + pix[1] / 3 + pix[2] / 3;
                if pix[3] == 0 {
                    intent = 0;
                }
                print!("{}", get_str_ascii(intent));
            }
        }
        if y % (scale * 2) == 0 {
            println!("");
        }
    }
}
fn main() {
    get_image(dir: "myth.heic", scale: 4);
}
