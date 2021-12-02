use std::fs;

use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 100;

struct Star {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Star {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Self {
        Star { x, y, vx, vy }
    }

    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stars = fs::read_to_string("res/day10.txt")
        .unwrap()
        .lines()
        .map(|l| {
            // Assume that input will allways have the same width
            let x = l[10..16].trim().parse::<i32>().unwrap();
            let y = l[18..24].trim().parse::<i32>().unwrap();
            let vx = l[36..38].trim().parse::<i32>().unwrap();
            let vy = l[40..42].trim().parse::<i32>().unwrap();

            Star::new(x, y, vx, vy)
        })
        .collect::<Vec<Star>>();

    for t in 0..20000 {
        let ((bbx, bby), (bbw, bbh)) = bounding_box(&stars);

        if bbh < 32f32 {
            let path = format!("plots/day10-{}.png", t);
            let root = BitMapBackend::new(&path, (WIDTH, HEIGHT)).into_drawing_area();

            root.fill(&RGBColor(255, 255, 255))?;

            let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
                0f32..1f32,
                0f32..1f32,
                (0..WIDTH as i32, 0..HEIGHT as i32),
            ));

            for star in &stars {
                let (x, y) = star.position();
                let x: f32 = x as f32;
                let y: f32 = y as f32;

                let (x, y) = ((x - bbx) / bbw, (y - bby) / bbh);

                root.draw(&Circle::new((x, y), 3, ShapeStyle::from(&BLACK).filled()))?;
            }

            root.draw(&EmptyElement::at((0f32, 0f32)))?;
        }

        stars.iter_mut().for_each(|s| s.step());
    }

    println!("To get answers look into \"plots\" directory and check the images.");
    println!("Clear text in one of them is the answer for part 1.");
    println!("Number in the filename is the answer for part 2.");

    Ok(())
}

fn bounding_box(stars: &Vec<Star>) -> ((f32, f32), (f32, f32)) {
    let minx = stars.into_iter().map(|s| s.position().0).min().unwrap() as f32;
    let maxx = stars.into_iter().map(|s| s.position().0).max().unwrap() as f32;
    let miny = stars.into_iter().map(|s| s.position().1).min().unwrap() as f32;
    let maxy = stars.into_iter().map(|s| s.position().1).max().unwrap() as f32;

    ((minx, miny), (maxx - minx, maxy - miny))
}
