use colored::Colorize;
use image::{DynamicImage, GenericImageView, ImageError, ImageReader, Pixel};
use oklab::{oklab_to_srgb, srgb_to_oklab, Oklab, Rgb};

use crate::region::Region;

pub fn get_averages_for_regions_in(filename: String, regions: &Vec<Region>) -> Result<(String, Vec<Rgb<u8>>), ImageError> {
    let image = ImageReader::open(&filename)?.decode()?;
    let mut triplets: Vec<Rgb<u8>> = Vec::with_capacity(regions.len());

    for range in regions {
        let averaged = get_average_of_rect_in(&image, range);

        triplets.push(averaged);
    }

    Ok((filename, triplets))
}

pub fn get_average_of_rect_in(image: &DynamicImage, rect: &Region) -> oklab::Rgb<u8> {
    let start = &rect.start;
    let end = &rect.end;

    let x_dim = end.x - start.x + 1;
    let y_dim = end.y - start.y + 1;
    let count = (x_dim * y_dim) as f32;

    // let mut pixels: Vec<Oklab> = Vec::with_capacity(total_count.try_into().unwrap());
    let mut l_total = 0.0;
    let mut a_total = 0.0;
    let mut b_total = 0.0;

    for x in start.x..=end.x {
        for y in start.y..=end.y {
            let pixel = image.get_pixel(x, y).to_rgb();
            let oklab = srgb_to_oklab(oklab::Rgb { r: pixel.0[0], g: pixel.0[1], b: pixel.0[2]});

            l_total += oklab.l;
            a_total += oklab.a;
            b_total += oklab.b;
        }
    }

    oklab_to_srgb(Oklab {
        l: l_total / count,
        a: a_total / count,
        b: b_total / count,
    })
}

pub fn to_hex(rgb: &Rgb<u8>) -> String {
    format!("#{:02x}{:02x}{:02x}", rgb.r, rgb.g, rgb.b).on_truecolor(rgb.r, rgb.g, rgb.b).to_string()
}
