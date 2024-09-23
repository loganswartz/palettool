use std::{env, error::Error, fmt::Display, str::FromStr};
use colored::Colorize;
use image::{DynamicImage, GenericImageView, ImageReader, Pixel};
use oklab::{oklab_to_srgb, srgb_to_oklab, Oklab};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let Some(file) = args.get(1) else {
        return Err("Please specify a file.".into());
    };

    if args.len() < 2 {
        return Err("You must specify a range".into());
    };

    let image = ImageReader::open(file)?.decode()?;
    for range in &args[2..] {
        let rect = Rect::from_str(range)?;
        let averaged = get_average_of_rect_in(&image, &rect);

        println!("{}", averaged.to_string().on_truecolor(averaged.r, averaged.g, averaged.b));
    }

    Ok(())
}

fn get_average_of_rect_in(image: &DynamicImage, rect: &Rect) -> oklab::Rgb<u8> {
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

            // pixels.push(oklab);
            // println!("{}", pixel.0.map(|v| v.to_string()).join(","));
            // println!("{},{},{}", oklab.l, oklab.a, oklab.b);
        }
    }

    oklab_to_srgb(Oklab {
        l: l_total / count,
        a: a_total / count,
        b: b_total / count,
    })
}

#[derive(Debug)]
struct Rect {
    start: Point,
    end: Point,
}

impl FromStr for Rect {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once("-").ok_or("Not a valid range.")?;
        let start = Point::from_str(first)?;
        let end = Point::from_str(second)?;

        Ok(Rect {
            start,
            end,
        })
    }
}

impl Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_once(",").ok_or("Not a valid range.")?;

        let Ok(x) = first.trim().parse() else {
            return Err("The x component couldn't be parsed.".to_string());
        };
        let Ok(y) = second.trim().parse() else {
            return Err("The y component couldn't be parsed.".to_string());
        };

        Ok(Point {
            x,
            y,
        })
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}
